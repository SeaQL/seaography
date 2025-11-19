use std::collections::BTreeSet;

use async_graphql::dynamic::ObjectAccessor;
use heck::ToUpperCamelCase;
use sea_orm::{ActiveEnum, ColumnTrait, ColumnType, Condition, DynIden, EntityTrait};

use crate::{
    format_variant, ActiveEnumBuilder, BuilderContext, FilterInfo, FilterOperation, SeaResult,
    SeaographyError,
};

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
    filter: &ObjectAccessor,
    column: &T::Column,
    condition: Condition,
) -> SeaResult<Condition>
where
    T: EntityTrait,
{
    let column_def = column.def();

    let ColumnType::Enum {
        name: enum_type_name,
        variants,
    } = column_def.get_column_type()
    else {
        return Ok(condition);
    };

    let extract_variant = |input: &str| -> SeaResult<String> {
        let variant = variants.iter().find(|variant| {
            let variant = format_variant(&variant.to_string());
            variant.eq(input)
        });
        Ok(variant
            .ok_or_else(|| {
                SeaographyError::TypeConversionError(enum_type_name.to_string(), input.to_owned())
            })?
            .to_string())
    };

    let condition = if let Some(data) = filter.get("eq") {
        let data = data.enum_name()?;
        condition.add(column.eq(extract_variant(data)?))
    } else {
        condition
    };

    let condition = if let Some(data) = filter.get("ne") {
        let data = data.enum_name()?;
        condition.add(column.ne(extract_variant(data)?))
    } else {
        condition
    };

    let condition = if let Some(data) = filter.get("gt") {
        let data = data.enum_name()?;
        condition.add(column.gt(extract_variant(data)?))
    } else {
        condition
    };

    let condition = if let Some(data) = filter.get("gte") {
        let data = data.enum_name()?;
        condition.add(column.gte(extract_variant(data)?))
    } else {
        condition
    };

    let condition = if let Some(data) = filter.get("lt") {
        let data = data.enum_name()?;
        condition.add(column.lt(extract_variant(data)?))
    } else {
        condition
    };

    let condition = if let Some(data) = filter.get("lte") {
        let data = data.enum_name()?;
        condition.add(column.lte(extract_variant(data)?))
    } else {
        condition
    };

    let condition = match filter.get("is_in") {
        Some(data) => {
            let mut values = Vec::new();
            for item in data.list()?.iter() {
                values.push(extract_variant(item.enum_name()?)?);
            }

            condition.add(column.is_in(values))
        }
        None => condition,
    };

    let condition = match filter.get("is_not_in") {
        Some(data) => {
            let mut values = Vec::new();
            for item in data.list()?.iter() {
                values.push(extract_variant(item.enum_name()?)?);
            }

            condition.add(column.is_not_in(values))
        }
        None => condition,
    };

    let condition = match filter.get("is_null") {
        Some(data) => {
            let data = data.boolean()?;

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
