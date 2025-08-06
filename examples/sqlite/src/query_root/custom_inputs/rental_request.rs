use seaography::async_graphql::dynamic::{InputObject, InputValue, TypeRef, ValueAccessor};
use seaography::{BuilderContext, CustomInput, GqlInputType, GqlInputValue, SeaResult};

#[derive(Clone)]
pub struct Input {
    pub customer: String,
    pub film: String,
    pub location: Option<String>,
}

impl CustomInput for Input {
    fn input_object() -> InputObject {
        InputObject::new("RentalRequestInput")
            .field(InputValue::new(
                "customer",
                TypeRef::named_nn(TypeRef::STRING),
            ))
            .field(InputValue::new("film", TypeRef::named_nn(TypeRef::STRING)))
            .field(InputValue::new("location", TypeRef::named(TypeRef::STRING)))
    }
}

impl GqlInputType for Input {
    fn gql_input_type_ref(_ctx: &'static BuilderContext) -> TypeRef {
        TypeRef::named_nn("RentalRequestInput".to_owned())
    }

    fn parse_input(
        context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        let object = value.unwrap().object().unwrap();

        let customer = GqlInputValue::parse_value(context, object.get("customer"))?;
        let film = GqlInputValue::parse_value(context, object.get("film"))?;
        let location = GqlInputValue::parse_value(context, object.get("location"))?;

        Ok(Input {
            customer,
            film,
            location,
        })
    }
}
