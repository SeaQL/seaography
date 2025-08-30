use seaography::macros::CustomOutput;

#[derive(Clone, CustomOutput)]
pub struct PurchaseOrder {
    pub po_number: String,
    pub lineitems: Vec<Lineitem>,
}

#[derive(Clone, CustomOutput)]
pub struct Lineitem {
    pub product: String,
    pub quantity: f64,
    pub size: Option<i32>,
}

/*

impl CustomOutput for PurchaseOrder {
    fn basic_object(ctx: &'static BuilderContext) -> Object {
        Object::new("PurchaseOrderBasic")
            .field(Field::new(
                "po_number",
                String::gql_output_type_ref(ctx),
                |ctx| {
                    let object = match ctx.parent_value.try_downcast_ref::<Self>() {
                        Ok(object) => object,
                        Err(err) => {
                            return FieldFuture::new(async move { Err::<Option<()>, _>(err) })
                        }
                    };
                    let field_value = String::gql_field_value(object.po_number.clone());

                    FieldFuture::new(async move { Ok(field_value) })
                },
            ))
            .field(Field::new(
                "lineitems",
                Vec::<Lineitem>::gql_output_type_ref(ctx),
                |ctx| {
                    let object = match ctx.parent_value.try_downcast_ref::<Self>() {
                        Ok(object) => object,
                        Err(err) => {
                            return FieldFuture::new(async move { Err::<Option<()>, _>(err) })
                        }
                    };
                    let field_value = Vec::<Lineitem>::gql_field_value(object.lineitems.clone());

                    FieldFuture::new(async move { Ok(field_value) })
                },
            ))
    }
}

impl CustomOutput for Lineitem {
    fn basic_object(ctx: &'static BuilderContext) -> Object {
        Object::new("LineitemBasic")
            .field(Field::new(
                "product",
                String::gql_output_type_ref(ctx),
                |ctx| {
                    let object = match ctx.parent_value.try_downcast_ref::<Self>() {
                        Ok(object) => object,
                        Err(err) => {
                            return FieldFuture::new(async move { Err::<Option<()>, _>(err) })
                        }
                    };
                    let field_value = String::gql_field_value(object.product.clone());

                    FieldFuture::new(async move { Ok(field_value) })
                },
            ))
            .field(Field::new(
                "quantity",
                f64::gql_output_type_ref(ctx),
                |ctx| {
                    let object = match ctx.parent_value.try_downcast_ref::<Self>() {
                        Ok(object) => object,
                        Err(err) => {
                            return FieldFuture::new(async move { Err::<Option<()>, _>(err) })
                        }
                    };
                    let field_value = f64::gql_field_value(object.quantity.clone());

                    FieldFuture::new(async move { Ok(field_value) })
                },
            ))
            .field(Field::new(
                "size",
                Option::<i32>::gql_output_type_ref(ctx),
                |ctx| {
                    let object = match ctx.parent_value.try_downcast_ref::<Self>() {
                        Ok(object) => object,
                        Err(err) => {
                            return FieldFuture::new(async move { Err::<Option<()>, _>(err) })
                        }
                    };
                    let field_value = Option::<i32>::gql_field_value(object.size.clone());

                    FieldFuture::new(async move { Ok(field_value) })
                },
            ))
    }
}

impl GqlOutputModelType for PurchaseOrder {
    fn gql_output_type_ref(_: &'static BuilderContext) -> TypeRef {
        TypeRef::named_nn("PurchaseOrderBasic")
    }
}

impl GqlOutputModelType for Lineitem {
    fn gql_output_type_ref(_: &'static BuilderContext) -> TypeRef {
        TypeRef::named_nn("LineitemBasic")
    }
}

*/
