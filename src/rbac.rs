pub use sea_orm::{
    rbac::{RbacEngine, RbacSnapshot, RbacUserId},
    DatabaseConnection, RestrictedConnection,
};

pub struct UserContext {
    pub user_id: RbacUserId,
}

impl Default for UserContext {
    fn default() -> Self {
        Self {
            user_id: RbacUserId(0),
        }
    }
}

pub trait DatabaseContext {
    fn unrestricted(self) -> Self;

    fn restricted(
        &self,
        user_ctx: Option<&UserContext>,
    ) -> Result<RestrictedConnection, async_graphql::Error>;
}

impl DatabaseContext for DatabaseConnection {
    fn unrestricted(self) -> Self {
        self.replace_rbac(RbacEngine::from_snapshot(
            RbacSnapshot::danger_unrestricted(),
        ));
        self
    }

    fn restricted(
        &self,
        user_ctx: Option<&UserContext>,
    ) -> Result<RestrictedConnection, async_graphql::Error> {
        Ok(self.restricted_for(match user_ctx {
            Some(user_ctx) => user_ctx.user_id,
            None => RbacUserId(0),
        })?)
    }
}
