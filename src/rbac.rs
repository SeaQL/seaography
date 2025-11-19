use sea_orm::{DatabaseConnection, DbErr, StatementBuilder};

#[derive(Default)]
pub struct UserContext {
    pub user_id: i64,
}

pub trait DatabaseContext {
    type Connection;

    fn unrestricted(self) -> Self;

    fn restricted(&self, user_ctx: Option<&UserContext>) -> Result<Self::Connection, DbErr>;

    fn user_can_run<S: StatementBuilder>(&self, stmt: &S) -> Result<(), DbErr>;
}

#[cfg(feature = "rbac")]
impl DatabaseContext for DatabaseConnection {
    type Connection = sea_orm::RestrictedConnection;

    fn unrestricted(self) -> Self {
        use sea_orm::rbac::{RbacEngine, RbacSnapshot};

        self.replace_rbac(RbacEngine::from_snapshot(
            RbacSnapshot::danger_unrestricted(),
        ));
        self
    }

    fn restricted(
        &self,
        user_ctx: Option<&UserContext>,
    ) -> Result<sea_orm::RestrictedConnection, DbErr> {
        use sea_orm::rbac::RbacUserId;

        self.restricted_for(match user_ctx {
            Some(user_ctx) => RbacUserId(user_ctx.user_id),
            None => RbacUserId(0),
        })
    }

    fn user_can_run<S: StatementBuilder>(&self, _: &S) -> Result<(), DbErr> {
        Err(DbErr::RbacError(format!(
            "feature `rbac` is enabled, can only query through RestrictedConnection"
        )))
    }
}

#[cfg(not(feature = "rbac"))]
impl DatabaseContext for DatabaseConnection {
    type Connection = DatabaseConnection;

    fn unrestricted(self) -> Self {
        self
    }

    fn restricted(&self, _: Option<&UserContext>) -> Result<DatabaseConnection, DbErr> {
        Ok(self.clone())
    }

    fn user_can_run<S: StatementBuilder>(&self, _: &S) -> Result<(), DbErr> {
        Ok(())
    }
}
