use seaography::macros::CustomInput;

#[derive(Clone, CustomInput)]
pub struct RentalRequest {
    pub customer: String,
    pub film: String,
    pub location: Option<Location>,
}

#[derive(Clone, CustomInput)]
pub struct Location {
    pub city: String,
    pub county: Option<String>,
}
