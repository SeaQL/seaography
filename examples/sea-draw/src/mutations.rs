use sea_orm::{ModelTrait, EntityTrait};
use async_graphql::Context;
use seaography::CustomFields;
use uuid::Uuid;
use crate::{
    entities::{Account, Project, Drawing, Object},
    types::{Fill, Stroke, Shape},
    backend::Backend,
};

pub struct CustomMutations;

#[CustomFields]
impl CustomMutations {
    async fn create_account(
        ctx: &Context<'_>,
        name: String,
        email: String,
    ) -> async_graphql::Result<Account> {
        let backend = ctx.data::<Backend>()?;
        let account = backend.create_account(name, email).await?;
        Ok(account)
    }

    async fn create_project(
        ctx: &Context<'_>,
        name: String,
    ) -> async_graphql::Result<Project> {
        let backend = ctx.data::<Backend>()?;
        let project = backend.create_project(name).await?;
        Ok(project)
    }

    async fn create_drawing(
        ctx: &Context<'_>,
        project_id: Uuid,
        name: String,
        width: i64,
        height: i64,
    ) -> async_graphql::Result<Drawing> {
        let backend = ctx.data::<Backend>()?;
        let drawing = backend.create_drawing(project_id, name, width, height).await?;
        Ok(drawing)
    }

    async fn create_object(
        ctx: &Context<'_>,
        drawing_id: Uuid,
        fill: Fill,
        stroke: Stroke,
        shape: Shape,
    ) -> async_graphql::Result<Object> {
        let backend = ctx.data::<Backend>()?;

        let drawing = <Drawing as ModelTrait>::Entity::find_by_id(drawing_id).one(&backend.db).await?
            .ok_or("Drawing not found")?;

        let object = backend.create_object(
            drawing.project_id,
            drawing_id,
            fill,
            stroke,
            shape,
        ).await?;
        Ok(object)
    }
}
