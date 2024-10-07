use std::collections::{BTreeMap, BTreeSet};

use async_graphql::dynamic::{InputObject, InputValue, ObjectAccessor, TypeRef};
use sea_orm::{ColumnTrait, ColumnType, Condition, EntityTrait};

use crate::{
    prepare_enumeration_condition, ActiveEnumFilterInputBuilder, BuilderContext,
    EntityObjectBuilder, SeaResult, TypesMapHelper,
};

type FnFilterCondition =
    Box<dyn Fn(Condition, &ObjectAccessor) -> SeaResult<Condition> + Send + Sync>;

/// The configuration for FilterTypesMapHelper
pub struct FilterTypesMapConfig {
    /// used to map entity_name.column_name to a custom filter type
    pub overwrites: BTreeMap<String, Option<FilterType>>,
    /// used to map entity_name.column_name to a custom condition function
    pub condition_functions: BTreeMap<String, FnFilterCondition>,

    // basic string filter
    pub string_filter_info: FilterInfo,
    // basic text filter
    pub text_filter_info: FilterInfo,
    // basic integer filter
    pub integer_filter_info: FilterInfo,
    // basic float filter
    pub float_filter_info: FilterInfo,
    // basic boolean filter
    pub boolean_filter_info: FilterInfo,
    // basic id filter
    pub id_filter_info: FilterInfo,
}

impl std::default::Default for FilterTypesMapConfig {
    fn default() -> Self {
        Self {
            overwrites: BTreeMap::default(),
            condition_functions: BTreeMap::default(),
            string_filter_info: FilterInfo {
                type_name: "StringFilterInput".into(),
                base_type: TypeRef::STRING.into(),
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
                    FilterOperation::Contains,
                    FilterOperation::StartsWith,
                    FilterOperation::EndsWith,
                    FilterOperation::Like,
                    FilterOperation::NotLike,
                    FilterOperation::Between,
                    FilterOperation::NotBetween,
                ]),
            },
            text_filter_info: FilterInfo {
                type_name: "TextFilterInput".into(),
                base_type: TypeRef::STRING.into(),
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
                    FilterOperation::Between,
                    FilterOperation::NotBetween,
                ]),
            },
            integer_filter_info: FilterInfo {
                type_name: "IntegerFilterInput".into(),
                base_type: TypeRef::INT.into(),
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
                    FilterOperation::Between,
                    FilterOperation::NotBetween,
                ]),
            },
            float_filter_info: FilterInfo {
                type_name: "FloatFilterInput".into(),
                base_type: TypeRef::FLOAT.into(),
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
                    FilterOperation::Between,
                    FilterOperation::NotBetween,
                ]),
            },
            boolean_filter_info: FilterInfo {
                type_name: "BooleanFilterInput".into(),
                base_type: TypeRef::BOOLEAN.into(),
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
            },
            id_filter_info: FilterInfo {
                type_name: "IdentityFilterInput".into(),
                base_type: TypeRef::STRING.into(),
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
                    FilterOperation::Between,
                    FilterOperation::NotBetween,
                ]),
            },
        }
    }
}

/// The FilterTypesMapHelper
/// * provides basic input filter types
/// * provides entity filter object type mappings
/// * helper functions that assist filtering on queries
/// * helper function that generate input filter types
pub struct FilterTypesMapHelper {
    pub context: &'static BuilderContext,
}

