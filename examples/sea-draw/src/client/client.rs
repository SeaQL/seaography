use super::graphql::{GraphQLError, graphql};
use serde_json::{Value, json};
use uuid::Uuid;

pub struct Client {
    pub url: String,
    pub token: Uuid,
}

impl Client {
    pub fn new(url: impl Into<String>, token: Uuid) -> Self {
        Self {
            url: url.into(),
            token,
        }
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
        )
        .await?;
        Ok(serde_json::from_value::<Uuid>(
            result["create_project"]["id"].clone(),
        )?)
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
            })),
        )
        .await?;
        Ok(serde_json::from_value::<Uuid>(
            result["create_drawing"]["id"].clone(),
        )?)
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
            })),
        )
        .await?;
        Ok(serde_json::from_value::<Uuid>(
            result["create_object"]["id"].clone(),
        )?)
    }

    pub async fn update_object(
        &self,
        object_id: Uuid,
        fill: Option<serde_json::Value>,
        stroke: Option<serde_json::Value>,
        shape: Option<serde_json::Value>,
    ) -> Result<(), GraphQLError> {
        graphql(
            &self.url,
            self.token,
            r#"
                mutation(
                    $object_id: String!
                    $fill: FillInput
                    $stroke: StrokeInput
                    $shape: ShapeInput
                ) {
                    update_object(
                        object_id: $object_id
                        fill: $fill
                        stroke: $stroke
                        shape: $shape
                    ) {
                        id
                    }
                }
            "#,
            Some(json!({
                "object_id": object_id,
                "fill": fill,
                "stroke": stroke,
                "shape": shape,
            })),
        )
        .await?;
        Ok(())
    }

    pub async fn delete_object(&self, object_id: Uuid) -> Result<(), GraphQLError> {
        graphql(
            &self.url,
            self.token,
            r#"
                mutation($id: String!) {
                    delete_object(object_id: $id)
                }
            "#,
            Some(json!({ "id": object_id })),
        )
        .await?;
        Ok(())
    }

    pub async fn drawing_svg(&self, drawing_id: Uuid) -> Result<String, GraphQLError> {
        let result = graphql(
            &self.url,
            self.token,
            r#"
                query($id: String!) {
                    drawing(id: $id) {
                        svg
                    }
                }
            "#,
            Some(json!({ "id": drawing_id })),
        )
        .await?;
        Ok(result["drawing"]["svg"].as_str().unwrap().to_string())
    }

    pub async fn set_project_permission(
        &self,
        project_id: Uuid,
        account_id: Uuid,
        permission: Option<&str>,
    ) -> async_graphql::Result<()> {
        graphql(
            &self.url,
            self.token,
            r#"
                mutation(
                    $project_id: String!,
                    $account_id: String!,
                    $permission: PermissionEnum
                ) {
                    set_project_permission(
                        project_id: $project_id,
                        account_id: $account_id,
                        permission: $permission
                    )
                }
            "#,
            Some(json!({
                "project_id": project_id,
                "account_id": account_id,
                "permission": permission,
            })),
        )
        .await?;
        Ok(())
    }

    pub async fn list_projects(&self) -> async_graphql::Result<Value> {
        let mut result = graphql(
            &self.url,
            self.token,
            r#"
                {
                    projects {
                        nodes {
                            id
                            name
                            permission
                        }
                    }
                }
            "#,
            None,
        )
        .await?;

        result["projects"]["nodes"]
            .as_array_mut()
            .unwrap()
            .sort_by(|a, b| a["name"].as_str().unwrap().cmp(b["name"].as_str().unwrap()));

        Ok(result["projects"]["nodes"].clone())
    }
}
