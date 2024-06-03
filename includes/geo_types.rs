use crate::DEFAULT_SRID;
use postgis::ewkb::{LineStringZ, PointZ, PolygonZ};

impl From<PointZ> for GeoPointZ {
    fn from(field: PointZ) -> Self {
        Self {
            x: field.x,
            y: field.y,
            z: field.z,
        }
    }
}

impl From<GeoPointZ> for PointZ {
    fn from(field: GeoPointZ) -> Self {
        Self {
            x: field.x,
            y: field.y,
            z: field.z,
            srid: Some(DEFAULT_SRID),
        }
    }
}

impl From<LineStringZ> for GeoLineStringZ {
    fn from(field: LineStringZ) -> Self {
        Self {
            points: field.points.into_iter().map(|point| point.into()).collect(),
        }
    }
}
impl From<GeoLineStringZ> for LineStringZ {
    fn from(field: GeoLineStringZ) -> Self {
        Self {
            points: field.points.into_iter().map(|point| point.into()).collect(),
            srid: Some(DEFAULT_SRID),
        }
    }
}

impl From<PolygonZ> for GeoPolygonZ {
    fn from(field: PolygonZ) -> Self {
        Self {
            rings: field.rings.into_iter().map(|line| line.into()).collect(),
        }
    }
}
impl From<Vec<LineStringZ>> for GeoPolygonZ {
    fn from(field: Vec<LineStringZ>) -> Self {
        Self {
            rings: field.into_iter().map(|line| line.into()).collect(),
        }
    }
}
impl From<GeoPolygonZ> for PolygonZ {
    fn from(field: GeoPolygonZ) -> Self {
        Self {
            rings: field.rings.into_iter().map(|line| line.into()).collect(),
            srid: Some(DEFAULT_SRID),
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
            z,
        };

        let expected = GeoPointZ { x, y, z };

        // Point into GeoPoint
        let result: GeoPointZ = from.into();
        assert_eq!(result, expected);
    }
    #[test]
    fn test_from_geo_point_to_point() {
        let x = 120.8;
        let y = -45.12;
        let z = 23.6;
        let from = GeoPointZ { x, y, z };

        let expected = PointZ {
            srid: Some(DEFAULT_SRID),
            x,
            y,
            z,
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

        let expected = GeoLineStringZ {
            points: vec![
                GeoPointZ {
                    x: x_1,
                    y: y_1,
                    z: z_1,
                },
                GeoPointZ {
                    x: x_2,
                    y: y_2,
                    z: z_2,
                },
            ],
        };

        // LineString into GeoLineString
        let result: GeoLineStringZ = from.into();
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
        let from = GeoLineStringZ {
            points: vec![
                GeoPointZ {
                    x: x_1,
                    y: y_1,
                    z: z_1,
                },
                GeoPointZ {
                    x: x_2,
                    y: y_2,
                    z: z_2,
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
                },
            ],
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
                },
            ],
        };

        let from = PolygonZ {
            srid: srid.clone(),
            rings: vec![exterior, interior],
        };

        let expected = GeoPolygonZ {
            rings: vec![
                GeoLineStringZ {
                    points: vec![
                        GeoPointZ {
                            x: x_1,
                            y: y_1,
                            z: z_1,
                        },
                        GeoPointZ {
                            x: x_2,
                            y: y_2,
                            z: z_1,
                        },
                    ],
                },
                GeoLineStringZ {
                    points: vec![
                        GeoPointZ {
                            x: x_1 - 1.0,
                            y: y_1 - 1.0,
                            z: z_2,
                        },
                        GeoPointZ {
                            x: x_2 - 1.0,
                            y: y_2 - 1.0,
                            z: z_2,
                        },
                    ],
                },
            ],
        };

        // Polygon into GeoPolygon
        let result: GeoPolygonZ = from.into();
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

        let interior = LineStringZ {
            srid: srid.clone(),
            points: vec![
                PointZ {
                    x: x_2 - 1.0,
                    y: y_2 - 1.0,
                    z: z_2,
                    srid: Some(DEFAULT_SRID),
                },
                PointZ {
                    x: x_1 - 1.0,
                    y: y_1 - 1.0,
                    z: z_1,
                    srid: Some(DEFAULT_SRID),
                },
            ],
        };

        let from = GeoPolygonZ {
            rings: vec![
                GeoLineStringZ {
                    points: vec![
                        GeoPointZ {
                            x: x_1,
                            y: y_1,
                            z: z_1,
                        },
                        GeoPointZ {
                            x: x_2,
                            y: y_2,
                            z: z_2,
                        },
                    ],
                },
                GeoLineStringZ {
                    points: vec![
                        GeoPointZ {
                            longitude: x_2 - 1.0,
                            latitude: y_2 - 1.0,
                            altitude: z_2,
                        },
                        GeoPointZ {
                            longitude: x_1 - 1.0,
                            latitude: y_1 - 1.0,
                            altitude: z_1,
                        },
                    ],
                },
            ],
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
