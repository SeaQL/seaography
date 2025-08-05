use seaography::async_graphql::dynamic::{InputObject, InputValue, TypeRef, ValueAccessor};
use seaography::{
    converted_value_to_sea_orm_value, BuilderContext, CustomInput, GqlInputType, SeaResult,
    TypesMapHelper,
};

#[derive(Clone)]
pub struct Input {
    pub customer: String,
    pub film: String,
}

impl CustomInput for Input {
    fn input_object() -> InputObject {
        InputObject::new("RentalRequestInput")
            .field(InputValue::new(
                "customer",
                TypeRef::named_nn(TypeRef::STRING),
            ))
            .field(InputValue::new("film", TypeRef::named_nn(TypeRef::STRING)))
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

        let customer = {
            let ty = sea_orm::ColumnType::string(None);
            let types_map_helper = TypesMapHelper { context };
            let column_type = types_map_helper.get_column_type_helper("", "", &ty);

            converted_value_to_sea_orm_value(
                &column_type,
                object.get("customer").as_ref().unwrap(),
                "",
                "",
            )?
        };
        let film = {
            let ty = sea_orm::ColumnType::string(None);
            let types_map_helper = TypesMapHelper { context };
            let column_type = types_map_helper.get_column_type_helper("", "", &ty);

            converted_value_to_sea_orm_value(
                &column_type,
                object.get("film").as_ref().unwrap(),
                "",
                "",
            )?
        };

        Ok(Input {
            customer: customer.unwrap(),
            film: film.unwrap(),
        })
    }
}
