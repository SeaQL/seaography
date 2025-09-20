use crate::{
    BuilderContext, Connection, ConnectionObjectBuilder, EntityObjectBuilder, PageInfo,
    PageInfoObjectBuilder, PaginationInfo, PaginationInfoObjectBuilder,
};
use async_graphql::dynamic::{FieldValue, Object, TypeRef};
#[cfg(feature = "macros")]
pub use seaography_macros::{ConvertOutput, CustomOutputType};

pub trait CustomOutputType {
    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef;
    fn gql_field_value(self, ctx: &'static BuilderContext) -> Option<FieldValue<'static>>;
}

pub trait CustomOutputObject {
    fn basic_object(context: &'static BuilderContext) -> Object;
}

pub trait ConvertOutput {
    fn convert_output(
        value: &sea_orm::sea_query::Value,
    ) -> async_graphql::Result<Option<FieldValue<'static>>>;
}

impl CustomOutputType for PageInfo {
    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        let page_info_object_builder = PageInfoObjectBuilder { context: ctx };
        TypeRef::named_nn(page_info_object_builder.type_name())
    }

    fn gql_field_value(self, _ctx: &'static BuilderContext) -> Option<FieldValue<'static>> {
        Some(FieldValue::owned_any(self))
    }
}

impl CustomOutputType for PaginationInfo {
    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        let page_info_object_builder = PaginationInfoObjectBuilder { context: ctx };
        TypeRef::named_nn(page_info_object_builder.type_name())
    }

    fn gql_field_value(self, _ctx: &'static BuilderContext) -> Option<FieldValue<'static>> {
        Some(FieldValue::owned_any(self))
    }
}

impl<T> CustomOutputType for Option<T>
where
    T: CustomOutputType,
{
    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        match T::gql_output_type_ref(ctx) {
            TypeRef::Named(n) => TypeRef::Named(n),
            TypeRef::List(t) => TypeRef::List(t),
            TypeRef::NonNull(t) => TypeRef::clone(&*t),
        }
    }

    fn gql_field_value(self, ctx: &'static BuilderContext) -> Option<FieldValue<'static>> {
        match self {
            Some(value) => T::gql_field_value(value, ctx),
            None => None,
        }
    }
}

impl<T> CustomOutputType for Vec<T>
where
    T: CustomOutputType,
{
    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        TypeRef::NonNull(Box::new(TypeRef::List(Box::new(T::gql_output_type_ref(
            ctx,
        )))))
    }

    fn gql_field_value(self, ctx: &'static BuilderContext) -> Option<FieldValue<'static>> {
        let mut items: Vec<FieldValue<'static>> = Vec::new();
        // TODO: Figure out what the right behaviour is here in the case where
        // T::gql_field_value returns None. For now, we just skip such values.
        for v in self.into_iter() {
            if let Some(item) = T::gql_field_value(v, ctx) {
                items.push(item);
            }
        }
        Some(FieldValue::list(items))
    }
}

impl<M> CustomOutputType for Box<M>
where
    M: CustomOutputType,
{
    fn gql_output_type_ref(context: &'static BuilderContext) -> TypeRef {
        M::gql_output_type_ref(context)
    }

    fn gql_field_value(self, ctx: &'static BuilderContext) -> Option<FieldValue<'static>> {
        M::gql_field_value(*self, ctx)
    }
}

impl<T> CustomOutputType for Connection<T>
where
    T: sea_orm::EntityTrait,
    <T as sea_orm::EntityTrait>::Model: Sync + CustomOutputType,
{
    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        let entity_object_builder = EntityObjectBuilder { context: ctx };
        let object_name: String = entity_object_builder.type_name::<T>();

        TypeRef::named_nn(ConnectionObjectBuilder { context: ctx }.type_name(&object_name))
    }

    fn gql_field_value(self, _ctx: &'static BuilderContext) -> Option<FieldValue<'static>> {
        Some(FieldValue::owned_any(self))
    }
}
