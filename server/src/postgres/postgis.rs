//! Implement Postgis Traits for our own Structs

use crate::grpc::server::geo_types::{GeoLineStringZ, GeoPointZ};
use crate::DEFAULT_SRID;
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

impl postgis::Point for GeoPointZ {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn opt_z(&self) -> Option<f64> {
        Some(self.x)
    }
}
impl EwkbRead for GeoPointZ {
    fn point_type() -> PointType {
        PointType::PointZ
    }
    fn read_ewkb_body<R: Read>(
        raw: &mut R,
        is_be: bool,
        _type_id: u32,
        _srid: Option<i32>,
    ) -> Result<Self, postgis::error::Error> {
        let x = read_f64(raw, is_be)?;
        let y = read_f64(raw, is_be)?;
        let z = read_f64(raw, is_be)?;
        Ok(GeoPointZ { x, y, z })
    }
}
impl<'a> AsEwkbPoint<'a> for GeoPointZ {
    fn as_ewkb(&'a self) -> EwkbPoint<'a> {
        EwkbPoint {
            geom: self,
            srid: Some(DEFAULT_SRID),
            point_type: PointType::PointZ,
        }
    }
}
impl<'a> FromSql<'a> for GeoPointZ {
    fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let mut rdr = Cursor::new(raw);
        GeoPointZ::read_ewkb(&mut rdr)
            .map_err(|_| format!("cannot convert {} to {}", ty, stringify!($ptype)).into())
    }

    accepts_geography!();
}
impl ToSql for GeoPointZ {
    fn to_sql(&self, _: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        self.as_ewkb().write_ewkb(&mut out.writer())?;
        Ok(IsNull::No)
    }

    to_sql_checked!();
    accepts_geography!();
}

impl<'a> postgis::LineString<'a> for GeoLineStringZ {
    type ItemType = GeoPointZ;
    type Iter = std::slice::Iter<'a, GeoPointZ>;

    fn points(&'a self) -> Self::Iter {
        self.points.iter()
    }
}
