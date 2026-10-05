use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoCoordinate {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude_meters: Option<f64>,
}

impl GeoCoordinate {
    pub fn new(latitude: f64, longitude: f64) -> Result<Self, String> {
        if !(-90.0..=90.0).contains(&latitude) {
            return Err("Latitude invalide".to_string());
        }

        if !(-180.0..=180.0).contains(&longitude) {
            return Err("Longitude invalide".to_string());
        }

        Ok(Self {
            latitude,
            longitude,
            altitude_meters: None,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub coordinate: Option<GeoCoordinate>,
    pub country_code: Option<String>,
    pub timezone: Option<String>,
    pub parent_location: Option<Uuid>,
}

impl Location {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: None,
            coordinate: None,
            country_code: None,
            timezone: None,
            parent_location: None,
        }
    }

    pub fn set_coordinate(
        &mut self,
        coordinate: GeoCoordinate,
    ) {
        self.coordinate = Some(coordinate);
    }

    pub fn set_parent(&mut self, parent: Uuid) {
        self.parent_location = Some(parent);
    }
}
