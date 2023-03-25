use async_graphql::dynamic::{InputObject, InputValue, ObjectAccessor, TypeRef};
use sea_orm::{ColumnTrait, ColumnType, Condition, EntityTrait, Iterable, Value};

use crate::{ActiveEnumFilterInputBuilder, BuilderContext, EntityObjectBuilder};

pub struct FilterInputConfig {
    pub type_name: Box<dyn Fn(&str) -> String + Sync + Send>,
    pub string_type: String,
    pub integer_type: String,
    pub float_type: String,
    pub text_type: String,
    pub boolean_type: String,
    pub id_type: String,
}

impl std::default::Default for FilterInputConfig {
    fn default() -> Self {
        FilterInputConfig {
            type_name: Box::new(|object_name: &str| -> String {
                format!("{}FilterInput", object_name)
            }),
            string_type: "StringFilterInput".into(),
            integer_type: "IntegerFilterInput".into(),
            float_type: "FloatFilterInput".into(),
            text_type: "TextFilterInput".into(),
            boolean_type: "BooleanFilterInput".into(),
            id_type: "IdFilterInput".into(),
        }
    }
}

pub struct FilterInputBuilder {
    pub context: &'static BuilderContext,
}

impl FilterInputBuilder {
    pub fn type_name(&self, object_name: &str) -> String {
        self.context.filter_input.type_name.as_ref()(object_name)
    }

    pub fn to_object<T>(&self) -> InputObject
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let active_enum_filter_input_builder = ActiveEnumFilterInputBuilder {
            context: self.context,
        };
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let object_name = entity_object_builder.type_name::<T>();

        let name = self.type_name(&object_name);

        let object = T::Column::iter().fold(InputObject::new(&name), |object, column| {
            let column_name = entity_object_builder.column_name::<T>(column);

            let field = match column.def().get_column_type() {
                ColumnType::Char(_) | ColumnType::String(_) | ColumnType::Text => {
                    Some(InputValue::new(
                        column_name,
                        TypeRef::named(&self.context.filter_input.string_type),
                    ))
                }
                ColumnType::TinyInteger
                | ColumnType::SmallInteger
                | ColumnType::Integer
                | ColumnType::BigInteger
                | ColumnType::TinyUnsigned
                | ColumnType::SmallUnsigned
                | ColumnType::Unsigned
                | ColumnType::BigUnsigned => Some(InputValue::new(
                    column_name,
                    TypeRef::named(&self.context.filter_input.integer_type),
                )),
                ColumnType::Float | ColumnType::Double => Some(InputValue::new(
                    column_name,
                    TypeRef::named(&self.context.filter_input.float_type),
                )),
                ColumnType::Decimal(_) | ColumnType::Money(_) => Some(InputValue::new(
                    column_name,
                    TypeRef::named(&self.context.filter_input.text_type),
                )),
                ColumnType::DateTime
                | ColumnType::Timestamp
                | ColumnType::TimestampWithTimeZone
                | ColumnType::Time
                | ColumnType::Date => Some(InputValue::new(
                    column_name,
                    TypeRef::named(&self.context.filter_input.text_type),
                )),
                ColumnType::Year(_) => Some(InputValue::new(
                    column_name,
                    TypeRef::named(&self.context.filter_input.integer_type),
                )),
                ColumnType::Interval(_, _) => Some(InputValue::new(
                    column_name,
                    TypeRef::named(&self.context.filter_input.text_type),
                )),
                // FIXME: binary type
                // ColumnType::Binary(_) |
                // ColumnType::VarBinary(_) |
                // ColumnType::Bit(_) |
                // ColumnType::VarBit(_) => Some(InputValue::new(
                //     column_name,
                //     TypeRef::named(&self.context.filter_input.text_type),
                // )),
                ColumnType::Boolean => Some(InputValue::new(
                    column_name,
                    TypeRef::named(&self.context.filter_input.boolean_type),
                )),
                // FIXME: json type
                // ColumnType::Json | ColumnType::JsonBinary => Some(InputValue::new(
                //     column_name,
                //     TypeRef::named(&self.context.filter_input.text_type),
                // )),
                ColumnType::Uuid => Some(InputValue::new(
                    column_name,
                    TypeRef::named(&self.context.filter_input.text_type),
                )),
                ColumnType::Enum {
                    name: enum_name,
                    variants: _,
                } => Some(InputValue::new(
                    column_name,
                    TypeRef::named(
                        active_enum_filter_input_builder.type_name_from_iden(&enum_name),
                    ),
                )),
                // FIXME: cidr, inet, mac type
                ColumnType::Cidr | ColumnType::Inet | ColumnType::MacAddr => Some(InputValue::new(
                    column_name,
                    TypeRef::named(&self.context.filter_input.text_type),
                )),
                // FIXME: support array types
                // ColumnType::Array(_) => {}
                // FIXME: support custom types
                // ColumnType::Custom(iden) => {}
                _ => None,
            };

            match field {
                Some(field) => object.field(field),
                None => object,
            }
        });

