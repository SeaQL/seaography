use std::collections::{BTreeMap, BTreeSet};

use async_graphql::dynamic::{InputObject, InputValue, ObjectAccessor, TypeRef};
use sea_orm::{ColumnTrait, ColumnType, Condition, EntityTrait, Iterable};

use crate::{
    ActiveEnumFilterInputBuilder, BuilderContext, EntityObjectBuilder, SeaResult, TypesMapHelper,
};

/// The configuration structure for FilterInputBuilder
pub struct FilterInputConfig {
    /// the filter input type name formatter function
    pub type_name: crate::SimpleNamingFn,
    /// used to define entity.column filter inputs
    pub entity_filter_overwrites: BTreeMap<String, String>,

    pub string_filter_info: FilterInfo,
    pub text_filter_info: FilterInfo,
    pub integer_filter_info: FilterInfo,
    pub float_filter_info: FilterInfo,
    pub boolean_filter_info: FilterInfo,
    pub id_filter_info: FilterInfo,
}

impl std::default::Default for FilterInputConfig {
    fn default() -> Self {
        FilterInputConfig {
            type_name: Box::new(|object_name: &str| -> String {
                format!("{}FilterInput", object_name)
            }),
            entity_filter_overwrites: BTreeMap::new(),
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

/// This builder is used to produce the filter input object of a SeaORM entity
pub struct FilterInputBuilder {
    pub context: &'static BuilderContext,
}

impl FilterInputBuilder {
    /// used to get the filter input object name
    /// object_name is the name of the SeaORM Entity GraphQL object
    pub fn type_name(&self, object_name: &str) -> String {
        self.context.filter_input.type_name.as_ref()(object_name)
    }

    /// used to produce the filter input object of a SeaORM entity
    pub fn to_object<T>(&self) -> InputObject
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let entity_name = entity_object_builder.type_name::<T>();

        let filter_name = self.type_name(&entity_name);

        let object = T::Column::iter().fold(InputObject::new(&filter_name), |object, column| {
            let column_name = entity_object_builder.column_name::<T>(&column);

            match self.map_column_type_to_filter_type(column.def().get_column_type()) {
                Some(filter_type) => match filter_type {
                    FilterType::Text => {
                        let info = &self.context.filter_input.text_filter_info;
                        object.field(InputValue::new(
                            column_name,
                            TypeRef::named(info.type_name.clone()),
                        ))
                    }
                    FilterType::String => {
                        let info = &self.context.filter_input.string_filter_info;
                        object.field(InputValue::new(
                            column_name,
                            TypeRef::named(info.type_name.clone()),
                        ))
                    }
                    FilterType::Integer => {
                        let info = &self.context.filter_input.integer_filter_info;
                        object.field(InputValue::new(
                            column_name,
                            TypeRef::named(info.type_name.clone()),
                        ))
                    }
                    FilterType::Float => {
                        let info = &self.context.filter_input.float_filter_info;
                        object.field(InputValue::new(
                            column_name,
                            TypeRef::named(info.type_name.clone()),
                        ))
                    }
                    FilterType::Boolean => {
                        let info = &self.context.filter_input.boolean_filter_info;
                        object.field(InputValue::new(
                            column_name,
                            TypeRef::named(info.type_name.clone()),
                        ))
                    }
                    FilterType::Id => {
                        let info = &self.context.filter_input.id_filter_info;
                        object.field(InputValue::new(
                            column_name,
                            TypeRef::named(info.type_name.clone()),
                        ))
                    }
                    FilterType::Enumeration(name) => {
                        let active_enum_filter_input_builder = ActiveEnumFilterInputBuilder {
                            context: self.context,
                        };

                        object.field(InputValue::new(
                            column_name,
                            TypeRef::named(
                                active_enum_filter_input_builder.type_name_from_string(&name),
                            ),
                        ))
                    }
                    FilterType::Custom(_) => {
                        match self
                            .context
                            .filter_input
                            .entity_filter_overwrites
                            .get(&format!("{}.{}", entity_name, column_name))
                        {
                            Some(ty) => {
                                object.field(InputValue::new(column_name, TypeRef::named(ty)))
                            }
                            None => object,
                        }
                    }
                },
                None => object,
            }
        });

        object
            .field(InputValue::new("and", TypeRef::named_nn_list(&filter_name)))
            .field(InputValue::new("or", TypeRef::named_nn_list(&filter_name)))
    }

    /// used to get all input filter objects
    pub fn get_input_filters(&self) -> Vec<InputObject> {
        vec![
            self.generate_filter_input(&self.context.filter_input.text_filter_info),
            self.generate_filter_input(&self.context.filter_input.string_filter_info),
            self.generate_filter_input(&self.context.filter_input.integer_filter_info),
            self.generate_filter_input(&self.context.filter_input.float_filter_info),
            self.generate_filter_input(&self.context.filter_input.boolean_filter_info),
            self.generate_filter_input(&self.context.filter_input.id_filter_info),
        ]
    }

    /// used to map sea orm column type to available filter types
    pub fn map_column_type_to_filter_type(&self, column_type: &ColumnType) -> Option<FilterType> {
        match column_type {
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
            ColumnType::Year(_) => Some(FilterType::Integer),
            ColumnType::Interval(_, _) => Some(FilterType::Text),
            ColumnType::Binary(_) => None,
            ColumnType::VarBinary(_) => None,
            ColumnType::Bit(_) => None,
            ColumnType::VarBit(_) => None,
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

        let filter_info = match self.map_column_type_to_filter_type(column.def().get_column_type())
        {
            Some(filter_type) => match filter_type {
                FilterType::Text => &self.context.filter_input.text_filter_info,
                FilterType::String => &self.context.filter_input.string_filter_info,
                FilterType::Integer => &self.context.filter_input.integer_filter_info,
                FilterType::Float => &self.context.filter_input.float_filter_info,
                FilterType::Boolean => &self.context.filter_input.boolean_filter_info,
                FilterType::Id => &self.context.filter_input.id_filter_info,
                FilterType::Enumeration(_) => return Ok(condition),
                FilterType::Custom(_) => return Ok(condition),
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
                        condition = condition.add(column.contains(&value.to_string()));
                    }
                }
                FilterOperation::StartsWith => {
                    if let Some(value) = filter.get("starts_with") {
                        let value = types_map_helper
                            .async_graphql_value_to_sea_orm_value::<T>(column, &value)?;
                        condition = condition.add(column.starts_with(&value.to_string()));
                    }
                }
                FilterOperation::EndsWith => {
                    if let Some(value) = filter.get("ends_with") {
                        let value = types_map_helper
                            .async_graphql_value_to_sea_orm_value::<T>(column, &value)?;
                        condition = condition.add(column.ends_with(&value.to_string()));
                    }
                }
                FilterOperation::Like => {
                    if let Some(value) = filter.get("like") {
                        let value = types_map_helper
                            .async_graphql_value_to_sea_orm_value::<T>(column, &value)?;
                        condition = condition.add(column.like(&value.to_string()));
                    }
                }
                FilterOperation::NotLike => {
                    if let Some(value) = filter.get("not_like") {
                        let value = types_map_helper
                            .async_graphql_value_to_sea_orm_value::<T>(column, &value)?;
                        condition = condition.add(column.not_like(&value.to_string()));
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

                        let a = value.get(0).unwrap().clone();
                        let b = value.get(1).unwrap().clone();

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

                        let a = value.get(0).unwrap().clone();
                        let b = value.get(1).unwrap().clone();

                        condition = condition.add(column.not_between(a, b));
                    }
                }
            }
        }

        Ok(condition)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

pub struct FilterInfo {
    pub type_name: String,
    pub base_type: String,
    pub supported_operations: BTreeSet<FilterOperation>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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
