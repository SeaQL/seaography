use crate::BuilderContext;

pub trait CustomOutput {
    fn basic_object(context: &'static BuilderContext) -> async_graphql::dynamic::Object;
}
