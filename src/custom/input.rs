use crate::{
    converted_null_to_sea_orm_value, converted_value_to_sea_orm_value, BuilderContext, SeaResult,
    TypesMapHelper,
};
use async_graphql::dynamic::ValueAccessor;

pub trait CustomInput {
    fn input_object(context: &'static BuilderContext) -> async_graphql::dynamic::InputObject;
}

pub trait GqlInputValue: Sized {
    fn parse_value(
        context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self>;
}

impl<T> GqlInputValue for T
where
    T: sea_orm::sea_query::ValueType,
{
    fn parse_value(
        context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        let types_map_helper = TypesMapHelper { context };
        let column_type = T::column_type();
        let column_type =
            types_map_helper.sea_orm_column_type_to_converted_type("", "", &column_type);

        if value.is_none() {
            Ok(converted_null_to_sea_orm_value(&column_type).unwrap())
        } else {
            let value = converted_value_to_sea_orm_value(&column_type, &value.unwrap(), "", "")?;
            Ok(value.unwrap())
        }
    }
}
