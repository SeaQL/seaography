use async_graphql::dynamic::Enum;
use heck::ToUpperCamelCase;
use sea_orm::{ActiveEnum, Value};

use crate::BuilderContext;

#[derive(Clone, Debug)]
pub struct ActiveEnumConfig {
    pub type_name: String,
}

impl std::default::Default for ActiveEnumConfig {
    fn default() -> Self {
        ActiveEnumConfig {
            type_name: "Enum".into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ActiveEnumBuilder {
    pub context: &'static BuilderContext,
}

impl ActiveEnumBuilder {
    // FIXME: use context naming function
    pub fn type_name<A: ActiveEnum>(&self) -> String {
        format!(
            "{}{}",
            A::name().to_string().to_upper_camel_case(),
            self.context.active_enum.type_name
        )
    }

    // FIXME: use context naming function
    pub fn variant_name(&self, variant: &String) -> String {
        variant.to_upper_camel_case().to_ascii_uppercase()
    }

    pub fn enumeration<A: ActiveEnum>(&self) -> Enum {
        let name = self.type_name::<A>();

        A::values()
            .into_iter()
            .fold(Enum::new(name), |enumeration, variant| {
                let variant: Value = variant.into();
                let variant: String = variant.to_string();
                enumeration.item(self.variant_name(&variant))
            })
    }
}
