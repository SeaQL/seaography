use crate::BuilderContext;

pub trait CustomInput {
    fn input_object(context: &'static BuilderContext) -> async_graphql::dynamic::InputObject;
}
