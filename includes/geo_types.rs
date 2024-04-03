use postgis::ewkb::{LineStringZ, PointZ, PolygonZ};
use crate::DEFAULT_SRID;

impl From<PointZ> for GeoPoint {
    fn from(field: PointZ) -> Self {
        Self {
            longitude: field.x,
            latitude: field.y,
            altitude: field.z,
        }
    }
}

impl From<GeoPoint> for PointZ {
    fn from(field: GeoPoint) -> Self {
        Self {
            x: field.longitude,
            y: field.latitude,
            z: field.altitude,
            srid: Some(DEFAULT_SRID),
        }
    }
}

impl From<LineStringZ> for GeoLineString {
    fn from(field: LineStringZ) -> Self {
        let points: Vec<GeoPoint> = field.points.into_iter().map(|point| point.into()).collect();
        Self { points }
    }
}
impl From<GeoLineString> for LineStringZ {
    fn from(field: GeoLineString) -> Self {
        LineStringZ {
            points: field.points.into_iter().map(|point| point.into()).collect(),
            srid: Some(DEFAULT_SRID),
        }
    }
}

impl From<PolygonZ> for GeoPolygon {
    fn from(field: PolygonZ) -> Self {
        let exterior = match field.rings.first() {
            Some(ring) => Some(ring.clone().into()),
            None => Some(GeoLineString { points: vec![] })
        };

        let interiors: Vec<GeoLineString> = field
            .rings
            .into_iter()
            .skip(1)
            .map(|line| line.into())
            .collect();

        Self {
            exterior,
            interiors
        }
    }
}
impl From<Vec<LineStringZ>> for GeoPolygon {
    fn from(field: Vec<LineStringZ>) -> Self {
        let exterior = match field.first() {
            Some(ring) => Some(ring.clone().into()),
            None => Some(GeoLineString { points: vec![] })
        };

        Self {
            exterior,
            interiors: field
                .into_iter()
                .skip(1)
                .map(|line| line.into())
                .collect(),
        }
    }
}
impl From<GeoPolygon> for PolygonZ {
    fn from(field: GeoPolygon) -> Self {
        let mut rings: Vec<LineStringZ> = field.interiors.into_iter().map(|line| line.into()).collect();
        if let Some(val) = field.exterior {
            rings.insert(0, val.into())
        }

        PolygonZ {
            srid: Some(DEFAULT_SRID),
            rings,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_point_to_geo_point() {
        let x = 120.8;
        let y = -45.12;
        let z = 100.2;
        let from = PointZ {
            srid: Some(DEFAULT_SRID),
            x,
            y,
            z
        };

        let expected = GeoPoint {
            longitude: x,
            latitude: y,
            altitude: z
        };

        // Point into GeoPoint
        let result: GeoPoint = from.into();
        assert_eq!(result, expected);
    }
    #[test]
    fn test_from_geo_point_to_point() {
        let x = 120.8;
        let y = -45.12;
        let z = 23.6;
        let from = GeoPoint {
            longitude: x,
            latitude: y,
            altitude: z
        };

        let expected = PointZ {
            srid: Some(DEFAULT_SRID),
            x,
            y,
            z
        };

        // GeoPoint into Point
        let result: PointZ = from.into();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_line_string_to_geo_line_string() {
        let x_1 = 120.8;
        let y_1 = -45.12;
        let z_1 = 50.8;
        let x_2 = 121.8;
        let y_2 = -46.12;
        let z_2 = 100.2;
        let from = LineStringZ {
            srid: Some(DEFAULT_SRID),
            points: vec![
                PointZ {
                    x: x_1,
                    y: y_1,
                    z: z_1,
                    srid: Some(DEFAULT_SRID),
                },
                PointZ {
                    x: x_2,
                    y: y_2,
                    z: z_2,
                    srid: Some(DEFAULT_SRID),
                },
            ],
        };
        
        let expected = GeoLineString {
            points: vec![
                GeoPoint {
                    longitude: x_1,
                    latitude: y_1,
                    altitude: z_1
                },
                GeoPoint {
                    longitude: x_2,
                    latitude: y_2,
                    altitude: z_2
                },
            ],
        };

        // LineString into GeoLineString
        let result: GeoLineString = from.into();
        assert_eq!(result, expected);
    }
    #[test]
    fn test_from_geo_line_string_to_line_string() {
        let x_1 = 120.8;
        let y_1 = -45.12;
        let z_1 = 50.8;
        let x_2 = 121.8;
        let y_2 = -46.12;
        let z_2 = 100.2;
        let from = GeoLineString {
            points: vec![
                GeoPoint {
                    longitude: x_1,
                    latitude: y_1,
                    altitude: z_1
                },
                GeoPoint {
                    longitude: x_2,
                    latitude: y_2,
                    altitude: z_2
                },
            ],
        };

        let expected = LineStringZ {
            srid: Some(DEFAULT_SRID),
            points: vec![
                PointZ {
                    x: x_1,
                    y: y_1,
                    z: z_1,
                    srid: Some(DEFAULT_SRID),
                },
                PointZ {
                    x: x_2,
                    y: y_2,
                    z: z_2,
                    srid: Some(DEFAULT_SRID),
                },
            ],
        };

        // GeoLineString into LineString
        let result: LineStringZ = from.into();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_polygon_to_geo_polygon() {
        let x_1 = 120.8;
        let y_1 = -45.12;
        let z_1 = 50.8;
        let x_2 = x_1 - 0.001;
        let y_2 = y_1 - 0.001;
        let z_2 = 100.2;

        let srid = Some(DEFAULT_SRID);
        let exterior = LineStringZ {
            srid: srid.clone(),
            points: vec![
                PointZ {
                    x: x_1,
                    y: y_1,
                    z: z_1,
                    srid: srid.clone(),
                },
                PointZ {
                    x: x_2,
                    y: y_2,
                    z: z_1,
                    srid: srid.clone(),
                }
            ]
        };

        let interior = LineStringZ {
            srid: srid.clone(),
            points: vec![
                PointZ {
                    x: x_1 - 1.0,
                    y: y_1 - 1.0,
                    z: z_2,
                    srid: srid.clone(),
                },
                PointZ {
                    x: x_2 - 1.0,
                    y: y_2 - 1.0,
                    z: z_2,
                    srid: srid.clone(),
                }
            ]
        };

        let from = PolygonZ {
            srid: srid.clone(),
            rings: vec![exterior, interior],
        };

        let expected = GeoPolygon {
            exterior: Some(GeoLineString {
                points: vec![
                    GeoPoint {
                        longitude: x_1,
                        latitude: y_1,
                        altitude: z_1
                    },
                    GeoPoint {
                        longitude: x_2,
                        latitude: y_2,
                        altitude: z_1
                    },
                ],
            }),
            interiors: vec![GeoLineString {
                points: vec![
                    GeoPoint {
                        longitude: x_1 - 1.0,
                        latitude: y_1 - 1.0,
                        altitude: z_2
                    },
                    GeoPoint {
                        longitude: x_2 - 1.0,
                        latitude: y_2 - 1.0,
                        altitude: z_2
                    },
                ],
            }],
        };

        // Polygon into GeoPolygon
        let result: GeoPolygon = from.into();
        assert_eq!(result, expected);
    }
    #[test]
    fn test_from_geo_polygon_to_polygon() {
        let x_1 = 120.8;
        let y_1 = -45.12;
        let z_1 = 50.8;
        let x_2 = x_1 - 0.001;
        let y_2 = y_1 - 0.001;
        let z_2 = 100.2;

        let srid = Some(DEFAULT_SRID);
        let exterior = LineStringZ {
            srid: srid.clone(),
            points: vec![
                PointZ { x: x_1, y: y_1, z: z_1, srid: Some(DEFAULT_SRID) },
                PointZ { x: x_2, y: y_2, z: z_2, srid: Some(DEFAULT_SRID) },
            ],
        };

        let interior = LineStringZ {
            srid: srid.clone(),
            points: vec![
                PointZ { x: x_2 - 1.0, y: y_2 - 1.0, z: z_2, srid: Some(DEFAULT_SRID)},
                PointZ { x: x_1 - 1.0, y: y_1 - 1.0, z: z_1, srid: Some(DEFAULT_SRID)}
            ],
        };

        let from = GeoPolygon {
            exterior: Some(GeoLineString {
                points: vec![
                    GeoPoint {
                        longitude: x_1,
                        latitude: y_1,
                        altitude: z_1
                    },
                    GeoPoint {
                        longitude: x_2,
                        latitude: y_2,
                        altitude: z_2
                    }
                ],
            }),
            interiors: vec![GeoLineString {
                points: vec![
                    GeoPoint {
                        longitude: x_2 - 1.0,
                        latitude: y_2 - 1.0,
                        altitude: z_2
                    },
                    GeoPoint {
                        longitude: x_1 - 1.0,
                        latitude: y_1 - 1.0,
                        altitude: z_1
                    }
                ],
            }],
        };

        let expected = PolygonZ {
            srid: srid.clone(),
            rings: vec![exterior, interior],
        };

        // GeoPolygon into Polygon
        let result: PolygonZ = from.into();
        assert_eq!(result, expected);
    }
}
