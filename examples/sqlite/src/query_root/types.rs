use async_graphql;
use sea_orm::entity::prelude::{DateTimeUtc, Decimal};
use seaography::{CustomFields, CustomInputType, CustomOutputType};

#[derive(Clone, CustomInputType)]
pub struct RentalRequest {
    pub customer: String,
    pub film: String,
    pub location: Option<Location>,
    pub timestamp: DateTimeUtc,
}

#[derive(Clone, CustomInputType)]
pub struct Location {
    pub city: String,
    pub county: Option<String>,
}

#[derive(Clone, CustomOutputType)]
pub struct PurchaseOrder {
    pub po_number: String,
    pub lineitems: Vec<Lineitem>,
}

#[derive(Clone, CustomOutputType)]
pub struct Lineitem {
    pub product: String,
    pub quantity: Decimal,
    pub size: Option<ProductSize>,
}

#[derive(Clone, CustomOutputType)]
pub struct ProductSize {
    pub size: i32,
}

#[derive(Clone, Copy, CustomInputType, CustomOutputType)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx * dx) + (dy * dy)).sqrt()
    }
}

#[derive(Clone, Copy, CustomInputType, CustomOutputType)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, CustomInputType, CustomOutputType)]
pub struct Rectangle {
    pub origin: Point,
    pub size: Size,
}

#[CustomFields]
impl Rectangle {
    pub async fn area(&self) -> async_graphql::Result<f64> {
        Ok(self.size.width * self.size.height)
    }
}

#[derive(Clone, CustomInputType, CustomOutputType)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

#[CustomFields]
impl Circle {
    pub async fn area(&self) -> async_graphql::Result<f64> {
        Ok(std::f64::consts::PI * self.radius * self.radius)
    }
}

#[derive(Clone, CustomInputType, CustomOutputType)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

#[CustomFields]
impl Triangle {
    pub async fn area(&self) -> async_graphql::Result<f64> {
        let a = self.p1.distance_to(&self.p2);
        let b = self.p2.distance_to(&self.p3);
        let c = self.p3.distance_to(&self.p1);
        let s = (a + b + c) / 2.0;
        Ok((s * (s - a) * (s - b) * (s - c)).sqrt())
    }
}

#[derive(Clone, CustomInputType, CustomOutputType)]
pub enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Triangle(Triangle),
}
