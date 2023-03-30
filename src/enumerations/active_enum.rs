use async_graphql::dynamic::Enum;
use heck::ToUpperCamelCase;
use sea_orm::{ActiveEnum, DynIden, Value};

use crate::BuilderContext;

/// The configuration structure for ActiveEnumBuilder
pub struct ActiveEnumConfig {
    /// used to format enumeration name
    pub type_name: Box<dyn Fn(&str) -> String + Sync + Send>,
    /// used to format variant name
    pub variant_name: Box<dyn Fn(&str, &str) -> String + Sync + Send>,
}

impl std::default::Default for ActiveEnumConfig {
    fn default() -> Self {
        ActiveEnumConfig {
            type_name: Box::new(|name: &str| -> String {
                format!("{}Enum", name.to_upper_camel_case())
            }),
            variant_name: Box::new(|_enum_name: &str, variant: &str| -> String {
                variant.to_upper_camel_case().to_ascii_uppercase()
            }),
        }
    }
}

/// This builder is used to convert a SeaORM enumeration to GraphQL
pub struct ActiveEnumBuilder {
    pub context: &'static BuilderContext,
}

impl ActiveEnumBuilder {
    /// used to format SeaORM enumeration name to GraphQL enumeration name
    pub fn type_name<A: ActiveEnum>(&self) -> String {
        let name = A::name().to_string();
        self.context.active_enum.type_name.as_ref()(&name)
    }

    /// used to format enumeration Iden name to GraphQL enumeration name
    pub fn type_name_from_iden(&self, name: &DynIden) -> String {
        let name = name.to_string();
        self.context.active_enum.type_name.as_ref()(&name)
    }

    /// used to format SeaORM variant name to GraphQL variant name
    pub fn variant_name(&self, enum_name: &str, variant: &str) -> String {
        self.context.active_enum.variant_name.as_ref()(enum_name, variant)
    }

    /// used to convert SeaORM enumeration to GraphQL enumeration
    pub fn enumeration<A: ActiveEnum>(&self) -> Enum {
        let enum_name = self.type_name::<A>();

        A::values()
            .into_iter()
            .fold(Enum::new(&enum_name), |enumeration, variant| {
                let variant: Value = variant.into();
                let variant: String = variant.to_string();
                enumeration.item(self.variant_name(&enum_name, &variant))
            })
    }
}
