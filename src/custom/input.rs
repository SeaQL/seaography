pub trait CustomInput {
    fn input_object() -> async_graphql::dynamic::InputObject;
}
