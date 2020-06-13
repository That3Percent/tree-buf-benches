use common::*;
use geojson::GeoJson;
pub mod model;

impl From<geojson::PolygonType> for model::Polygon {
    fn from(polygon: geojson::PolygonType) -> Self {
        let coordinates: Vec<_> = polygon
            .iter()
            .map(|ring| {
                let v: Vec<_> = ring
                    .iter()
                    .map(|position| (position[0], position[1]))
                    .collect();
                v
            })
            .collect();
        model::Polygon { coordinates }
    }
}

impl From<geojson::GeoJson> for model::GeoJson {
    fn from(value: geojson::GeoJson) -> Self {
        match value {
            GeoJson::FeatureCollection(feature_collection) => model::GeoJson::FeatureCollection(
                feature_collection
                    .features
                    .into_iter()
                    .map(|feature| {
                        let geojson::Feature {
                            properties,
                            geometry,
                            ..
                        } = feature;
                        let properties = properties.unwrap();
                        let geometry = match geometry.unwrap().value {
                            geojson::Value::Polygon(polygon) => {
                                model::Geometry::Polygon(polygon.into())
                            }
                            geojson::Value::MultiPolygon(multi_polygon) => {
                                model::Geometry::MultiPolygon(
                                    multi_polygon.into_iter().map(|p| p.into()).collect(),
                                )
                            }
                            _ => unimplemented!(),
                        };
                        let properties = properties
                            .iter()
                            .map(|(key, value)| {
                                let value = match value {
                                    serde_json::Value::String(s) => {
                                        model::JsonValue::String(s.to_owned())
                                    }
                                    _ => unimplemented!(),
                                };
                                (key.to_owned(), value)
                            })
                            .collect();
                        model::Feature {
                            properties,
                            geometry,
                        }
                    })
                    .collect(),
            ),
            _ => unimplemented!(),
        }
    }
}

pub fn load_data(stats: &mut Stats) -> model::GeoJson {
    let geojson_str = include_str!("countries.geojson");
    let geojson = geojson_str.parse::<GeoJson>().unwrap();

    stats.profile(
        "GeoJson",
        0,
        &geojson,
        |g| g.to_string().into_bytes(),
        |v| std::str::from_utf8(v).unwrap().parse::<GeoJson>().unwrap(),
    );
    geojson.into()
}
