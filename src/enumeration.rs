use async_graphql::dynamic::Enum;
use heck::ToUpperCamelCase;
use sea_orm::{ActiveEnum, Value};

pub fn enumeration_map<A: ActiveEnum>() -> Enum {
    let name = format!("{}Enum", A::name().to_string().to_upper_camel_case());
    A::values()
        .into_iter()
        .fold(Enum::new(name), |enumeration, variant| {
            let variant: Value = variant.into();
            let variant: String = variant.to_string();

            enumeration.item(variant.to_upper_camel_case().to_ascii_uppercase())
        })
}