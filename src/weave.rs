//! # Weave Core - GraphQL Type System Support
//!
//! This crate provides traits and macros for implementing GraphQL type system support
//! for both scalar types and custom model types. It includes support for nullable
//! (Option<T>) variants of all types.
//!
//! ## Core Traits
//!
//! - `GqlFieldValue<'a>`: Convert values to GraphQL field values
//! - `GqlTypeRef`: Generate GraphQL type references
//!
//! ## Scalar Type Support
//!
//! All standard scalar types are supported out of the box:
//! - `bool` and `Option<bool>`
//! - Integer types: `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64` and their `Option<>` variants
//! - Float types: `f32`, `f64` and their `Option<>` variants
//! - `String` and `Option<String>`
//!
//! ## Custom Model Type Support
//!
//! Use the provided macros to implement GraphQL support for your custom types:
//!
//! ```rust,ignore
//! use seaography::impl_gql;
//!
//! // Basic implementation for a model
//! impl_gql!(MyModel);
//!
//! // If you need Option<MyModel> support, add this separately:
//! // impl_gql_field_value_option!(MyModel);
//! // impl_gql_type_ref_option!(MyModel);
//! ```
//!
//! ## Orphan Rule Compliance
//!
//! Due to Rust's orphan rules, Option<T> implementations for external types
//! must be implemented separately using the `*_option` macros. This prevents
//! compilation errors when working with types from other crates.

use async_graphql::{
    dynamic::{FieldValue, TypeRef},
    Value,
};

// Add support for seaography Connection types - import is handled in impl blocks

/// A trait for values that can be used as GraphQL field values.
pub trait GqlFieldValue<'a> {
    fn gql_field_value(self) -> FieldValue<'a>;
}

macro_rules! gql_scalar_field_value {
    ($type:ty) => {
        impl<'a> GqlFieldValue<'a> for $type {
            fn gql_field_value(self) -> FieldValue<'a> {
                FieldValue::value(Value::from(self))
            }
        }
    };
}

gql_scalar_field_value!(bool);
gql_scalar_field_value!(i8);
gql_scalar_field_value!(i16);
gql_scalar_field_value!(i32);
gql_scalar_field_value!(i64);
gql_scalar_field_value!(u8);
gql_scalar_field_value!(u16);
gql_scalar_field_value!(u32);
gql_scalar_field_value!(u64);
gql_scalar_field_value!(f32);
gql_scalar_field_value!(f64);
gql_scalar_field_value!(String);

// Implement GqlFieldValue for Option<> of scalar types
macro_rules! gql_scalar_field_value_option {
    ($type:ty) => {
        impl<'a> GqlFieldValue<'a> for Option<$type> {
            fn gql_field_value(self) -> FieldValue<'a> {
                match self {
                    Some(value) => FieldValue::value(Value::from(value)),
                    None => FieldValue::value(Value::Null),
                }
            }
        }
    };
}

gql_scalar_field_value_option!(bool);
gql_scalar_field_value_option!(i8);
gql_scalar_field_value_option!(i16);
gql_scalar_field_value_option!(i32);
gql_scalar_field_value_option!(i64);
gql_scalar_field_value_option!(u8);
gql_scalar_field_value_option!(u16);
gql_scalar_field_value_option!(u32);
gql_scalar_field_value_option!(u64);
gql_scalar_field_value_option!(f32);
gql_scalar_field_value_option!(f64);
gql_scalar_field_value_option!(String);

/// A trait for types that can be used as GraphQL types.
pub trait GqlTypeRef {
    fn gql_type_ref() -> TypeRef;
}

impl<T> GqlTypeRef for async_graphql::Result<T>
where
    T: GqlTypeRef,
{
    fn gql_type_ref() -> TypeRef {
        T::gql_type_ref()
    }
}

macro_rules! gql_scalar_type_ref {
    ($type:ty, $e:expr) => {
        impl GqlTypeRef for Option<$type> {
            fn gql_type_ref() -> TypeRef {
                TypeRef::named($e)
            }
        }

        impl GqlTypeRef for $type {
            fn gql_type_ref() -> TypeRef {
                TypeRef::named_nn($e)
            }
        }
    };
}

