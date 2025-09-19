use super::graphql::{graphql, GraphQLError};
use serde_json::{Value, json};
use uuid::Uuid;

pub struct Client {
    pub url: String,
    pub token: Uuid,
}

impl Client {
    pub fn new(url: String, token: Uuid) -> Self {
        Self { url, token }
    }

    pub async fn create_project(&self, name: impl Into<String>) -> Result<Uuid, GraphQLError> {
        let result = graphql(
            &self.url,
            self.token,
            r#"
                mutation($name: String!) {
                    create_project(name: $name) {
                        id
                    }
                }
            "#,
            Some(json!({ "name": name.into() })),
        ).await?;
        Ok(serde_json::from_value::<Uuid>(result["create_project"]["id"].clone())?)
    }

    pub async fn create_drawing(
        &self,
        project_id: Uuid,
        name: impl Into<String>,
        width: i64,
        height: i64,
    ) -> Result<Uuid, GraphQLError> {
        let result = graphql(
            &self.url,
            self.token,
            r#"
                mutation($project_id: String!, $name: String!, $width: Int!, $height: Int!) {
                    create_drawing(
                        project_id: $project_id,
                        name: $name,
                        width: $width,
                        height: $height,
                    ) {
                        id
                    }
                }
            "#,
            Some(json!({
                "project_id": project_id,
                "name": name.into(),
                "width": width,
                "height": height,
            }))
        ).await?;
        Ok(serde_json::from_value::<Uuid>(result["create_drawing"]["id"].clone())?)
    }

    pub async fn create_object(
        &self,
        drawing_id: Uuid,
        fill: Value,
        stroke: Value,
        shape: Value,
    ) -> Result<Uuid, GraphQLError> {
        let result = graphql(
            &self.url,
            self.token,
            r#"
                mutation(
                    $drawing_id: String!
                    $fill: FillInput!
                    $stroke: StrokeInput!
                    $shape: ShapeInput!
                ) {

                    create_object(
                        drawing_id: $drawing_id,
                        fill: $fill,
                        stroke: $stroke,
                        shape: $shape,
                    ) {
                        id
                    }
                }
            "#,
            Some(json!({
                "drawing_id": drawing_id,
                "fill": fill,
                "stroke": stroke,
                "shape": shape,
            }))
        ).await?;
        Ok(serde_json::from_value::<Uuid>(result["create_object"]["id"].clone())?)
    }
}
