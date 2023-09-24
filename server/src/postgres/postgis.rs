//! Implement Postgis Traits for our own Structs

use crate::grpc::server::grpc_geo_types::{GeoLineString, GeoPoint, GeoPolygon};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use bytes::{BufMut, BytesMut};
use postgis::ewkb::*;
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use std::error::Error;
use std::io::{Cursor, Read};

macro_rules! accepts_geography {
    () => {
        fn accepts(ty: &Type) -> bool {
            match ty.name() {
                "geography" | "geometry" => true,
                _ => false,
            }
        }
    };
}

fn read_f64<R: Read>(raw: &mut R, is_be: bool) -> Result<f64, postgis::error::Error> {
    Ok(if is_be {
        raw.read_f64::<BigEndian>()?
    } else {
        raw.read_f64::<LittleEndian>()?
    })
}

impl postgis::Point for GeoPoint {
    fn x(&self) -> f64 {
        self.longitude
    }
    fn y(&self) -> f64 {
        self.latitude
    }
}
impl EwkbRead for GeoPoint {
    fn point_type() -> PointType {
        PointType::Point
    }
    fn read_ewkb_body<R: Read>(
        raw: &mut R,
        is_be: bool,
        _type_id: u32,
        _srid: Option<i32>,
    ) -> Result<Self, postgis::error::Error> {
        let longitude = read_f64(raw, is_be)?;
        let latitude = read_f64(raw, is_be)?;
        Ok(GeoPoint {
            longitude,
            latitude,
        })
    }
}
impl<'a> AsEwkbPoint<'a> for GeoPoint {
    fn as_ewkb(&'a self) -> EwkbPoint<'a> {
        EwkbPoint {
            geom: self,
            srid: None,
            point_type: PointType::Point,
        }
    }
}
impl<'a> FromSql<'a> for GeoPoint {
    fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let mut rdr = Cursor::new(raw);
        GeoPoint::read_ewkb(&mut rdr)
            .map_err(|_| format!("cannot convert {} to {}", ty, stringify!($ptype)).into())
    }

    accepts_geography!();
}
impl ToSql for GeoPoint {
    fn to_sql(&self, _: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        self.as_ewkb().write_ewkb(&mut out.writer())?;
        Ok(IsNull::No)
    }

    to_sql_checked!();
    accepts_geography!();
}

impl<'a> postgis::LineString<'a> for GeoLineString {
    type ItemType = GeoPoint;
    type Iter = std::slice::Iter<'a, GeoPoint>;

    fn points(&'a self) -> Self::Iter {
        self.points.iter()
    }
}

impl From<Point> for GeoPoint {
    fn from(field: Point) -> Self {
        GeoPoint {
            longitude: field.x,
            latitude: field.y,
        }
    }
}
impl From<LineString> for GeoLineString {
    fn from(field: LineString) -> Self {
        let mut points: Vec<GeoPoint> = vec![];
        for point in field.points {
            points.push(point.into())
        }
        GeoLineString { points }
    }
}

impl From<Polygon> for GeoPolygon {
    fn from(field: Polygon) -> Self {
        let mut polygon: Self = Self {
            exterior: None,
            interiors: vec![],
        };

        for line in field.rings {
            let line_string: GeoLineString = line.into();
            if polygon.exterior.is_some() {
                polygon.interiors.push(line_string)
            } else {
                polygon.exterior = Some(line_string)
            }
        }
        polygon
    }
}
