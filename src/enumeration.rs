use async_graphql::dynamic::{Enum, InputObject, InputValue, TypeRef};
use heck::ToUpperCamelCase;
use sea_orm::{ActiveEnum, Value};

pub fn active_enum_to_enum_type<A: ActiveEnum>() -> Enum {
    let name = format!("{}Enum", A::name().to_string().to_upper_camel_case());
    A::values()
        .into_iter()
        .fold(Enum::new(name), |enumeration, variant| {
            let variant: Value = variant.into();
            let variant: String = variant.to_string();

            enumeration.item(variant.to_upper_camel_case().to_ascii_uppercase())
        })
}

pub fn active_enum_to_enum_filter_input<A: ActiveEnum>() -> InputObject {
    let name = format!("{}EnumFilterInput", A::name().to_string().to_upper_camel_case());

    let enum_name = format!("{}Enum", A::name().to_string().to_upper_camel_case());

    InputObject::new(name)
        .field(InputValue::new("eq", TypeRef::named(&enum_name)))
        .field(InputValue::new("ne", TypeRef::named(&enum_name)))
        .field(InputValue::new("gt", TypeRef::named(&enum_name)))
        .field(InputValue::new("gte", TypeRef::named(&enum_name)))
        .field(InputValue::new("lt", TypeRef::named(&enum_name)))
        .field(InputValue::new("lte", TypeRef::named(&enum_name)))
        .field(InputValue::new(
            "is_in",
            TypeRef::named_nn_list(&enum_name),
        ))
        .field(InputValue::new(
            "is_not_in",
            TypeRef::named_nn_list(&enum_name),
        ))
        .field(InputValue::new("is_null", TypeRef::named(TypeRef::BOOLEAN)))
}