impl FilterTypesMapHelper {
    /// used to map sea orm column type to filter type
    pub fn get_column_filter_type<T>(&self, column: &T::Column) -> Option<FilterType>
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };

        let entity_name = entity_object_builder.type_name::<T>();
        let column_name = entity_object_builder.column_name::<T>(column);

        // used to honor overwrites
        if let Some(ty) = self
            .context
            .filter_types
            .overwrites
            .get(&format!("{entity_name}.{column_name}"))
        {
            return ty.clone();
        }

        // default mappings
        match column.def().get_column_type() {
            ColumnType::Char(_) => Some(FilterType::Text),
            ColumnType::String(_) => Some(FilterType::String),
            ColumnType::Text => Some(FilterType::String),
            ColumnType::TinyInteger => Some(FilterType::Integer),
            ColumnType::SmallInteger => Some(FilterType::Integer),
            ColumnType::Integer => Some(FilterType::Integer),
            ColumnType::BigInteger => Some(FilterType::Integer),
            ColumnType::TinyUnsigned => Some(FilterType::Integer),
            ColumnType::SmallUnsigned => Some(FilterType::Integer),
            ColumnType::Unsigned => Some(FilterType::Integer),
            ColumnType::BigUnsigned => Some(FilterType::Integer),
            ColumnType::Float => Some(FilterType::Float),
            ColumnType::Double => Some(FilterType::Float),
            ColumnType::Decimal(_) => Some(FilterType::Text),
            ColumnType::DateTime => Some(FilterType::Text),
            ColumnType::Timestamp => Some(FilterType::Text),
            ColumnType::TimestampWithTimeZone => Some(FilterType::Text),
            ColumnType::Time => Some(FilterType::Text),
            ColumnType::Date => Some(FilterType::Text),
            ColumnType::Year => Some(FilterType::Integer),
            ColumnType::Interval(_, _) => Some(FilterType::Text),
            ColumnType::Binary(_) => None,
            ColumnType::VarBinary(_) => None,
            ColumnType::Bit(_) => None,
            ColumnType::VarBit(_) => None,
            ColumnType::Blob => None,
            ColumnType::Boolean => Some(FilterType::Boolean),
            ColumnType::Money(_) => Some(FilterType::Text),
            ColumnType::Json => None,
            ColumnType::JsonBinary => None,
            ColumnType::Uuid => Some(FilterType::Text),
            ColumnType::Custom(name) => Some(FilterType::Custom(name.to_string())),
            ColumnType::Enum { name, variants: _ } => {
                Some(FilterType::Enumeration(name.to_string()))
            }
            ColumnType::Array(_) => None,
            ColumnType::Cidr => Some(FilterType::Text),
            ColumnType::Inet => Some(FilterType::Text),
            ColumnType::MacAddr => Some(FilterType::Text),
            _ => None,
        }
    }

    /// used to get the GraphQL input value field for a SeaORM entity column
    pub fn get_column_filter_input_value<T>(&self, column: &T::Column) -> Option<InputValue>
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let column_name = entity_object_builder.column_name::<T>(column);

        let filter_type: Option<FilterType> = self.get_column_filter_type::<T>(column);

        match filter_type {
            Some(filter_type) => match filter_type {
                FilterType::Text => {
                    let info = &self.context.filter_types.text_filter_info;
                    Some(InputValue::new(
                        column_name,
                        TypeRef::named(info.type_name.clone()),
                    ))
                }
                FilterType::String => {
                    let info = &self.context.filter_types.string_filter_info;
                    Some(InputValue::new(
                        column_name,
                        TypeRef::named(info.type_name.clone()),
                    ))
                }
                FilterType::Integer => {
                    let info = &self.context.filter_types.integer_filter_info;
                    Some(InputValue::new(
                        column_name,
                        TypeRef::named(info.type_name.clone()),
                    ))
                }
                FilterType::Float => {
                    let info = &self.context.filter_types.float_filter_info;
                    Some(InputValue::new(
                        column_name,
                        TypeRef::named(info.type_name.clone()),
                    ))
                }
                FilterType::Boolean => {
                    let info = &self.context.filter_types.boolean_filter_info;
                    Some(InputValue::new(
                        column_name,
                        TypeRef::named(info.type_name.clone()),
                    ))
                }
                FilterType::Id => {
                    let info = &self.context.filter_types.id_filter_info;
                    Some(InputValue::new(
                        column_name,
                        TypeRef::named(info.type_name.clone()),
                    ))
                }
                FilterType::Enumeration(name) => {
                    let active_enum_filter_input_builder = ActiveEnumFilterInputBuilder {
                        context: self.context,
                    };

                    Some(InputValue::new(
                        column_name,
                        TypeRef::named(
                            active_enum_filter_input_builder.type_name_from_string(&name),
                        ),
                    ))
                }
                FilterType::Custom(type_name) => {
                    Some(InputValue::new(column_name, TypeRef::named(type_name)))
                }
            },
            None => None,
        }
    }

    /// used to get all basic input filter objects
    pub fn get_input_filters(&self) -> Vec<InputObject> {
        vec![
            self.generate_filter_input(&self.context.filter_types.text_filter_info),
            self.generate_filter_input(&self.context.filter_types.string_filter_info),
            self.generate_filter_input(&self.context.filter_types.integer_filter_info),
            self.generate_filter_input(&self.context.filter_types.float_filter_info),
            self.generate_filter_input(&self.context.filter_types.boolean_filter_info),
            self.generate_filter_input(&self.context.filter_types.id_filter_info),
        ]
    }

    /// used to convert a filter input info struct into input object
    pub fn generate_filter_input(&self, filter_info: &FilterInfo) -> InputObject {
        filter_info.supported_operations.iter().fold(
            InputObject::new(filter_info.type_name.to_string()),
            |object, cur| {
                let field = match cur {
                    FilterOperation::Equals => {
                        InputValue::new("eq", TypeRef::named(filter_info.base_type.clone()))
                    }
                    FilterOperation::NotEquals => {
                        InputValue::new("ne", TypeRef::named(filter_info.base_type.clone()))
                    }
                    FilterOperation::GreaterThan => {
                        InputValue::new("gt", TypeRef::named(filter_info.base_type.clone()))
                    }
                    FilterOperation::GreaterThanEquals => {
                        InputValue::new("gte", TypeRef::named(filter_info.base_type.clone()))
                    }
                    FilterOperation::LessThan => {
                        InputValue::new("lt", TypeRef::named(filter_info.base_type.clone()))
                    }
                    FilterOperation::LessThanEquals => {
                        InputValue::new("lte", TypeRef::named(filter_info.base_type.clone()))
                    }
                    FilterOperation::IsIn => InputValue::new(
                        "is_in",
                        TypeRef::named_nn_list(filter_info.base_type.clone()),
                    ),
                    FilterOperation::IsNotIn => InputValue::new(
                        "is_not_in",
                        TypeRef::named_nn_list(filter_info.base_type.clone()),
                    ),
                    FilterOperation::IsNull => {
                        InputValue::new("is_null", TypeRef::named(filter_info.base_type.clone()))
                    }
                    FilterOperation::IsNotNull => InputValue::new(
                        "is_not_null",
                        TypeRef::named(filter_info.base_type.clone()),
                    ),
                    FilterOperation::Contains => {
                        InputValue::new("contains", TypeRef::named(filter_info.base_type.clone()))
                    }
                    FilterOperation::StartsWith => InputValue::new(
                        "starts_with",
                        TypeRef::named(filter_info.base_type.clone()),
                    ),
                    FilterOperation::EndsWith => {
                        InputValue::new("ends_with", TypeRef::named(filter_info.base_type.clone()))
                    }
                    FilterOperation::Like => {
                        InputValue::new("like", TypeRef::named(filter_info.base_type.clone()))
                    }
                    FilterOperation::NotLike => {
                        InputValue::new("not_like", TypeRef::named(filter_info.base_type.clone()))
                    }
                    FilterOperation::Between => InputValue::new(
                        "between",
                        TypeRef::named_nn_list(filter_info.base_type.clone()),
                    ),
                    FilterOperation::NotBetween => InputValue::new(
                        "not_between",
                        TypeRef::named_nn_list(filter_info.base_type.clone()),
                    ),
                };
                object.field(field)
            },
        )
    }

    /// used to parse a filter input object and update the query condition
    pub fn prepare_column_condition<T>(
        &self,
        mut condition: Condition,
        filter: &ObjectAccessor,
        column: &T::Column,
    ) -> SeaResult<Condition>
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let types_map_helper = TypesMapHelper {
            context: self.context,
        };

        let filter_info = match self.get_column_filter_type::<T>(column) {
            Some(filter_type) => match filter_type {
                FilterType::Text => &self.context.filter_types.text_filter_info,
                FilterType::String => &self.context.filter_types.string_filter_info,
                FilterType::Integer => &self.context.filter_types.integer_filter_info,
                FilterType::Float => &self.context.filter_types.float_filter_info,
                FilterType::Boolean => &self.context.filter_types.boolean_filter_info,
                FilterType::Id => &self.context.filter_types.id_filter_info,
                FilterType::Enumeration(name) => {
                    return prepare_enumeration_condition::<T>(
                        self.context,
                        filter,
                        column,
                        condition,
                        &name,
                    );
                }
                FilterType::Custom(_) => {
                    let entity_object_builder = EntityObjectBuilder {
                        context: self.context,
                    };

                    let entity_name = entity_object_builder.type_name::<T>();
                    let column_name = entity_object_builder.column_name::<T>(column);

                    if let Some(filter_condition_fn) = self
                        .context
                        .filter_types
                        .condition_functions
                        .get(&format!("{entity_name}.{column_name}"))
                    {
                        return filter_condition_fn(condition, filter);
                    } else {
                        // FIXME: add log warning to console
                        return Ok(condition);
                    }
                }
            },
            None => return Ok(condition),
        };

        for operation in filter_info.supported_operations.iter() {
            match operation {
                FilterOperation::Equals => {
                    if let Some(value) = filter.get("eq") {
                        let value = types_map_helper
                            .async_graphql_value_to_sea_orm_value::<T>(column, &value)?;
                        condition = condition.add(column.eq(value));
                    }
                }
                FilterOperation::NotEquals => {
                    if let Some(value) = filter.get("ne") {
                        let value = types_map_helper
                            .async_graphql_value_to_sea_orm_value::<T>(column, &value)?;
                        condition = condition.add(column.ne(value));
                    }
                }
                FilterOperation::GreaterThan => {
                    if let Some(value) = filter.get("gt") {
                        let value = types_map_helper
                            .async_graphql_value_to_sea_orm_value::<T>(column, &value)?;
                        condition = condition.add(column.gt(value));
                    }
                }
                FilterOperation::GreaterThanEquals => {
                    if let Some(value) = filter.get("gte") {
                        let value = types_map_helper
                            .async_graphql_value_to_sea_orm_value::<T>(column, &value)?;
                        condition = condition.add(column.gte(value));
                    }
                }
                FilterOperation::LessThan => {
                    if let Some(value) = filter.get("lt") {
                        let value = types_map_helper
                            .async_graphql_value_to_sea_orm_value::<T>(column, &value)?;
                        condition = condition.add(column.lt(value));
                    }
                }
                FilterOperation::LessThanEquals => {
                    if let Some(value) = filter.get("lte") {
                        let value = types_map_helper
                            .async_graphql_value_to_sea_orm_value::<T>(column, &value)?;
                        condition = condition.add(column.lte(value));
                    }
                }
                FilterOperation::IsIn => {
                    if let Some(value) = filter.get("is_in") {
                        let value = value
                            .list()?
                            .iter()
                            .map(|v| {
                                types_map_helper
                                    .async_graphql_value_to_sea_orm_value::<T>(column, &v)
                            })
                            .collect::<SeaResult<Vec<_>>>()?;
                        condition = condition.add(column.is_in(value));
                    }
                }
                FilterOperation::IsNotIn => {
                    if let Some(value) = filter.get("is_not_in") {
                        let value = value
                            .list()?
                            .iter()
                            .map(|v| {
                                types_map_helper
                                    .async_graphql_value_to_sea_orm_value::<T>(column, &v)
                            })
                            .collect::<SeaResult<Vec<_>>>()?;
                        condition = condition.add(column.is_not_in(value));
                    }
                }
                FilterOperation::IsNull => {
                    if filter.get("is_null").is_some() {
                        condition = condition.add(column.is_null());
                    }
                }
                FilterOperation::IsNotNull => {
                    if filter.get("is_not_null").is_some() {
                        condition = condition.add(column.is_not_null());
                    }
                }
                FilterOperation::Contains => {
                    if let Some(value) = filter.get("contains") {
                        let value = types_map_helper
                            .async_graphql_value_to_sea_orm_value::<T>(column, &value)?;
                        let s = match value {
                            sea_orm::sea_query::Value::String(Some(s)) => s.to_string(),
                            _ => value.to_string(),
                        };
                        condition = condition.add(column.contains(s));
                    }
                }
                FilterOperation::StartsWith => {
                    if let Some(value) = filter.get("starts_with") {
                        let value = types_map_helper
                            .async_graphql_value_to_sea_orm_value::<T>(column, &value)?;
                        let s = match value {
                            sea_orm::sea_query::Value::String(Some(s)) => s.to_string(),
                            _ => value.to_string(),
                        };
                        condition = condition.add(column.starts_with(s));
                    }
                }
                FilterOperation::EndsWith => {
                    if let Some(value) = filter.get("ends_with") {
                        let value = types_map_helper
                            .async_graphql_value_to_sea_orm_value::<T>(column, &value)?;
                        let s = match value {
                            sea_orm::sea_query::Value::String(Some(s)) => s.to_string(),
                            _ => value.to_string(),
                        };
                        condition = condition.add(column.ends_with(s));
                    }
                }
                FilterOperation::Like => {
                    if let Some(value) = filter.get("like") {
                        let value = types_map_helper
                            .async_graphql_value_to_sea_orm_value::<T>(column, &value)?;
                        let s = match value {
                            sea_orm::sea_query::Value::String(Some(s)) => s.to_string(),
                            _ => value.to_string(),
                        };
                        condition = condition.add(column.like(s));
                    }
                }
                FilterOperation::NotLike => {
                    if let Some(value) = filter.get("not_like") {
                        let value = types_map_helper
                            .async_graphql_value_to_sea_orm_value::<T>(column, &value)?;
                        condition = condition.add(column.not_like(value.to_string()));
                    }
                }
                FilterOperation::Between => {
                    if let Some(value) = filter.get("between") {
                        let value = value
                            .list()?
                            .iter()
                            .map(|v| {
                                types_map_helper
                                    .async_graphql_value_to_sea_orm_value::<T>(column, &v)
                            })
                            .collect::<SeaResult<Vec<_>>>()?;

                        let a = value[0].clone();
                        let b = value[1].clone();

                        condition = condition.add(column.between(a, b));
                    }
                }
                FilterOperation::NotBetween => {
                    if let Some(value) = filter.get("not_between") {
                        let value = value
                            .list()?
                            .iter()
                            .map(|v| {
                                types_map_helper
                                    .async_graphql_value_to_sea_orm_value::<T>(column, &v)
                            })
                            .collect::<SeaResult<Vec<_>>>()?;

                        let a = value[0].clone();
                        let b = value[1].clone();

                        condition = condition.add(column.not_between(a, b));
                    }
                }
            }
        }

        Ok(condition)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FilterType {
    Text,
    String,
    Integer,
    Float,
    Boolean,
    Id,
    Enumeration(String),
    Custom(String),
}

#[derive(Clone, Debug)]
pub struct FilterInfo {
    pub type_name: String,
    pub base_type: String,
    pub supported_operations: BTreeSet<FilterOperation>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FilterOperation {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,
    IsIn,
    IsNotIn,
    IsNull,
    IsNotNull,
    Contains,
    StartsWith,
    EndsWith,
    Like,
    NotLike,
    Between,
    NotBetween,
}
