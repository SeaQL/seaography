use crate::{
    backend::Backend,
    entities::{Access, Permission, ProjectPermissionSummary, permissions_by_account},
};
use async_graphql::{Context, dynamic::ResolverContext};
use chrono::{DateTime, Utc};
use sea_orm::{
    Condition,
    entity::prelude::{
        ActiveModelBehavior, ColumnTrait, DeriveEntityModel, DerivePrimaryKey, DeriveRelatedEntity,
        DeriveRelation, EntityTrait, EnumIter, Expr, PrimaryKeyTrait, Related, RelationDef,
        RelationTrait,
    },
};
use seaography::{
    CustomFields, GuardAction, LifecycleHooksInterface, OperationType, try_downcast_ref,
};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{never_condition, permission_for_operation_type};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, FromRow)]
#[sea_orm(table_name = "accounts")]
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
    #[sea_orm(column_type = "Text")]
    pub email: String,
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

        if !access.has_permission_on_account(self.id, Permission::Read) {
            return Err("unauthorized".into());
        }

        Ok(permissions_by_account(&backend.db, access.account_id())
            .await?
            .into_iter()
            .map(ProjectPermissionSummary::from)
            .collect())
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::project_permissions::Entity")]
    ProjectPermissions,
}

impl Related<super::project_permissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProjectPermissions.def()
    }
}

impl Related<super::projects::Entity> for Entity {
    fn to() -> RelationDef {
        super::project_permissions::Relation::Projects.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::project_permissions::Relation::Accounts.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    // #[sea_orm(entity = "super::project_permissions::Entity")]
    // ProjectPermissions,
    #[sea_orm(entity = "super::projects::Entity")]
    Projects,
}

impl LifecycleHooksInterface for Entity {
    fn entity_guard(
        &self,
        _ctx: &ResolverContext,
        _entity: &str,
        _action: OperationType,
    ) -> GuardAction {
        // TODO(seaography)
        GuardAction::Allow
        // GuardAction::Block(Some("entity_guard not implemented for Account".to_string()))
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

        let Ok(account) = try_downcast_ref::<Model>(ctx.parent_value) else {
            return GuardAction::Block(Some("Cannot downcast to Account".to_string()));
        };

        let permission = permission_for_operation_type(action);

        if access.has_permission_on_account(account.id, permission) {
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
                Some(Condition::all().add(Column::Id.eq(access.account_id())))
            }
        } else {
            // Client is not authenticated
            Some(never_condition())
        }
    }
}
