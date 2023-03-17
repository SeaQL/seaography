use async_graphql::dynamic::{InputObject, InputValue, ObjectAccessor, TypeRef};
use heck::ToUpperCamelCase;
use sea_orm::{ActiveEnum, ColumnTrait, Condition, DynIden, Iden};

use crate::{ActiveEnumBuilder, BuilderContext};

#[derive(Clone, Debug)]
pub struct ActiveEnumFilterInputConfig {
    pub type_name: String,
}

impl std::default::Default for ActiveEnumFilterInputConfig {
    fn default() -> Self {
        ActiveEnumFilterInputConfig {
            type_name: "EnumFilterInput".into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ActiveEnumFilterInputBuilder {
    pub context: &'static BuilderContext,
}

impl ActiveEnumFilterInputBuilder {
    // FIXME: use context naming function
    pub fn type_name<A: ActiveEnum>(&self) -> String {
        format!(
            "{}{}",
            A::name().to_string().to_upper_camel_case(),
            self.context.active_enum.type_name
        )
    }

    // FIXME: use context naming function
    pub fn type_name_from_iden(&self, name: &DynIden) -> String {
        format!(
            "{}{}",
            name.to_string().to_upper_camel_case(),
            self.context.active_enum.type_name
        )
    }

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
