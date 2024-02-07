use std::collections::BTreeSet;

use async_graphql::dynamic::ObjectAccessor;
use heck::ToUpperCamelCase;
use sea_orm::{ActiveEnum, ColumnTrait, ColumnType, Condition, DynIden, EntityTrait};

use crate::{ActiveEnumBuilder, BuilderContext, FilterInfo, FilterOperation, SeaResult};

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

    /// used to get filter input name from string
    pub fn type_name_from_string(&self, enum_name: &str) -> String {
        self.context.active_enum_filter_input.type_name.as_ref()(enum_name)
    }

    /// used to map an active enum to an input filter info object
    pub fn filter_info<A: ActiveEnum>(&self) -> FilterInfo {
        let active_enum_builder = ActiveEnumBuilder {
            context: self.context,
        };

        FilterInfo {
            type_name: self.type_name::<A>(),
            base_type: active_enum_builder.type_name::<A>(),
            supported_operations: BTreeSet::from([
                FilterOperation::Equals,
                FilterOperation::NotEquals,
                FilterOperation::GreaterThan,
                FilterOperation::GreaterThanEquals,
                FilterOperation::LessThan,
                FilterOperation::LessThanEquals,
                FilterOperation::IsIn,
                FilterOperation::IsNotIn,
                FilterOperation::IsNull,
                FilterOperation::IsNotNull,
            ]),
        }
    }
}

/// used to update the query condition with enumeration filters
pub fn prepare_enumeration_condition<T>(
    context: &'static BuilderContext,
    filter: &ObjectAccessor,
    column: &T::Column,
    condition: Condition,
    type_name: &str,
) -> SeaResult<Condition>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let variants = if let ColumnType::Enum { name: _, variants } = column.def().get_column_type() {
        variants.clone()
    } else {
        return Ok(condition);
    };

    let extract_variant = move |input: &str| -> String {
        let variant = variants.iter().find(|orm_variant| {
            let orm_variant = orm_variant.to_string();

            let builder = ActiveEnumBuilder { context };

            let gql_variant = builder.variant_name(type_name, &orm_variant);

            gql_variant.eq(input)
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

    Ok(condition)
}
