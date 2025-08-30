use crate::BuilderContext;

pub trait CustomOutput {
    fn type_name() -> &'static str;

    fn basic_object(context: &'static BuilderContext) -> async_graphql::dynamic::Object;
}
