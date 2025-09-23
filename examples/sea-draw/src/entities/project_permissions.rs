use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, Order, QueryFilter, QueryOrder,
    RelationTrait,
    entity::prelude::{
        ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelatedEntity,
        DeriveRelation, EnumIter, PrimaryKeyTrait, Related, RelationDef,
    },
};
use seaography::{CustomFields, CustomOutputType};
use serde::Serialize;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

use crate::{
    backend::Backend,
    entities::{Account, Object, Permission, Project, ProjectPermission},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "project_permissions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub project_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub account_id: Uuid,
    pub permission: Permission,
}

// For security reasons, we only expose a simple object containing a few basic fields
// relating to the account. This avoids accidentally exposing sensitive information about
// the account and the possibility of clients requesting other fields of the Account object
// in the graphql query to gain access to information they aren't supposed to see.
//
// Specifically, just because you have access to see the list of permissions for project,
// you shouldn't be able to view all information about the accounts that have access.
#[derive(Clone, Debug, CustomOutputType)]
pub struct ProjectPermissionSummary {
    pub project_id: Uuid,
    pub account_id: Uuid,
    pub permission: Permission,
}

impl From<ProjectPermission> for ProjectPermissionSummary {
    fn from(input: ProjectPermission) -> Self {
        Self {
            account_id: input.account_id,
            project_id: input.project_id,
            permission: input.permission,
        }
    }
}

#[derive(Clone, Debug, CustomOutputType)]
pub struct PermissionAccount {
    pub id: Uuid,
    pub name: String,
}

#[CustomFields]
impl ProjectPermissionSummary {
    // For security reasons, we only expose a simple object containing a few basic fields
    // relating to the account. This avoids accidentally exposing sensitive information about
    // the account and the possibility of clients requesting other fields of the Account object
    // in the graphql query to gain access to information they aren't supposed to see.
    pub async fn account(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<PermissionAccount> {
        let backend = ctx.data::<Backend>()?;
        let account = crate::entities::accounts::Entity::find_by_id(self.account_id)
            .one(&backend.db)
            .await?
            .ok_or("account not found")?;

        Ok(PermissionAccount {
            id: account.id,
            name: account.name,
        })
    }

    // No need to hide information here - if the user can see the permissions for a project,
    // they at least have read access to the project itself.
    pub async fn project(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<Project> {
        let backend = ctx.data::<Backend>()?;
        let project = crate::entities::projects::Entity::find_by_id(self.project_id)
            .one(&backend.db)
            .await?
            .ok_or("project not found")?;
        Ok(project)
    }
}

pub async fn permissions_by_project(
    db: &DatabaseConnection,
    project_id: Uuid,
) -> Result<Vec<ProjectPermission>, DbErr> {
    Entity::find()
        .filter(Column::ProjectId.eq(project_id))
        .order_by(Column::AccountId, Order::Asc)
        .all(db)
        .await
}

pub async fn permissions_by_account(
    db: &DatabaseConnection,
    account_id: Uuid,
) -> Result<Vec<ProjectPermission>, DbErr> {
    Entity::find()
        .filter(Column::AccountId.eq(account_id))
        .order_by(Column::ProjectId, Order::Asc)
        .all(db)
        .await
}

#[derive(Clone, Debug)]
pub struct Access {
    pub account: Account,
    permissions: Arc<Mutex<BTreeMap<Uuid, Permission>>>,
    pub is_root: bool,
}

impl Access {
    pub fn new(account: Account, permissions: BTreeMap<Uuid, Permission>, is_root: bool) -> Self {
        Self {
            account,
            permissions: Arc::new(Mutex::new(permissions)),
            is_root,
        }
    }

    pub fn permissions(&self) -> BTreeMap<Uuid, Permission> {
        self.permissions.lock().unwrap().clone()
    }

    pub fn add_permission(&self, project_id: Uuid, permission: Permission) {
        self.permissions
            .lock()
            .unwrap()
            .insert(project_id, permission);
    }

    pub fn has_permission_on_project(&self, project_id: Uuid, permission: Permission) -> bool {
        // Root is always allowed to do anything
        if self.is_root {
            return true;
        }

        // Anything in a public project can be read by anyone
        if permission == Permission::Read && project_id == Uuid::nil() {
            return true;
        }

        match self.permissions.lock().unwrap().get(&project_id) {
            Some(perms) => perms.includes(permission),
            None => false,
        }
    }

    pub fn has_permission_on_account(&self, account_id: Uuid, _permission: Permission) -> bool {
        // Root is always allowed to do anything
        if self.is_root {
            return true;
        }

        self.account.id == account_id
    }

    pub fn has_permission_on_object(&self, object: &Object, permission: Permission) -> bool {
        self.has_permission_on_project(object.project_id, permission)
    }

    pub fn account_id(&self) -> Uuid {
        self.account.id
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::accounts::Entity",
        from = "Column::AccountId",
        to = "super::accounts::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Accounts,
    #[sea_orm(
        belongs_to = "super::projects::Entity",
        from = "Column::ProjectId",
        to = "super::projects::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Projects,
}

impl Related<super::accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Accounts.def()
    }
}

impl Related<super::projects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Projects.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::accounts::Entity")]
    Accounts,
    #[sea_orm(entity = "super::projects::Entity")]
    Projects,
}
