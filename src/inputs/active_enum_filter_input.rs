use async_graphql::dynamic::{InputObject, InputValue, ObjectAccessor, TypeRef};
use heck::ToUpperCamelCase;
use sea_orm::{ActiveEnum, ColumnTrait, Condition, DynIden, Iden};

use crate::{ActiveEnumBuilder, BuilderContext};

/// The configuration structure for ActiveEnumFilterInputConfig
pub struct ActiveEnumFilterInputConfig {
    /// used to format type_name
    pub type_name: crate::SimpleNamingFn,
}

impl std::default::Default for ActiveEnumFilterInputConfig {
    fn default() -> Self {
        ActiveEnumFilterInputConfig {
            type_name: Box::new(|enum_name: &str| -> String {
                format!("{}EnumFilterInput", enum_name.to_upper_camel_case())
            }),
        }
    }
}

/// This builder produces a filter input for a SeaORM enumeration
pub struct ActiveEnumFilterInputBuilder {
    pub context: &'static BuilderContext,
}

impl ActiveEnumFilterInputBuilder {
    /// used to get filter input name for SeaORM enumeration
    pub fn type_name<A: ActiveEnum>(&self) -> String {
        let enum_name = A::name().to_string();
        self.context.active_enum_filter_input.type_name.as_ref()(&enum_name)
    }

    /// used to get filter input name for SeaORM enumeration Iden
    pub fn type_name_from_iden(&self, enum_name: &DynIden) -> String {
        let enum_name = enum_name.to_string();
        self.context.active_enum_filter_input.type_name.as_ref()(&enum_name)
    }

    /// used to get filter input object for SeaORM enumeration
    pub fn input_object<A: ActiveEnum>(&self) -> InputObject {
        let active_enum_builder = ActiveEnumBuilder {
            context: self.context,
        };

        let name = self.type_name::<A>();

        let enum_name = active_enum_builder.type_name::<A>();

        InputObject::new(name)
            .field(InputValue::new("eq", TypeRef::named(&enum_name)))
            .field(InputValue::new("ne", TypeRef::named(&enum_name)))
            .field(InputValue::new("gt", TypeRef::named(&enum_name)))
            .field(InputValue::new("gte", TypeRef::named(&enum_name)))
            .field(InputValue::new("lt", TypeRef::named(&enum_name)))
            .field(InputValue::new("lte", TypeRef::named(&enum_name)))
            .field(InputValue::new("is_in", TypeRef::named_nn_list(&enum_name)))
            .field(InputValue::new(
                "is_not_in",
                TypeRef::named_nn_list(&enum_name),
            ))
            .field(InputValue::new("is_null", TypeRef::named(TypeRef::BOOLEAN)))
    }
}

/// used to update the query condition with enumeration filters
pub fn prepare_enumeration_condition<T>(
    filter: &ObjectAccessor,
    column: T,
    variants: &[std::sync::Arc<dyn Iden>],
    condition: Condition,
) -> Condition
where
    T: ColumnTrait,
{
    let extract_variant = move |input: &str| -> String {
        let variant = variants.iter().find(|variant| {
            let variant = variant
                .to_string()
                .to_upper_camel_case()
                .to_ascii_uppercase();
            variant.eq(input)
        });
        variant.unwrap().to_string()
    };

    let condition = if let Some(data) = filter.get("eq") {
        let data = data.enum_name().unwrap();
        condition.add(column.eq(extract_variant(data)))
    } else {
        condition
    };

    let condition = if let Some(data) = filter.get("ne") {
        let data = data.enum_name().unwrap();
        condition.add(column.ne(extract_variant(data)))
    } else {
        condition
    };

    let condition = if let Some(data) = filter.get("gt") {
        let data = data.enum_name().unwrap();
        condition.add(column.gt(extract_variant(data)))
    } else {
        condition
    };

    let condition = if let Some(data) = filter.get("gte") {
        let data = data.enum_name().unwrap();
        condition.add(column.gte(extract_variant(data)))
    } else {
        condition
    };

    let condition = if let Some(data) = filter.get("lt") {
        let data = data.enum_name().unwrap();
        condition.add(column.lt(extract_variant(data)))
    } else {
        condition
    };

    let condition = if let Some(data) = filter.get("lte") {
        let data = data.enum_name().unwrap();
        condition.add(column.lte(extract_variant(data)))
    } else {
        condition
    };

    let condition = match filter.get("is_in") {
        Some(data) => {
            let data: Vec<_> = data
                .list()
                .unwrap()
                .iter()
                .map(|item| item.enum_name().unwrap().to_string())
                .map(|v| extract_variant(&v))
                .collect();

            condition.add(column.is_in(data))
        }
        None => condition,
    };

    let condition = match filter.get("is_not_in") {
        Some(data) => {
            let data: Vec<_> = data
                .list()
                .unwrap()
                .iter()
                .map(|item| item.enum_name().unwrap().to_string())
                .map(|v| extract_variant(&v))
                .collect();

            condition.add(column.is_not_in(data))
        }
        None => condition,
    };

    let condition = match filter.get("is_null") {
        Some(data) => {
            let data = data.boolean().unwrap();

            if data {
                condition.add(column.is_null())
            } else {
                condition
            }
        }
        None => condition,
    };

    condition
}
