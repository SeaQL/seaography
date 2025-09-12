use seaography::CustomOutputType;

#[derive(Clone, CustomOutputType)]
pub struct PurchaseOrder {
    pub po_number: String,
    pub lineitems: Vec<Lineitem>,
}

#[derive(Clone, CustomOutputType)]
pub struct Lineitem {
    pub product: String,
    pub quantity: f64,
    pub size: Option<ProductSize>,
}

#[derive(Clone, CustomOutputType)]
pub struct ProductSize {
    pub size: i32,
}
