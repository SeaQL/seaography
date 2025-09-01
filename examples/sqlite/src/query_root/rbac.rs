use super::*;

use async_graphql::{dynamic::ResolverContext, Result as GqlResult};
use sea_orm::{
    rbac::entity::{
        permission,
        resource::{self, ResourceId},
        role::{self, RoleId},
        role_hierarchy::{self, Model as RoleHierarchy},
    },
    DatabaseConnection,
};
use seaography::{
    macros::{CustomOperation, CustomOutput},
    DatabaseContext, UserContext,
};
use std::collections::HashMap;

#[derive(Clone, CustomOutput)]
#[seaography(prefix = "SeaOrm", suffix = "")]
pub struct UserRolePermissions {
    pub role: role::Model,
    pub permissions: Vec<ResourcePermission>,
}

#[derive(Clone, CustomOutput)]
#[seaography(prefix = "SeaOrm", suffix = "")]
pub struct ResourcePermission {
    pub resource: resource::Model,
    pub permission: permission::Model,
}

#[derive(Clone, CustomOutput)]
#[seaography(prefix = "SeaOrm", suffix = "")]
pub struct RoleAndRank {
    pub role: role::Model,
    pub rank: u32,
}

#[derive(Clone, CustomOutput)]
#[seaography(prefix = "SeaOrm", suffix = "")]
pub struct ResourceAssociatedPermissions {
    pub resource: resource::Model,
    pub permissions: Vec<PermissionGrant>,
}

#[derive(Clone, CustomOutput)]
#[seaography(prefix = "SeaOrm", suffix = "")]
pub struct PermissionGrant {
    pub permission: permission::Model,
    pub grant: bool,
}

#[allow(dead_code)]
#[derive(CustomOperation)]
pub struct Queries {
    _sea_orm_current_user_role_permissions: fn() -> UserRolePermissions,

    _sea_orm_roles_and_ranks: fn() -> Vec<RoleAndRank>,

    _sea_orm_role_hierarchy_edges: fn(role_id: RoleId) -> Vec<RoleHierarchy>,

    _sea_orm_role_permissions_by_resources:
        fn(role_id: RoleId) -> Vec<ResourceAssociatedPermissions>,
}

impl Queries {
    async fn _sea_orm_current_user_role_permissions(
        ctx: &ResolverContext<'_>,
    ) -> GqlResult<UserRolePermissions> {
        let db = &ctx
            .data::<DatabaseConnection>()?
            .restricted(ctx.data_opt::<UserContext>())?;

        let role = db.current_user_role_permissions()?;

        Ok(UserRolePermissions {
            role: role.role,
            permissions: role
                .permissions
                .into_iter()
                .map(|(resource, permission)| ResourcePermission {
                    resource,
                    permission,
                })
                .collect(),
        })
    }

    async fn _sea_orm_roles_and_ranks(ctx: &ResolverContext<'_>) -> GqlResult<Vec<RoleAndRank>> {
        let db = &ctx
            .data::<DatabaseConnection>()?
            .restricted(ctx.data_opt::<UserContext>())?;

        Ok(db
            .roles_and_ranks()?
            .into_iter()
            .map(|(role, rank)| RoleAndRank { role, rank })
            .collect())
    }

    async fn _sea_orm_role_hierarchy_edges(
        ctx: &ResolverContext<'_>,
        role_id: RoleId,
    ) -> GqlResult<Vec<RoleHierarchy>> {
        let db = &ctx
            .data::<DatabaseConnection>()?
            .restricted(ctx.data_opt::<UserContext>())?;

        Ok(db.role_hierarchy_edges(role_id)?)
    }

    async fn _sea_orm_role_permissions_by_resources(
        ctx: &ResolverContext<'_>,
        role_id: RoleId,
    ) -> GqlResult<Vec<ResourceAssociatedPermissions>> {
        let db = &ctx
            .data::<DatabaseConnection>()?
            .restricted(ctx.data_opt::<UserContext>())?;

        let (all_resources, all_permissions) = db.resources_and_permissions()?;

        let all_permissions: Vec<_> = all_permissions
            .into_iter()
            .map(|p| PermissionGrant {
                permission: p,
                grant: false,
            })
            .collect();

        // empty table for all resources where all permissions are not granted
        let mut permissions: HashMap<ResourceId, Vec<PermissionGrant>> = all_resources
            .iter()
            .map(|r| (r.id, all_permissions.clone()))
            .collect();

        let mut all_resources: HashMap<ResourceId, resource::Model> =
            all_resources.into_iter().map(|r| (r.id, r)).collect();

        for (resource, granted_permissions) in db.role_permissions_by_resources(role_id)? {
            let mut associated_permissions = all_permissions.clone();
            for permission in granted_permissions {
                associated_permissions
                    .iter_mut()
                    .find(|p| p.permission.id == permission.id)
                    .expect("Must have Permission")
                    .grant = true;
            }
            permissions.insert(resource.id, associated_permissions);
        }

        let mut result: Vec<_> = permissions
            .into_iter()
            .map(|(r, p)| ResourceAssociatedPermissions {
                resource: all_resources.remove(&r).expect("Must have Resource"),
                permissions: p,
            })
            .collect();

        result.sort_by_key(|r| r.resource.id);

        Ok(result)
    }
}

pub fn register(builder: &mut Builder) {
    seaography::register_custom_outputs!(
        builder,
        [
            PermissionGrant,
            ResourceAssociatedPermissions,
            ResourcePermission,
            RoleAndRank,
            UserRolePermissions,
        ]
    );

    seaography::register_custom_entities!(builder, [role, role_hierarchy, resource, permission]);

    seaography::register_custom_queries!(builder, [Queries]);
}
