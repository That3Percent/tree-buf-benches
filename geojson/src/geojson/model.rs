use common::*;
use std::collections::HashMap;

// TODO: To support the full serde json value recursion is necessary.
#[derive(Decode, Encode, Debug, PartialEq)]
pub enum JsonValue {
    String(String),
}

#[derive(Decode, Encode, Debug, PartialEq)]
pub struct Polygon {
    pub coordinates: Vec<Vec<(f64, f64)>>,
}

#[derive(Decode, Encode, Debug, PartialEq)]
pub enum Geometry {
    Polygon(Polygon),
    MultiPolygon(Vec<Polygon>),
}

#[derive(Decode, Encode, Debug, PartialEq)]
pub struct Feature {
    pub properties: HashMap<String, JsonValue>,
    pub geometry: Geometry,
}

#[derive(Decode, Encode, Debug, PartialEq)]
pub enum GeoJson {
    FeatureCollection(Vec<Feature>),
}

#[derive(Decode, Debug, PartialEq)]
pub enum GeoJsonAttributes {
    FeatureCollection(Vec<FeatureAttributes>),
}

#[derive(Decode, Debug, PartialEq)]
pub struct FeatureAttributes {
    pub properties: HashMap<String, JsonValue>,
}
