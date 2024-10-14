use crate::DEFAULT_SRID;
use postgis::ewkb::{LineStringZ, PointZ, PolygonZ};
use std::fmt;

impl From<PointZ> for GeoPointZ {
    fn from(field: PointZ) -> Self {
        Self {
            x: field.x,
            y: field.y,
            z: field.z,
        }
    }
}
impl fmt::Display for GeoPointZ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let point: PointZ = (*self).into();

        #[cfg(not(tarpaulin_include))]
        // no_coverage: (Rnever) It's impossible to get None, as GeoPointZ into PointZ will always add the default srid
        let srid = point.srid.unwrap_or(DEFAULT_SRID);

        f.write_str(&format!(
            "SRID={};POINT Z({:.15} {:.15} {:.15})",
            srid, point.x, point.y, point.z
        ))
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
impl fmt::Display for GeoLineStringZ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let line_string: LineStringZ = (*self).clone().into();

        #[cfg(not(tarpaulin_include))]
        // no_coverage: (Rnever) It's impossible to get None, as GeoPointZ into PointZ will always add the default srid
        let srid = line_string.srid.unwrap_or(DEFAULT_SRID);

        let line_string_points = line_string
            .points
            .into_iter()
            .map(|pt| format!("{:.15} {:.15} {:.15}", pt.x, pt.y, pt.z))
            .collect::<Vec<String>>()
            .join(","); // x y z, x y z, x y z
        f.write_str(&format!(
            "SRID={};LINESTRING Z({})",
            srid, line_string_points
        ))
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

impl fmt::Display for GeoPolygonZ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let polygon: PolygonZ = (*self).clone().into();

        #[cfg(not(tarpaulin_include))]
        // no_coverage: (Rnever) It's impossible to get None, as GeoPolygonZ into PolygonZ will always add the default srid
        let srid = polygon.srid.unwrap_or(DEFAULT_SRID);

        let polygon_rings = polygon
            .rings
            .into_iter()
            .map(|ring| {
                let ring_str = ring
                    .points
                    .into_iter()
                    .map(|pt| format!("{:.15} {:.15} {:.15}", pt.x, pt.y, pt.z))
                    .collect::<Vec<String>>()
                    .join(","); // x y z, x y z, x y z

                format!("({ring_str})") // (x y z, x y z, x y z)
            })
            .collect::<Vec<String>>()
            .join(","); // (x y z, x y z),(x y z, x y z)
        f.write_str(&format!("SRID={};POLYGON Z({})", srid, polygon_rings))
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

        // GeoPointZ into Point
        let result: PointZ = from.into();
        assert_eq!(result, expected);
    }
    #[test]
    fn test_geo_point_display() {
        let x = 120.8;
        let y = -45.12;
        let z = 23.6;
        let from = GeoPointZ { x, y, z };

        let expected = format!(
            "SRID={};POINT Z({:.15} {:.15} {:.15})",
            DEFAULT_SRID, x, y, z
        );

        let result = from.to_string();
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

        // GeoLineStringZ into LineString
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
            srid,
            points: vec![
                PointZ {
                    x: x_1,
                    y: y_1,
                    z: z_1,
                    srid,
                },
                PointZ {
                    x: x_2,
                    y: y_2,
                    z: z_2,
                    srid,
                },
            ],
        };

        let interior = LineStringZ {
            srid,
            points: vec![
                PointZ {
                    x: x_1 - 1.0,
                    y: y_1 - 1.0,
                    z: z_1,
                    srid,
                },
                PointZ {
                    x: x_2 - 1.0,
                    y: y_2 - 1.0,
                    z: z_2,
                    srid,
                },
            ],
        };

        let from = PolygonZ {
            srid,
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
                            z: z_2,
                        },
                    ],
                },
                GeoLineStringZ {
                    points: vec![
                        GeoPointZ {
                            x: x_1 - 1.0,
                            y: y_1 - 1.0,
                            z: z_1,
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
                            x: x_2 - 1.0,
                            y: y_2 - 1.0,
                            z: z_2,
                        },
                        GeoPointZ {
                            x: x_1 - 1.0,
                            y: y_1 - 1.0,
                            z: z_1,
                        },
                    ],
                },
            ],
        };

        let expected = PolygonZ {
            srid: srid,
            rings: vec![exterior, interior],
        };

        // GeoPolygonZ into Polygon
        let result: PolygonZ = from.into();
        assert_eq!(result, expected);
    }
}
