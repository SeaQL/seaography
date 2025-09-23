use entities::Permission;
use seaography::OperationType;
use uuid::Uuid;

pub mod backend;
pub mod client;
pub mod client_test;
pub mod entities;
pub mod mutations;
pub mod queries;
pub mod schema;
pub mod server;
pub mod subscriptions;
pub mod types;

pub fn never_condition() -> sea_orm::Condition {
    sea_orm::query::Condition::any().add(sea_orm::sea_query::ConditionExpression::SimpleExpr(
        sea_orm::sea_query::SimpleExpr::Constant(sea_orm::sea_query::Value::Bool(Some(false))),
    ))
}

pub fn permission_for_operation_type(op: seaography::OperationType) -> entities::Permission {
    match op {
        OperationType::Read => Permission::Read,
        OperationType::Create => Permission::Write,
        OperationType::Update => Permission::Write,
        OperationType::Delete => Permission::Write,
    }
}

pub trait InProject {
    fn project_id(&self) -> Uuid;
}
