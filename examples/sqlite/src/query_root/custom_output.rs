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
    pub size: Option<ProductSize>,
}

#[derive(Clone, CustomOutput)]
pub struct ProductSize {
    pub size: i32,
}
