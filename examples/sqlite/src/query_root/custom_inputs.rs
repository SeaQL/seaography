use seaography::CustomInputType;

#[derive(Clone, CustomInputType)]
pub struct RentalRequest {
    pub customer: String,
    pub film: String,
    pub location: Option<Location>,
}

#[derive(Clone, CustomInputType)]
pub struct Location {
    pub city: String,
    pub county: Option<String>,
}
