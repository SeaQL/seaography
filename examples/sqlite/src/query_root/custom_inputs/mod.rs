use seaography::async_graphql::dynamic::{InputObject, InputValue, TypeRef, ValueAccessor};
use seaography::{
    BuilderContext, CustomInput, GqlInputType, GqlInputValue, GqlScalarValueType, SeaResult,
};

#[derive(Clone)]
pub struct RentalRequest {
    pub customer: String,
    pub film: String,
    pub location: Option<Location>,
}

#[derive(Clone)]
pub struct Location {
    pub city: String,
    pub county: Option<String>,
}

impl CustomInput for RentalRequest {
    fn input_object(ctx: &'static BuilderContext) -> InputObject {
        InputObject::new("RentalRequestInput")
            .field(InputValue::new("customer", String::gql_input_type_ref(ctx)))
            .field(InputValue::new("film", String::gql_input_type_ref(ctx)))
            .field(InputValue::new(
                "location",
                Option::<Location>::gql_input_type_ref(ctx),
            ))
    }
}

impl GqlInputType for RentalRequest {
    fn gql_input_type_ref(_ctx: &'static BuilderContext) -> TypeRef {
        TypeRef::named_nn("RentalRequestInput".to_owned())
    }

    fn parse_value(
        ctx: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        let object = value.unwrap().object().unwrap();

        let customer = String::parse_value(ctx, object.get("customer"))?;
        let film = String::parse_value(ctx, object.get("film"))?;
        let location = Option::<Location>::parse_value(ctx, object.get("location"))?;

        Ok(RentalRequest {
            customer,
            film,
            location,
        })
    }
}

impl CustomInput for Location {
    fn input_object(ctx: &'static BuilderContext) -> InputObject {
        InputObject::new("LocationInput")
            .field(InputValue::new("city", String::gql_input_type_ref(ctx)))
            .field(InputValue::new(
                "county",
                Option::<String>::gql_input_type_ref(ctx),
            ))
    }
}

impl GqlInputType for Location {
    fn gql_input_type_ref(_ctx: &'static BuilderContext) -> TypeRef {
        TypeRef::named_nn("LocationInput".to_owned())
    }

    fn parse_value(
        ctx: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        let object = value.unwrap().object().unwrap();

        let city = String::parse_value(ctx, object.get("city"))?;
        let county = Option::<String>::parse_value(ctx, object.get("county"))?;

        Ok(Location { city, county })
    }
}
