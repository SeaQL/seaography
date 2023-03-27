#[derive(Debug, seaography::macros::QueryRoot)]
#[seaography(entity = "crate::entities::actor")]
#[seaography(entity = "crate::entities::address")]
#[seaography(entity = "crate::entities::category")]
#[seaography(entity = "crate::entities::city")]
#[seaography(entity = "crate::entities::country")]
#[seaography(entity = "crate::entities::customer")]
#[seaography(entity = "crate::entities::film")]
#[seaography(entity = "crate::entities::film_actor")]
#[seaography(entity = "crate::entities::film_category")]
#[seaography(entity = "crate::entities::film_text")]
#[seaography(entity = "crate::entities::inventory")]
#[seaography(entity = "crate::entities::language")]
#[seaography(entity = "crate::entities::payment")]
#[seaography(entity = "crate::entities::rental")]
#[seaography(entity = "crate::entities::staff")]
#[seaography(
    entity = "crate::entities::store",
    object_config = "guard = \"RoleGuard::new(Role::Admin)\""
)]
pub struct QueryRoot;

#[derive(Eq, PartialEq, Copy, Clone)]
enum Role {
    Admin,
    Guest,
}

struct RoleGuard {
    role: Role,
}

impl RoleGuard {
    fn new(role: Role) -> Self {
        Self { role }
    }
}

#[async_trait::async_trait]
impl async_graphql::Guard for RoleGuard {
    async fn check(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<()> {
        if ctx.data_opt::<Role>() == Some(&self.role) {
            Ok(())
        } else {
            Err("Forbidden".into())
        }
    }
}