        // FIXME: 'and' & 'or' should be configurable
        object
            .field(InputValue::new("and", TypeRef::named_nn_list(&name)))
            .field(InputValue::new("or", TypeRef::named_nn_list(&name)))
    }

    pub fn string_filter(&self) -> InputObject {
        InputObject::new(&self.context.filter_input.string_type)
            .field(InputValue::new("eq", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("ne", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("gt", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("gte", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("lt", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("lte", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new(
                "is_in",
                TypeRef::named_nn_list(TypeRef::STRING),
            ))
            .field(InputValue::new(
                "is_not_in",
                TypeRef::named_nn_list(TypeRef::STRING),
            ))
            .field(InputValue::new("is_null", TypeRef::named(TypeRef::BOOLEAN)))
            .field(InputValue::new("contains", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new(
                "starts_with",
                TypeRef::named(TypeRef::STRING),
            ))
            .field(InputValue::new(
                "ends_with",
                TypeRef::named(TypeRef::STRING),
            ))
            .field(InputValue::new("like", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("not_like", TypeRef::named(TypeRef::STRING)))
    }

    pub fn text_filter(&self) -> InputObject {
        InputObject::new(&self.context.filter_input.text_type)
            .field(InputValue::new("eq", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("ne", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("gt", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("gte", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("lt", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("lte", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new(
                "is_in",
                TypeRef::named_nn_list(TypeRef::STRING),
            ))
            .field(InputValue::new(
                "is_not_in",
                TypeRef::named_nn_list(TypeRef::STRING),
            ))
            .field(InputValue::new("is_null", TypeRef::named(TypeRef::BOOLEAN)))
    }

    pub fn integer_filter(&self) -> InputObject {
        InputObject::new(&self.context.filter_input.integer_type)
            .field(InputValue::new("eq", TypeRef::named(TypeRef::INT)))
            .field(InputValue::new("ne", TypeRef::named(TypeRef::INT)))
            .field(InputValue::new("gt", TypeRef::named(TypeRef::INT)))
            .field(InputValue::new("gte", TypeRef::named(TypeRef::INT)))
            .field(InputValue::new("lt", TypeRef::named(TypeRef::INT)))
            .field(InputValue::new("lte", TypeRef::named(TypeRef::INT)))
            .field(InputValue::new(
                "is_in",
                TypeRef::named_nn_list(TypeRef::INT),
            ))
            .field(InputValue::new(
                "is_not_in",
                TypeRef::named_nn_list(TypeRef::INT),
            ))
            .field(InputValue::new("is_null", TypeRef::named(TypeRef::BOOLEAN)))
    }

    pub fn float_filter(&self) -> InputObject {
        InputObject::new(&self.context.filter_input.float_type)
            .field(InputValue::new("eq", TypeRef::named(TypeRef::FLOAT)))
            .field(InputValue::new("ne", TypeRef::named(TypeRef::FLOAT)))
            .field(InputValue::new("gt", TypeRef::named(TypeRef::FLOAT)))
            .field(InputValue::new("gte", TypeRef::named(TypeRef::FLOAT)))
            .field(InputValue::new("lt", TypeRef::named(TypeRef::FLOAT)))
            .field(InputValue::new("lte", TypeRef::named(TypeRef::FLOAT)))
            .field(InputValue::new(
                "is_in",
                TypeRef::named_nn_list(TypeRef::FLOAT),
            ))
            .field(InputValue::new(
                "is_not_in",
                TypeRef::named_nn_list(TypeRef::FLOAT),
            ))
            .field(InputValue::new("is_null", TypeRef::named(TypeRef::BOOLEAN)))
    }

    pub fn boolean_filter(&self) -> InputObject {
        InputObject::new(&self.context.filter_input.boolean_type)
            .field(InputValue::new("eq", TypeRef::named(TypeRef::BOOLEAN)))
            .field(InputValue::new("ne", TypeRef::named(TypeRef::BOOLEAN)))
            .field(InputValue::new("gt", TypeRef::named(TypeRef::BOOLEAN)))
            .field(InputValue::new("gte", TypeRef::named(TypeRef::BOOLEAN)))
            .field(InputValue::new("lt", TypeRef::named(TypeRef::BOOLEAN)))
            .field(InputValue::new("lte", TypeRef::named(TypeRef::BOOLEAN)))
            .field(InputValue::new(
                "is_in",
                TypeRef::named_nn_list(TypeRef::BOOLEAN),
            ))
            .field(InputValue::new(
                "is_not_in",
                TypeRef::named_nn_list(TypeRef::BOOLEAN),
            ))
            .field(InputValue::new("is_null", TypeRef::named(TypeRef::BOOLEAN)))
    }

    pub fn id_filter(&self) -> InputObject {
        InputObject::new(&self.context.filter_input.id_type)
            .field(InputValue::new("eq", TypeRef::named(TypeRef::ID)))
            .field(InputValue::new("ne", TypeRef::named(TypeRef::ID)))
            .field(InputValue::new("gt", TypeRef::named(TypeRef::ID)))
            .field(InputValue::new("gte", TypeRef::named(TypeRef::ID)))
            .field(InputValue::new("lt", TypeRef::named(TypeRef::ID)))
            .field(InputValue::new("lte", TypeRef::named(TypeRef::ID)))
            .field(InputValue::new(
                "is_in",
                TypeRef::named_nn_list(TypeRef::ID),
            ))
            .field(InputValue::new(
                "is_not_in",
                TypeRef::named_nn_list(TypeRef::ID),
            ))
            .field(InputValue::new("is_null", TypeRef::named(TypeRef::BOOLEAN)))
    }
}

pub fn prepare_string_condition<T>(
    filter: &ObjectAccessor,
    column: T,
    condition: Condition,
) -> Condition
where
    T: ColumnTrait,
{
    let condition = match filter.get("eq") {
        Some(data) => {
            let data = data.string().unwrap();

            condition.add(column.eq(data))
        }
        None => condition,
    };

    let condition = match filter.get("ne") {
        Some(data) => {
            let data = data.string().unwrap();

            condition.add(column.ne(data))
        }
        None => condition,
    };

    let condition = match filter.get("gt") {
        Some(data) => {
            let data = data.string().unwrap();

            condition.add(column.gt(data))
        }
        None => condition,
    };

    let condition = match filter.get("gte") {
        Some(data) => {
            let data = data.string().unwrap();

            condition.add(column.gte(data))
        }
        None => condition,
    };

    let condition = match filter.get("lt") {
        Some(data) => {
            let data = data.string().unwrap();

            condition.add(column.lt(data))
        }
        None => condition,
    };

    let condition = match filter.get("lte") {
        Some(data) => {
            let data = data.string().unwrap();

            condition.add(column.lte(data))
        }
        None => condition,
    };

    let condition = match filter.get("is_in") {
        Some(data) => {
            let data = data.list().unwrap();
            let data: Vec<String> = data
                .iter()
                .map(|v| v.string().unwrap().to_string())
                .collect();

            condition.add(column.is_in(data))
        }
        None => condition,
    };

    let condition = match filter.get("is_not_in") {
        Some(data) => {
            let data = data.list().unwrap();
            let data: Vec<String> = data
                .iter()
                .map(|v| v.string().unwrap().to_string())
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

    let condition = match filter.get("contains") {
        Some(data) => {
            let data = data.string().unwrap();

            condition.add(column.contains(data))
        }
        None => condition,
    };

    let condition = match filter.get("starts_with") {
        Some(data) => {
            let data = data.string().unwrap();

            condition.add(column.starts_with(data))
        }
        None => condition,
    };

    let condition = match filter.get("ends_with") {
        Some(data) => {
            let data = data.string().unwrap();

            condition.add(column.ends_with(data))
        }
        None => condition,
    };

    let condition = match filter.get("like") {
        Some(data) => {
            let data = data.string().unwrap();

            condition.add(column.like(data))
        }
        None => condition,
    };

    let condition = match filter.get("not_like") {
        Some(data) => {
            let data = data.string().unwrap();

            condition.add(column.not_like(data))
        }
        None => condition,
    };

    condition
}

pub fn prepare_text_condition<T>(
    filter: &ObjectAccessor,
    column: T,
    condition: Condition,
) -> Condition
where
    T: ColumnTrait,
{
    let condition = match filter.get("eq") {
        Some(data) => {
            let data = data.string().unwrap();

            condition.add(column.eq(data))
        }
        None => condition,
    };

    let condition = match filter.get("ne") {
        Some(data) => {
            let data = data.string().unwrap();

            condition.add(column.ne(data))
        }
        None => condition,
    };

    let condition = match filter.get("gt") {
        Some(data) => {
            let data = data.string().unwrap();

            condition.add(column.gt(data))
        }
        None => condition,
    };

    let condition = match filter.get("gte") {
        Some(data) => {
            let data = data.string().unwrap();

            condition.add(column.gte(data))
        }
        None => condition,
    };

    let condition = match filter.get("lt") {
        Some(data) => {
            let data = data.string().unwrap();

            condition.add(column.lt(data))
        }
        None => condition,
    };

    let condition = match filter.get("lte") {
        Some(data) => {
            let data = data.string().unwrap();

            condition.add(column.lte(data))
        }
        None => condition,
    };

    let condition = match filter.get("is_in") {
        Some(data) => {
            let data = data.list().unwrap();
            let data: Vec<String> = data
                .iter()
                .map(|v| v.string().unwrap().to_string())
                .collect();

            condition.add(column.is_in(data))
        }
        None => condition,
    };

    let condition = match filter.get("is_not_in") {
        Some(data) => {
            let data = data.list().unwrap();
            let data: Vec<String> = data
                .iter()
                .map(|v| v.string().unwrap().to_string())
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

pub fn prepare_parsed_condition<T, Y, F>(
    filter: &ObjectAccessor,
    column: T,
    parse: F,
    condition: Condition,
) -> Condition
where
    T: ColumnTrait,
    Y: Into<Value>,
    F: Fn(String) -> Y,
{
    let condition = match filter.get("eq") {
        Some(data) => {
            let data = data.string().unwrap().to_string();
            let data = parse(data);

            condition.add(column.eq(data))
        }
        None => condition,
    };

    let condition = match filter.get("ne") {
        Some(data) => {
            let data = data.string().unwrap().to_string();
            let data = parse(data);

            condition.add(column.ne(data))
        }
        None => condition,
    };

    let condition = match filter.get("gt") {
        Some(data) => {
            let data = data.string().unwrap().to_string();
            let data = parse(data);

            condition.add(column.gt(data))
        }
        None => condition,
    };

    let condition = match filter.get("gte") {
        Some(data) => {
            let data = data.string().unwrap().to_string();
            let data = parse(data);

            condition.add(column.gte(data))
        }
        None => condition,
    };

    let condition = match filter.get("lt") {
        Some(data) => {
            let data = data.string().unwrap().to_string();
            let data = parse(data);

            condition.add(column.lt(data))
        }
        None => condition,
    };

    let condition = match filter.get("lte") {
        Some(data) => {
            let data = data.string().unwrap().to_string();
            let data = parse(data);

            condition.add(column.lte(data))
        }
        None => condition,
    };

    let condition = match filter.get("is_in") {
        Some(data) => {
            let data: Vec<_> = data
                .list()
                .unwrap()
                .iter()
                .map(|item| item.string().unwrap().to_string())
                .map(&parse)
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
                .map(|item| item.string().unwrap().to_string())
                .map(&parse)
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

pub fn prepare_integer_condition<T>(
    filter: &ObjectAccessor,
    column: T,
    condition: Condition,
) -> Condition
where
    T: ColumnTrait,
{
    let condition = match filter.get("eq") {
        Some(data) => {
            let data = data.i64().unwrap();

            condition.add(column.eq(data))
        }
        None => condition,
    };

    let condition = match filter.get("ne") {
        Some(data) => {
            let data = data.i64().unwrap();

            condition.add(column.ne(data))
        }
        None => condition,
    };

    let condition = match filter.get("gt") {
        Some(data) => {
            let data = data.i64().unwrap();

            condition.add(column.gt(data))
        }
        None => condition,
    };

    let condition = match filter.get("gte") {
        Some(data) => {
            let data = data.i64().unwrap();

            condition.add(column.gte(data))
        }
        None => condition,
    };

    let condition = match filter.get("lt") {
        Some(data) => {
            let data = data.i64().unwrap();

            condition.add(column.lt(data))
        }
        None => condition,
    };

    let condition = match filter.get("lte") {
        Some(data) => {
            let data = data.i64().unwrap();

            condition.add(column.lte(data))
        }
        None => condition,
    };

    let condition = match filter.get("is_in") {
        Some(data) => {
            let data: Vec<i64> = data
                .list()
                .unwrap()
                .iter()
                .map(|item| item.i64().unwrap())
                .collect();

            condition.add(column.is_in(data))
        }
        None => condition,
    };

    let condition = match filter.get("is_not_in") {
        Some(data) => {
            let data: Vec<i64> = data
                .list()
                .unwrap()
                .iter()
                .map(|item| item.i64().unwrap())
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

pub fn prepare_unsigned_condition<T>(
    filter: &ObjectAccessor,
    column: T,
    condition: Condition,
) -> Condition
where
    T: ColumnTrait,
{
    let condition = match filter.get("eq") {
        Some(data) => {
            let data = data.u64().unwrap();

            condition.add(column.eq(data))
        }
        None => condition,
    };

    let condition = match filter.get("ne") {
        Some(data) => {
            let data = data.u64().unwrap();

            condition.add(column.ne(data))
        }
        None => condition,
    };

    let condition = match filter.get("gt") {
        Some(data) => {
            let data = data.u64().unwrap();

            condition.add(column.gt(data))
        }
        None => condition,
    };

    let condition = match filter.get("gte") {
        Some(data) => {
            let data = data.u64().unwrap();

            condition.add(column.gte(data))
        }
        None => condition,
    };

    let condition = match filter.get("lt") {
        Some(data) => {
            let data = data.u64().unwrap();

            condition.add(column.lt(data))
        }
        None => condition,
    };

    let condition = match filter.get("lte") {
        Some(data) => {
            let data = data.u64().unwrap();

            condition.add(column.lte(data))
        }
        None => condition,
    };

    let condition = match filter.get("is_in") {
        Some(data) => {
            let data: Vec<u64> = data
                .list()
                .unwrap()
                .iter()
                .map(|item| item.u64().unwrap())
                .collect();

            condition.add(column.is_in(data))
        }
        None => condition,
    };

    let condition = match filter.get("is_not_in") {
        Some(data) => {
            let data: Vec<u64> = data
                .list()
                .unwrap()
                .iter()
                .map(|item| item.u64().unwrap())
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

pub fn prepare_float_condition<T>(
    filter: &ObjectAccessor,
    column: T,
    condition: Condition,
) -> Condition
where
    T: ColumnTrait,
{
    let condition = match filter.get("eq") {
        Some(data) => {
            let data = data.f64().unwrap();

            condition.add(column.eq(data))
        }
        None => condition,
    };

    let condition = match filter.get("ne") {
        Some(data) => {
            let data = data.f64().unwrap();

            condition.add(column.ne(data))
        }
        None => condition,
    };

    let condition = match filter.get("gt") {
        Some(data) => {
            let data = data.f64().unwrap();

            condition.add(column.gt(data))
        }
        None => condition,
    };

    let condition = match filter.get("gte") {
        Some(data) => {
            let data = data.f64().unwrap();

            condition.add(column.gte(data))
        }
        None => condition,
    };

    let condition = match filter.get("lt") {
        Some(data) => {
            let data = data.f64().unwrap();

            condition.add(column.lt(data))
        }
        None => condition,
    };

    let condition = match filter.get("lte") {
        Some(data) => {
            let data = data.f64().unwrap();

            condition.add(column.lte(data))
        }
        None => condition,
    };

    let condition = match filter.get("is_in") {
        Some(data) => {
            let data: Vec<f64> = data
                .list()
                .unwrap()
                .iter()
                .map(|item| item.f64().unwrap())
                .collect();

            condition.add(column.is_in(data))
        }
        None => condition,
    };

    let condition = match filter.get("is_not_in") {
        Some(data) => {
            let data: Vec<f64> = data
                .list()
                .unwrap()
                .iter()
                .map(|item| item.f64().unwrap())
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

pub fn prepare_boolean_condition<T>(
    filter: &ObjectAccessor,
    column: T,
    condition: Condition,
) -> Condition
where
    T: ColumnTrait,
{
    let condition = match filter.get("eq") {
        Some(data) => {
            let data = data.boolean().unwrap();

            condition.add(column.eq(data))
        }
        None => condition,
    };

    let condition = match filter.get("ne") {
        Some(data) => {
            let data = data.boolean().unwrap();

            condition.add(column.ne(data))
        }
        None => condition,
    };

    let condition = match filter.get("gt") {
        Some(data) => {
            let data = data.boolean().unwrap();

            condition.add(column.gt(data))
        }
        None => condition,
    };

    let condition = match filter.get("gte") {
        Some(data) => {
            let data = data.boolean().unwrap();

            condition.add(column.gte(data))
        }
        None => condition,
    };

    let condition = match filter.get("lt") {
        Some(data) => {
            let data = data.boolean().unwrap();

            condition.add(column.lt(data))
        }
        None => condition,
    };

    let condition = match filter.get("lte") {
        Some(data) => {
            let data = data.boolean().unwrap();

            condition.add(column.lte(data))
        }
        None => condition,
    };

    let condition = match filter.get("is_in") {
        Some(data) => {
            let data: Vec<bool> = data
                .list()
                .unwrap()
                .iter()
                .map(|item| item.boolean().unwrap())
                .collect();

            condition.add(column.is_in(data))
        }
        None => condition,
    };

    let condition = match filter.get("is_not_in") {
        Some(data) => {
            let data: Vec<bool> = data
                .list()
                .unwrap()
                .iter()
                .map(|item| item.boolean().unwrap())
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
