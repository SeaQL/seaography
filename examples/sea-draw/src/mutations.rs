use crate::{
    backend::Backend,
    entities::{self, Access, Account, Drawing, Object, Permission, Project, ProjectPermission},
    types::{Fill, Shape, Stroke},
};
use async_graphql::Context;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter,
    sea_query::query::OnConflict,
};
use seaography::CustomFields;
use tracing::instrument;
use uuid::Uuid;

pub struct CustomMutations;

#[CustomFields]
impl CustomMutations {
    #[instrument(skip_all)]
    async fn create_account(
        ctx: &Context<'_>,
        name: String,
        email: String,
    ) -> async_graphql::Result<Account> {
        let backend = ctx.data::<Backend>()?;
        let access = ctx
            .data::<Access>()
            .map_err(|_| "unauthenticated".to_string())?;

        // Only root can create new accounts
        if !access.is_root {
            return Err("unauthorized".into());
        }

        let account = Account {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            name,
            email,
        };

        backend.insert(account.clone()).await?;
        tracing::info!("Created account {}", account.id);

        Ok(account)
    }

    #[instrument(skip_all)]
    async fn create_project(ctx: &Context<'_>, name: String) -> async_graphql::Result<Project> {
        let backend = ctx.data::<Backend>()?;
        let access = ctx
            .data::<Access>()
            .map_err(|_| "unauthenticated".to_string())?;

        // Create the project
        let project = Project {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            name,
        };

        // backend.insert(project.clone()).await?;
        entities::projects::Entity::insert(project.clone().into_active_model().reset_all())
            .exec(&backend.db)
            .await?;

        // Give the account that created the project admin permission on it
        let pp = ProjectPermission {
            project_id: project.id,
            account_id: access.account_id(),
            permission: Permission::Admin,
        };
        entities::project_permissions::Entity::insert(pp.into_active_model())
            .exec(&backend.db)
            .await?;

        // Update the Access object so that the permission hooks called by seaography permit
        // the caller to access fields of the result
        access.add_permission(project.id, Permission::Admin);

        tracing::info!("Created project {}", project.id);
        Ok(project)
    }

    /// Grant or revoke permission on a project to a specified account
    ///
    /// If permission is one of read, write, or admin, then the permission will be grated.
    /// If the permission is null, the permission will be revoked. Only accounts with admin
    /// permissions on a project are allowed to call this message, and attempts to revoke or
    /// downgrade your own permission on the project are prohibited.
    #[instrument(skip_all)]
    pub async fn set_project_permission(
        ctx: &Context<'_>,
        project_id: Uuid,
        account_id: Uuid,
        permission: Option<Permission>,
    ) -> async_graphql::Result<bool> {
        let backend = ctx.data::<Backend>()?;
        let access = ctx
            .data::<Access>()
            .map_err(|_| "unauthenticated".to_string())?;
        tracing::info!(
            "set_project_permission: project_id = {}, account_id = {}, permission = {:?}",
            project_id,
            account_id,
            permission,
        );

        if !access.has_permission_on_project(project_id, Permission::Admin) {
            tracing::info!("set_project_permission: unauthenticated");
            return Err("unauthorized".into());
        }

        if account_id == access.account_id() {
            tracing::info!("set_project_permission: self");
            // This is sufficient to prevent projects becoming "orphaned", leaving nobody with
            // admin access, *unless* the request is made by the super account. In the latter case
            // the situation can be repaired by the super account since they can grant admin access
            // to a project to anyone.
            return Err("Cannot change permissions for your own account".into());
        }

        if let Some(permission) = permission {
            let pp = ProjectPermission {
                project_id,
                account_id,
                permission,
            };
            entities::project_permissions::Entity::insert(pp.into_active_model())
                .on_conflict(
                    OnConflict::columns([
                        entities::project_permissions::Column::AccountId,
                        entities::project_permissions::Column::ProjectId,
                    ])
                    .update_column(entities::project_permissions::Column::Permission)
                    .to_owned(),
                )
                .exec(&backend.db)
                .await?;
        } else {
            entities::project_permissions::Entity::delete_many()
                .filter(entities::project_permissions::Column::AccountId.eq(account_id))
                .filter(entities::project_permissions::Column::ProjectId.eq(project_id))
                .exec(&backend.db)
                .await?;
        }

        Ok(true)
    }

