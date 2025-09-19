use async_graphql;
use sea_orm::FromJsonQueryResult;
use seaography::{CustomFields, CustomInputType, CustomOutputType, CustomEnum};
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    CustomInputType,
    CustomOutputType,
    FromJsonQueryResult,
    PartialEq,
)]
pub struct Fill {
    pub color: Color,
    pub opacity: f64,
}

impl std::cmp::Eq for Fill {} // contains float values

#[derive(
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    CustomInputType,
    CustomOutputType,
    FromJsonQueryResult,
    PartialEq,
)]
pub struct Stroke {
    pub color: Color,
    pub width: f64,
    pub style: Style,
}

impl std::cmp::Eq for Stroke {} // contains float values

#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    CustomInputType,
    CustomOutputType,
    CustomEnum,
    Eq,
    FromJsonQueryResult,
    PartialEq,
    Default,
)]
pub enum Style {
    #[default]
    Solid,
    Dotted,
    Dashed,
}

#[derive(
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    CustomInputType,
    CustomOutputType,
    FromJsonQueryResult,
    PartialEq,
)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl std::cmp::Eq for Color {} // contains float values

#[derive(
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    CustomInputType,
    CustomOutputType,
    FromJsonQueryResult,
    PartialEq,
)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl std::cmp::Eq for Point {} // contains float values

impl Point {
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx * dx) + (dy * dy)).sqrt()
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    CustomInputType,
    CustomOutputType,
    FromJsonQueryResult,
    PartialEq,
)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl std::cmp::Eq for Size {} // contains float values

#[derive(
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    CustomInputType,
    CustomOutputType,
    FromJsonQueryResult,
    PartialEq,
    Eq,
)]
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

#[derive(
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    CustomInputType,
    CustomOutputType,
    FromJsonQueryResult,
    PartialEq,
)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl std::cmp::Eq for Circle {} // contains float values

#[CustomFields]
impl Circle {
    pub async fn area(&self) -> async_graphql::Result<f64> {
        Ok(std::f64::consts::PI * self.radius * self.radius)
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    CustomInputType,
    CustomOutputType,
    FromJsonQueryResult,
    PartialEq,
    Eq,
)]
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

#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    CustomInputType,
    CustomOutputType,
    FromJsonQueryResult,
    PartialEq,
    Eq,
)]
pub enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Triangle(Triangle),
}
