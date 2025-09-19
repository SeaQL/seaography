use reqwest::{
    RequestBuilder, StatusCode,
    multipart::{Form, Part},
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::borrow::Borrow;
use uuid::{Uuid, uuid};

pub async fn graphql(
    url: &str,
    token: impl Borrow<Uuid>,
    query: impl Into<String>,
    variables: Option<Value>,
) -> Result<Value, GraphQLError> {
    let query = query.into();
    let mut map = Map::new();
    tracing::debug!("GraphQL query {}", query);
    map.insert("query".to_string(), Value::String(query));
    if let Some(variables) = variables {
        tracing::debug!("GraphQL variables: {:#}", variables);
        map.insert("variables".to_string(), variables);
    }

    let mut client = reqwest::Client::new().post(url);
    let token: &Uuid = token.borrow();
    if *token != unauthenticated() {
        client = client.header(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token))?,
        );
    }
    client = client.json(&Value::Object(map));

    send_graphql_request(client).await
}

pub struct Upload {
    pub var: String,
    pub content: Vec<u8>,
    pub filename: String,
}

pub async fn graphql_upload(
    url: &str,
    token: impl Borrow<Uuid>,
    query: impl Into<String>,
    variables: Option<Value>,
    uploads: Vec<Upload>,
) -> Result<Value, GraphQLError> {
    let query = query.into();
    let mut map = Map::new();
    tracing::debug!("GraphQL query {}", query);
    map.insert("query".to_string(), Value::String(query));

    let mut variables_map: Map<String, Value> = match variables {
        Some(Value::Object(map)) => map,
        Some(_) => panic!("variables is not an object"),
        None => Map::new(),
    };

    let mut part_map: Map<String, Value> = Map::new();
    for (i, upload) in uploads.iter().enumerate() {
        variables_map.insert(upload.var.clone(), Value::Null);
        part_map.insert(
            format!("{}", i),
            Value::String(format!("variables.{}", upload.var)),
        );
    }

    let variables = Some(Value::Object(variables_map));
    if let Some(variables) = variables {
        tracing::debug!("GraphQL variables: {:#}", variables);
        map.insert("variables".to_string(), variables);
    }

    let part1_text = serde_json::to_string_pretty(&Value::Object(map)).unwrap();
    let part2_text =
        serde_json::to_string_pretty(&serde_json::json!({ "0": ["variables.file"] })).unwrap();

    let form = Form::new();
    let form = form.part("operations", Part::text(part1_text));
    let mut form = form.part("map", Part::text(part2_text));
    for (i, upload) in uploads.iter().enumerate() {
        form = form.part(
            format!("{}", i),
            Part::bytes(upload.content.clone()).file_name("x"),
        );
    }

    let client = reqwest::Client::new()
        .post(url)
        .header(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token.borrow()))?,
        )
        .multipart(form);

    send_graphql_request(client).await
}

async fn send_graphql_request(client: RequestBuilder) -> Result<Value, GraphQLError> {
    let response = client.send().await?;

    if response.status().as_u16() < 200 || response.status().as_u16() > 299 {
        return Err(GraphQLError::Http(response.status()));
    }

    let json_response: Value = response.json().await?;
    tracing::debug!("GraphQL response: {:#}", json_response);
    let graphql_response: GraphQLResponse = serde_json::from_value(json_response)?;

    if let Some(errors) = graphql_response.errors {
        if errors.len() == 1 && errors[0].message == "unauthenticated" {
            Err(GraphQLError::Unauthenticated)
        } else if errors.len() == 1 && errors[0].message == "unauthorized" {
            Err(GraphQLError::Unauthorized)
        } else {
            Err(GraphQLError::GraphQL(errors))
        }
    } else {
        Ok(graphql_response.data)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GraphQLResponse {
    pub data: Value,
    pub extensions: Option<Value>,
    pub errors: Option<Vec<GraphQLErrorDetail>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GraphQLErrorLocation {
    pub line: u32,
    pub column: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GraphQLErrorDetail {
    pub message: String,
    pub locations: Option<Vec<GraphQLErrorLocation>>,
}

#[derive(Debug)]
pub enum GraphQLError {
    Reqwest(reqwest::Error),
    Http(StatusCode),
    ObjectUploadPut(StatusCode),
    String(String),
    Json(serde_json::Error),
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
    Unauthenticated,
    Unauthorized,
    GraphQL(Vec<GraphQLErrorDetail>),
}

impl From<reqwest::Error> for GraphQLError {
    fn from(v: reqwest::Error) -> Self {
        GraphQLError::Reqwest(v)
    }
}

impl From<String> for GraphQLError {
    fn from(v: String) -> Self {
        GraphQLError::String(v)
    }
}

impl From<serde_json::Error> for GraphQLError {
    fn from(v: serde_json::Error) -> Self {
        GraphQLError::Json(v)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for GraphQLError {
    fn from(v: reqwest::header::InvalidHeaderValue) -> Self {
        GraphQLError::InvalidHeaderValue(v)
    }
}

impl std::fmt::Display for GraphQLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            GraphQLError::GraphQL(errors) if !errors.is_empty() => {
                for (i, error) in errors.iter().enumerate() {
                    if i > 0 {
                        writeln!(f)?;
                    }
                    write!(f, "{}", error.message)?;
                }
                Ok(())
            }
            GraphQLError::Http(status) => {
                write!(f, "Server returned HTTP status {}", status)
            }
            GraphQLError::ObjectUploadPut(status) => {
                write!(
                    f,
                    "Google object storage PUT returned HTTP status {}",
                    status
                )
            }
            _ => std::fmt::Debug::fmt(self, f),
        }
    }
}

impl std::error::Error for GraphQLError {}

pub fn unauthenticated() -> Uuid {
    uuid!("ffffffff-ffff-ffff-ffff-ffffffffffff")
}
