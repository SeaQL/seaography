use async_graphql::{Context, dynamic::ResolverContext};
use chrono::{DateTime, Utc};
use sea_orm::{Condition, entity::prelude::*};
use seaography::{
    CustomFields, GuardAction, LifecycleHooksInterface, OperationType, try_downcast_ref,
};

use crate::{
    backend::Backend,
    entities::{Access, Permission, ProjectPermissionSummary, permissions_by_project},
    never_condition, permission_for_operation_type,
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTime<Utc>,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    #[sea_orm(column_type = "Text")]
    pub name: String,
}

#[CustomFields]
impl Model {
    async fn permissions(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<ProjectPermissionSummary>> {
        let backend = ctx.data::<Backend>()?;
        let access = ctx
            .data::<Access>()
            .map_err(|_| "unauthenticated".to_string())?;

        if !access.has_permission_on_project(self.id, Permission::Admin) {
            return Err("unauthorized".into());
        }

        Ok(permissions_by_project(&backend.db, self.id)
            .await?
            .into_iter()
            .map(ProjectPermissionSummary::from)
            .collect())
    }

    async fn permission(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<Permission>> {
        let access = ctx
            .data::<Access>()
            .map_err(|_| "unauthenticated".to_string())?;
        Ok(access.permissions().get(&self.id).copied())
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::drawings::Entity")]
    Drawings,
    #[sea_orm(has_many = "super::objects::Entity")]
    Objects,
    #[sea_orm(has_many = "super::project_permissions::Entity")]
    ProjectPermissions,
}

impl Related<super::drawings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Drawings.def()
    }
}

impl Related<super::objects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Objects.def()
    }
}

impl Related<super::project_permissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProjectPermissions.def()
    }
}

impl Related<super::accounts::Entity> for Entity {
    fn to() -> RelationDef {
        super::project_permissions::Relation::Accounts.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::project_permissions::Relation::Projects.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::drawings::Entity")]
    Drawings,
    #[sea_orm(entity = "super::objects::Entity")]
    Objects,
    // #[sea_orm(entity = "super::project_permissions::Entity")]
    // ProjectPermissions,
    #[sea_orm(entity = "super::accounts::Entity")]
    Accounts,
}

impl LifecycleHooksInterface for Entity {
    fn entity_guard(
        &self,
        _ctx: &ResolverContext,
        _entity: &str,
        _action: OperationType,
    ) -> GuardAction {
        GuardAction::Allow
    }

    fn field_guard(
        &self,
        ctx: &ResolverContext,
        _entity: &str,
        _field: &str,
        action: OperationType,
    ) -> GuardAction {
        let access = if let Ok(access) = ctx.data::<Access>() {
            access
        } else {
            return GuardAction::Block(Some("unauthenticated".to_string()));
        };

        let Ok(project) = try_downcast_ref::<Model>(ctx.parent_value) else {
            return GuardAction::Block(Some("Cannot downcast to Project".to_string()));
        };

        let permission = permission_for_operation_type(action);

        if access.has_permission_on_project(project.id, permission) {
            GuardAction::Allow
        } else {
            GuardAction::Block(Some("unauthorized".to_string()))
        }
    }

    fn entity_filter(
        &self,
        ctx: &ResolverContext,
        _entity: &str,
        _action: OperationType,
    ) -> Option<Condition> {
        if let Ok(access) = ctx.data::<Access>() {
            if access.is_root {
                None
            } else {
                let public_project_id = Uuid::nil();
                Some(
                    Condition::all().add(
                        Column::Id.is_in(
                            access
                                .permissions()
                                .keys()
                                .cloned()
                                .chain([public_project_id])
                                .collect::<Vec<Uuid>>(),
                        ),
                    ),
                )
            }
        } else {
            // Client is not authenticated
            Some(never_condition())
        }
    }
}
