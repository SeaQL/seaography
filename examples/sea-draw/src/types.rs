use async_graphql;
use sea_orm::FromJsonQueryResult;
use seaography::{CustomEnum, CustomFields, CustomInputType, CustomOutputType};
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

impl Fill {
    pub fn to_svg_attrs(&self) -> String {
        let mut svg = String::new();
        svg.push_str(&format!(" fill=\"{}\"", self.color.to_svg_color()));
        if self.opacity != 0.0 {
            svg.push_str(&format!(" fill-opacity=\"{}\"", self.opacity));
        }
        svg
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
pub struct Stroke {
    pub color: Color,
    pub width: f64,
    pub style: Style,
}

impl std::cmp::Eq for Stroke {} // contains float values

impl Stroke {
    pub fn to_svg_attrs(&self) -> String {
        let mut svg = String::new();
        svg.push_str(&format!(" stroke=\"{}\"", self.color.to_svg_color()));
        svg.push_str(&format!(" stroke-width=\"{}\"", self.width));
        svg
    }
}

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

impl Color {
    fn to_svg_color(&self) -> String {
        let r = (self.red * 255.0) as u8;
        let g = (self.green * 255.0) as u8;
        let b = (self.blue * 255.0) as u8;
        format!("rgb({}, {}, {})", r, g, b)
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

impl Rectangle {
    pub fn to_svg(&self, fill: &Fill, stroke: &Stroke) -> String {
        let mut svg = String::new();
        svg.push_str("<rect");
        svg.push_str(&format!(
            " x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"",
            self.origin.x, self.origin.y, self.size.width, self.size.height,
        ));
        svg.push_str(&fill.to_svg_attrs());
        svg.push_str(&stroke.to_svg_attrs());
        svg.push_str("/>");
        svg
    }
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

impl Circle {
    pub fn to_svg(&self, fill: &Fill, stroke: &Stroke) -> String {
        let mut svg = String::new();
        svg.push_str("<circle");
        svg.push_str(&format!(
            " cx=\"{}\" cy=\"{}\" r=\"{}\"",
            self.center.x, self.center.y, self.radius,
        ));
        svg.push_str(&fill.to_svg_attrs());
        svg.push_str(&stroke.to_svg_attrs());
        svg.push_str("/>");
        svg
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

impl Triangle {
    pub fn to_svg(&self, fill: &Fill, stroke: &Stroke) -> String {
        let mut svg = String::new();
        svg.push_str("<polygon ");
        svg.push_str(&format!(
            " points=\"{} {} {} {} {} {}\"",
            self.p1.x, self.p1.y, self.p2.x, self.p2.y, self.p3.x, self.p3.y,
        ));
        svg.push_str(&fill.to_svg_attrs());
        svg.push_str(&stroke.to_svg_attrs());
        svg.push_str("/>");
        svg
    }
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

impl Shape {
    pub fn to_svg(&self, fill: &Fill, stroke: &Stroke) -> String {
        match self {
            Shape::Rectangle(r) => r.to_svg(fill, stroke),
            Shape::Circle(c) => c.to_svg(fill, stroke),
            Shape::Triangle(t) => t.to_svg(fill, stroke),
        }
    }
}
