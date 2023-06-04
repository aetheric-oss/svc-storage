impl From<Point> for GeoPoint {
    fn from(field: Point) -> Self {
        Self {
            x: field.x(),
            y: field.y(),
        }
    }
}

impl From<GeoPoint> for Point {
    fn from(field: GeoPoint) -> Self {
        Self::new(field.x, field.y)
    }
}
impl From<GeoPoint> for Coord {
    fn from(field: GeoPoint) -> Self {
        Coord {
            x: field.x,
            y: field.y,
        }
    }
}

impl From<LineString> for GeoLineString {
    fn from(field: LineString) -> Self {
        let mut points: Vec<GeoPoint> = vec![];
        for coord in field.coords() {
            let point: Point = (*coord).into();
            points.push(point.into());
        }
        Self { points }
    }
}
impl From<GeoLineString> for LineString {
    fn from(field: GeoLineString) -> Self {
        let mut points: Vec<Coord> = vec![];
        for point in field.points {
            points.push(point.into())
        }
        LineString::from(points)
    }
}

impl From<Polygon> for GeoPolygon {
    fn from(field: Polygon) -> Self {
        let mut interiors: Vec<GeoLineString> = vec![];
        for line in field.interiors() {
            interiors.push(line.clone().into())
        }

        Self {
            exterior: Some(field.exterior().clone().into()),
            interiors,
        }
    }
}
impl From<Vec<LineString>> for GeoPolygon {
    fn from(field: Vec<LineString>) -> Self {
        let mut polygon: Self = Self {
            exterior: None,
            interiors: vec![],
        };

        for mut line in field {
            // Make sure we're working with closed lines
            line.close();
            if polygon.exterior.is_some() {
                polygon.interiors.push(line.into())
            } else {
                polygon.exterior = Some(line.into())
            }
        }
        polygon
    }
}
impl From<GeoPolygon> for Polygon {
    fn from(field: GeoPolygon) -> Self {
        let mut interiors: Vec<LineString> = vec![];
        let exterior = match field.exterior {
            Some(val) => val.into(),
            None => LineString::new(vec![]),
        };

        for line in field.interiors {
            interiors.push(line.into())
        }
        Polygon::new(exterior, interiors)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_point_to_geo_point() {
        let x = 120.8;
        let y = -45.12;
        let from = Point::new(x, y);
        let expected = GeoPoint { x, y };

        // Point into GeoPoint
        let result: GeoPoint = from.into();
        assert_eq!(result, expected);
    }
    #[test]
    fn test_from_geo_point_to_point() {
        let x = 120.8;
        let y = -45.12;
        let from = GeoPoint { x, y };
        let expected = Point::new(x, y);

        // GeoPoint into Point
        let result: Point = from.into();
        assert_eq!(result, expected);
    }
    #[test]
    fn test_from_geo_point_to_coord() {
        let x = 120.8;
        let y = -45.12;
        let from = GeoPoint { x, y };
        let expected = Coord { x, y };

        // GeoPoint into Coord
        let result: Coord = from.into();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_line_string_to_geo_line_string() {
        let x_1 = 120.8;
        let y_1 = -45.12;
        let x_2 = 121.8;
        let y_2 = -46.12;
        let from = LineString::from(vec![(x_1, y_1), (x_2, y_2)]);
        let expected = GeoLineString {
            points: vec![GeoPoint { x: x_1, y: y_1 }, GeoPoint { x: x_2, y: y_2 }],
        };

        // LineString into GeoLineString
        let result: GeoLineString = from.into();
        assert_eq!(result, expected);
    }
    #[test]
    fn test_from_geo_line_string_to_line_string() {
        let x_1 = 120.8;
        let y_1 = -45.12;
        let x_2 = 121.8;
        let y_2 = -46.12;
        let from = GeoLineString {
            points: vec![GeoPoint { x: x_1, y: y_1 }, GeoPoint { x: x_2, y: y_2 }],
        };
        let expected = LineString::from(vec![(x_1, y_1), (x_2, y_2)]);

        // GeoLineString into LineString
        let result: LineString = from.into();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_polygon_to_geo_polygon() {
        let x_1 = 120.8;
        let y_1 = -45.12;
        let x_2 = x_1 - 0.001;
        let y_2 = y_1 - 0.001;
        let exterior = LineString::from(vec![(x_1, y_1), (x_2, y_2)]);
        let interiors = vec![LineString::from(vec![
            (x_1 - 1.0, y_1 - 1.0),
            (x_2 - 1.0, y_2 - 1.0),
        ])];
        let from = Polygon::new(exterior, interiors);
        let expected = GeoPolygon {
            exterior: Some(GeoLineString {
                points: vec![
                    GeoPoint { x: x_1, y: y_1 },
                    GeoPoint { x: x_2, y: y_2 },
                    GeoPoint { x: x_1, y: y_1 },
                ],
            }),
            interiors: vec![GeoLineString {
                points: vec![
                    GeoPoint {
                        x: x_1 - 1.0,
                        y: y_1 - 1.0,
                    },
                    GeoPoint {
                        x: x_2 - 1.0,
                        y: y_2 - 1.0,
                    },
                    GeoPoint {
                        x: x_1 - 1.0,
                        y: y_1 - 1.0,
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
        let x_2 = x_1 - 0.001;
        let y_2 = y_1 - 0.001;
        let exterior = LineString::from(vec![(x_1, y_1), (x_2, y_2)]);
        let interiors = vec![LineString::from(vec![
            (x_1 - 1.0, y_1 - 1.0),
            (x_2 - 1.0, y_2 - 1.0),
        ])];
        let from = GeoPolygon {
            exterior: Some(GeoLineString {
                points: vec![
                    GeoPoint { x: x_1, y: y_1 },
                    GeoPoint { x: x_2, y: y_2 },
                    GeoPoint { x: x_1, y: y_1 },
                ],
            }),
            interiors: vec![GeoLineString {
                points: vec![
                    GeoPoint {
                        x: x_1 - 1.0,
                        y: y_1 - 1.0,
                    },
                    GeoPoint {
                        x: x_2 - 1.0,
                        y: y_2 - 1.0,
                    },
                    GeoPoint {
                        x: x_1 - 1.0,
                        y: y_1 - 1.0,
                    },
                ],
            }],
        };
        let expected = Polygon::new(exterior, interiors);

        // GeoPolygon into Polygon
        let result: Polygon = from.into();
        assert_eq!(result, expected);
    }
    #[test]
    fn test_from_vec_line_string_to_geo_polygon() {
        let x_1 = 120.8;
        let y_1 = -45.12;
        let x_2 = x_1 - 0.001;
        let y_2 = y_1 - 0.001;
        let from = vec![
            LineString::from(vec![(x_1, y_1), (x_2, y_2)]),
            LineString::from(vec![(x_1 - 1.0, y_1 - 1.0), (x_2 - 1.0, y_2 - 1.0)]),
        ];
        let expected = GeoPolygon {
            exterior: Some(GeoLineString {
                points: vec![
                    GeoPoint { x: x_1, y: y_1 },
                    GeoPoint { x: x_2, y: y_2 },
                    GeoPoint { x: x_1, y: y_1 },
                ],
            }),
            interiors: vec![GeoLineString {
                points: vec![
                    GeoPoint {
                        x: x_1 - 1.0,
                        y: y_1 - 1.0,
                    },
                    GeoPoint {
                        x: x_2 - 1.0,
                        y: y_2 - 1.0,
                    },
                    GeoPoint {
                        x: x_1 - 1.0,
                        y: y_1 - 1.0,
                    },
                ],
            }],
        };

        // Vec<LineString> into GeoPolygon
        let result: GeoPolygon = from.into();
        assert_eq!(result, expected);
    }
}