    #[instrument(skip_all)]
    async fn create_drawing(
        ctx: &Context<'_>,
        project_id: Uuid,
        name: String,
        width: i64,
        height: i64,
    ) -> async_graphql::Result<Drawing> {
        let backend = ctx.data::<Backend>()?;
        let access = ctx
            .data::<Access>()
            .map_err(|_| "unauthenticated".to_string())?;

        // Only allow creation of drawings in this project if the account has write permission
        if !access.has_permission_on_project(project_id, Permission::Write) {
            return Err("unauthorized".into());
        }

        let drawing = Drawing {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            project_id,
            name,
            width,
            height,
            // tags: vec!["one".to_string(), "two".to_string()],
            // data: json!(["one", "two"]),
        };

        // backend.insert(drawing.clone()).await?;

        entities::drawings::Entity::insert(drawing.clone().into_active_model())
            .exec(&backend.db)
            .await?;

        tracing::info!("Created drawing {}", drawing.id);

        Ok(drawing)
    }

    #[instrument(skip_all)]
    async fn create_object(
        ctx: &Context<'_>,
        drawing_id: Uuid,
        fill: Fill,
        stroke: Stroke,
        shape: Shape,
    ) -> async_graphql::Result<Object> {
        let backend = ctx.data::<Backend>()?;
        let access = ctx
            .data::<Access>()
            .map_err(|_| "unauthenticated".to_string())?;

        let drawing = entities::drawings::Entity::find_by_id(drawing_id)
            .one(&backend.db)
            .await?
            .ok_or("Drawing not found")?;

        // Only allow creation of objects in this drawing if the account has write permission
        // on the project containing the drawing
        if !access.has_permission_on_project(drawing.project_id, Permission::Write) {
            return Err("unauthorized".into());
        }

        let object = Object {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            project_id: drawing.project_id,
            drawing_id,
            fill,
            stroke,
            shape,
        };

        entities::objects::Entity::insert(object.clone().into_active_model())
            .exec(&backend.db)
            .await?;

        tracing::info!("Created object {}", object.id);

        Ok(object)
    }

    #[instrument(skip_all)]
    async fn update_object(
        ctx: &Context<'_>,
        object_id: Uuid,
        fill: Option<Fill>,
        stroke: Option<Stroke>,
        shape: Option<Shape>,
    ) -> async_graphql::Result<Object> {
        let backend = ctx.data::<Backend>()?;
        let access = ctx
            .data::<Access>()
            .map_err(|_| "unauthenticated".to_string())?;

        let mut object = entities::objects::Entity::find_by_id(object_id)
            .one(&backend.db)
            .await?
            .ok_or("Object not found")?;

        // Only allow updates to object if the account has write permission on the project it's in
        if !access.has_permission_on_project(object.project_id, Permission::Write) {
            return Err("unauthorized".into());
        }

        // Update specified fields
        if let Some(fill) = fill {
            object.fill = fill;
        }
        if let Some(stroke) = stroke {
            object.stroke = stroke;
        }
        if let Some(shape) = shape {
            object.shape = shape;
        }
        object.updated_at = Utc::now();

        // Save the object
        entities::objects::Entity::update(object.clone().into_active_model().reset_all())
            .exec(&backend.db)
            .await?;

        Ok(object)
    }

    #[instrument(skip_all)]
    async fn delete_object(ctx: &Context<'_>, object_id: Uuid) -> async_graphql::Result<bool> {
        let backend = ctx.data::<Backend>()?;
        let access = ctx
            .data::<Access>()
            .map_err(|_| "unauthenticated".to_string())?;

        let mut object = entities::objects::Entity::find_by_id(object_id)
            .one(&backend.db)
            .await?
            .ok_or("Object not found")?;

        // Only allow deletion of object if the account has write permission on the project it's in
        if !access.has_permission_on_project(object.project_id, Permission::Write) {
            return Err("unauthorized".into());
        }

        // Implement soft-delete by setting the deleted_at field to a non-NULL value. The object
        // still exists in the database, but clients who wish to query all objects that still
        // exist can do so by supplying a filter indicating that they are only interested in
        // objects that have deleted_at set to NULL.
        object.deleted_at = Some(Utc::now());

        // Save the object
        entities::objects::Entity::update(object.clone().into_active_model().reset_all())
            .exec(&backend.db)
            .await?;

        Ok(true)
    }
}
