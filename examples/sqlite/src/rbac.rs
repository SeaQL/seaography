use crate::entities::*;
use sea_orm::{
    error::*,
    rbac::{
        entity::{
            permission::{ActiveModel as Permission, PermissionId},
            resource::{ActiveModel as Resource, ResourceId},
            role::{ActiveModel as Role, RoleId},
            role_hierarchy::ActiveModel as RoleHierarchy,
            role_permission::ActiveModel as RolePermission,
            user::UserId,
            user_override::ActiveModel as UserOverride,
            user_role::ActiveModel as UserRole,
        },
        schema::{action_str, create_tables},
        AccessType, RbacUserId,
    },
    ActiveModelTrait, ConnectionTrait, DbConn, EntityName, EntityTrait, ExecResult, Schema, Set,
};
use std::collections::HashMap;

pub async fn setup(db: &DbConn) -> Result<(), DbErr> {
    if create_tables(db).await.is_err() {
        return Ok(());
    }

    let mut resources = HashMap::new();
    let mut permissions = HashMap::new();
    let mut roles = HashMap::new();

    let tables = [
        actor::Entity.table_name(),
        address::Entity.table_name(),
        category::Entity.table_name(),
        city::Entity.table_name(),
        country::Entity.table_name(),
        customer::Entity.table_name(),
        film::Entity.table_name(),
        film_actor::Entity.table_name(),
        film_category::Entity.table_name(),
        film_text::Entity.table_name(),
        inventory::Entity.table_name(),
        language::Entity.table_name(),
        payment::Entity.table_name(),
        rental::Entity.table_name(),
        staff::Entity.table_name(),
        store::Entity.table_name(),
        "*", // WILDCARD
    ];

    for table_name in tables {
        resources.insert(
            table_name,
            Resource {
                table: Set(table_name.to_owned()),
                ..Default::default()
            }
            .insert(db)
            .await?
            .id,
        );
    }

    for action in [
        AccessType::Select,
        AccessType::Insert,
        AccessType::Update,
        AccessType::Delete,
    ] {
        permissions.insert(
            action_str(&action),
            Permission {
                action: Set(action_str(&action).to_owned()),
                ..Default::default()
            }
            .insert(db)
            .await?
            .id,
        );
    }

    for role in ["admin", "manager", "public"] {
        let role_id = Role {
            role: Set(role.to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?
        .id;
        roles.insert(role, role_id);

        UserRole {
            user_id: Set(RbacUserId(role_id.0)),
            role_id: Set(role_id),
        }
        .insert(db)
        .await?;
    }

    // public can only select public film data
    let public_tables = [
        actor::Entity.table_name(),
        category::Entity.table_name(),
        film::Entity.table_name(),
        film_actor::Entity.table_name(),
        film_category::Entity.table_name(),
        film_text::Entity.table_name(),
        language::Entity.table_name(),
    ];
    for table_name in public_tables {
        RolePermission {
            role_id: Set(*roles.get("public").unwrap()),
            permission_id: Set(*permissions.get("select").unwrap()),
            resource_id: Set(*resources.get(table_name).unwrap()),
        }
        .insert(db)
        .await?;
    }

    // manager can select everything
    RolePermission {
        role_id: Set(*roles.get("manager").unwrap()),
        permission_id: Set(*permissions.get("select").unwrap()),
        resource_id: Set(*resources.get("*").unwrap()),
    }
    .insert(db)
    .await?;

    // manager can create / update everything except city
    for (name, resource) in resources.iter() {
        if matches!(*name, "city" | "*") {
            continue;
        }
        for action in ["insert", "update"] {
            RolePermission {
                role_id: Set(*roles.get("manager").unwrap()),
                permission_id: Set(*permissions.get(action).unwrap()),
                resource_id: Set(*resource),
            }
            .insert(db)
            .await?;
        }
    }

    // manager can delete film
    for resource in ["film"] {
        RolePermission {
            role_id: Set(*roles.get("manager").unwrap()),
            permission_id: Set(*permissions.get("delete").unwrap()),
            resource_id: Set(*resources.get(resource).unwrap()),
        }
        .insert(db)
        .await?;
    }

    // admin can do anything, in addition to public / manager
    RolePermission {
        role_id: Set(*roles.get("admin").unwrap()),
        permission_id: Set(*permissions.get("delete").unwrap()),
        resource_id: Set(*resources.get("*").unwrap()),
    }
    .insert(db)
    .await?;

    // add permissions to city which manager doesn't have
    for action in ["insert", "update"] {
        RolePermission {
            role_id: Set(*roles.get("admin").unwrap()),
            permission_id: Set(*permissions.get(action).unwrap()),
            resource_id: Set(*resources.get("city").unwrap()),
        }
        .insert(db)
        .await?;
    }

    RoleHierarchy {
        role_id: Set(*roles.get("public").unwrap()),
        super_role_id: Set(*roles.get("manager").unwrap()),
    }
    .insert(db)
    .await?;
    RoleHierarchy {
        role_id: Set(*roles.get("manager").unwrap()),
        super_role_id: Set(*roles.get("admin").unwrap()),
    }
    .insert(db)
    .await?;

    Ok(())
}