gql_scalar_type_ref!(bool, TypeRef::BOOLEAN);
gql_scalar_type_ref!(i8, TypeRef::INT);
gql_scalar_type_ref!(i16, TypeRef::INT);
gql_scalar_type_ref!(i32, TypeRef::INT);
gql_scalar_type_ref!(i64, TypeRef::INT);
gql_scalar_type_ref!(u8, TypeRef::INT);
gql_scalar_type_ref!(u16, TypeRef::INT);
gql_scalar_type_ref!(u32, TypeRef::INT);
gql_scalar_type_ref!(u64, TypeRef::INT);
gql_scalar_type_ref!(f32, TypeRef::FLOAT);
gql_scalar_type_ref!(f64, TypeRef::FLOAT);
gql_scalar_type_ref!(String, TypeRef::STRING);

#[macro_export]
macro_rules! impl_gql_field_value {
    ($model:ty) => {
        impl<'a> seaography::GqlFieldValue<'a> for $model {
            fn gql_field_value(self) -> async_graphql::dynamic::FieldValue<'a> {
                async_graphql::dynamic::FieldValue::owned_any(self)
            }
        }
    };
}

/// Implements `GqlFieldValue` for `Option<T>` where `T` is a custom model type.
/// This is a separate macro to avoid orphan rule issues.
///
/// # Example
/// ```rust,ignore
/// use seaography::impl_gql_field_value_option;
///
/// // For a custom model type
/// impl_gql_field_value_option!(MyModel);
/// ```
#[macro_export]
macro_rules! impl_gql_field_value_option {
    ($model:ty) => {
        impl<'a> seaography::GqlFieldValue<'a> for Option<$model> {
            fn gql_field_value(self) -> async_graphql::dynamic::FieldValue<'a> {
                match self {
                    Some(value) => async_graphql::dynamic::FieldValue::owned_any(value),
                    None => async_graphql::dynamic::FieldValue::value(async_graphql::Value::Null),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_gql_type_ref {
    ($model:ty) => {
        seaography::impl_gql_type_ref!($model where context: &CONTEXT);
    };
    ($model:ty where context: $context:expr) => {
        impl seaography::GqlTypeRef for $model {
            fn gql_type_ref() -> async_graphql::dynamic::TypeRef {
                async_graphql::dynamic::TypeRef::named_nn(
                    seaography::EntityObjectBuilder { context: $context }.type_name::<<Self as sea_orm::ModelTrait>::Entity>(),
                )
            }
        }
    };
}

/// Implements `GqlTypeRef` for `Option<T>` where `T` is a custom model type.
/// This creates a nullable GraphQL type reference.
///
/// # Example
/// ```rust,ignore
/// use seaography::impl_gql_type_ref_option;
///
/// // For a custom model type with default context
/// impl_gql_type_ref_option!(MyModel);
///
/// // For a custom model type with specific context
/// impl_gql_type_ref_option!(MyModel where context: &my_context);
/// ```
#[macro_export]
macro_rules! impl_gql_type_ref_option {
    ($model:ty) => {
        seaography::impl_gql_type_ref_option!($model where context: &CONTEXT);
    };
    ($model:ty where context: $context:expr) => {
        impl seaography::GqlTypeRef for Option<$model> {
            fn gql_type_ref() -> async_graphql::dynamic::TypeRef {
                async_graphql::dynamic::TypeRef::named(
                    crate::EntityObjectBuilder { context: $context }.type_name::<<$model as sea_orm::ModelTrait>::Entity>(),
                )
            }
        }
    };
}

/// Implements both `GqlFieldValue` and `GqlTypeRef` for a model type.
/// This is a convenience macro that combines the two implementations.
///
/// Note: This does NOT automatically implement the Option<> versions to avoid orphan rule issues.
/// If you need Option<> support, use the separate macros:
/// - `impl_gql_field_value_option!`
/// - `impl_gql_type_ref_option!`
///
/// # Example
/// ```rust,ignore
/// use seaography::impl_gql;
///
/// // Basic usage with default context
/// impl_gql!(MyModel);
///
/// // With specific context
/// impl_gql!(MyModel where context: &my_context);
///
/// // If you also need Option<> support, add these:
/// // impl_gql_field_value_option!(MyModel);
/// // impl_gql_type_ref_option!(MyModel);
/// ```
#[macro_export]
macro_rules! impl_gql {
    ($model:ty) => {
        seaography::impl_gql!($model where context: &CONTEXT);
    };
    ($model:ty where context: $context:expr) => {
        seaography::impl_gql_field_value!($model);
        seaography::impl_gql_type_ref!($model where context: $context);
    };
}

// Connection support for seaography
impl<'a, T> GqlFieldValue<'a> for crate::Connection<T>
where
    T: sea_orm::EntityTrait + 'static,
    T::Model: Sync,
{
    fn gql_field_value(self) -> FieldValue<'a> {
        FieldValue::owned_any(self)
    }
}

impl GqlTypeRef for Option<crate::PaginationInput> {
    fn gql_type_ref() -> TypeRef {
        TypeRef::named("PaginationInput".to_string())
    }
}

impl<T> GqlTypeRef for crate::Connection<T>
where
    T: sea_orm::EntityTrait + 'static,
    T::Model: Sync,
{
    // FIXME: pretty sure this doenst work
    fn gql_type_ref() -> TypeRef {
        // Generate the connection type name based on the entity name
        let entity_name = std::any::type_name::<T>()
            .split("::")
            .last()
            .unwrap_or("Unknown")
            .replace("Entity", "");
        TypeRef::named_nn(format!("{}Connection", entity_name))
        //TypeRef::named_nn("Connection")
    }
}

/// Implements both `GqlFieldValue` and `GqlTypeRef` for seaography Connection types.
/// This is a convenience macro for Connection<T> where T is a SeaORM entity.
///
/// # Example
/// ```rust,ignore
/// use seaography::impl_gql_connection;
/// use my_entities::modules;
///
/// // Implement GraphQL support for Connection<modules::Entity>
/// impl_gql_connection!(modules::Entity);
/// ```
#[macro_export]
macro_rules! impl_gql_connection {
    ($entity:ty) => {
        // Connection types are already implemented generically above
        // This macro is provided for consistency and future extensibility
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_option_field_values() {
        // Test that the methods exist and can be called without errors
        // We can't easily test the internal structure of FieldValue without
        // accessing private implementation details

        // Test Option<bool>
        let some_bool: Option<bool> = Some(true);
        let none_bool: Option<bool> = None;
        let _field_value1 = some_bool.gql_field_value();
        let _field_value2 = none_bool.gql_field_value();

        // Test Option<i32>
        let some_int: Option<i32> = Some(42);
        let none_int: Option<i32> = None;
        let _field_value3 = some_int.gql_field_value();
        let _field_value4 = none_int.gql_field_value();

        // Test Option<String>
        let some_string: Option<String> = Some("hello".to_string());
        let none_string: Option<String> = None;
        let _field_value5 = some_string.gql_field_value();
        let _field_value6 = none_string.gql_field_value();

        // Test Option<f64>
        let some_float: Option<f64> = Some(3.14);
        let none_float: Option<f64> = None;
        let _field_value7 = some_float.gql_field_value();
        let _field_value8 = none_float.gql_field_value();
    }

    #[test]
    fn test_scalar_type_refs() {
        // Test non-nullable types
        assert_eq!(bool::gql_type_ref(), TypeRef::named_nn(TypeRef::BOOLEAN));
        assert_eq!(i32::gql_type_ref(), TypeRef::named_nn(TypeRef::INT));
        assert_eq!(f64::gql_type_ref(), TypeRef::named_nn(TypeRef::FLOAT));
        assert_eq!(String::gql_type_ref(), TypeRef::named_nn(TypeRef::STRING));

        // Test nullable types
        assert_eq!(
            Option::<bool>::gql_type_ref(),
            TypeRef::named(TypeRef::BOOLEAN)
        );
        assert_eq!(Option::<i32>::gql_type_ref(), TypeRef::named(TypeRef::INT));
        assert_eq!(
            Option::<f64>::gql_type_ref(),
            TypeRef::named(TypeRef::FLOAT)
        );
        assert_eq!(
            Option::<String>::gql_type_ref(),
            TypeRef::named(TypeRef::STRING)
        );
    }

    #[test]
    fn test_result_type_ref() {
        // Test that async_graphql::Result<T> uses T's type ref
        assert_eq!(
            async_graphql::Result::<bool>::gql_type_ref(),
            TypeRef::named_nn(TypeRef::BOOLEAN)
        );
        assert_eq!(
            async_graphql::Result::<Option<i32>>::gql_type_ref(),
            TypeRef::named(TypeRef::INT)
        );
    }
}