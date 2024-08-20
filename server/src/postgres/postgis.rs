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
        Some(self.z)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    #[cfg(test)]
    fn hex_to_vec(hexstr: &str) -> Vec<u8> {
        hexstr.as_bytes().chunks(2).map(|chars| {
            let hb = if chars[0] <= 57 { chars[0] - 48 } else { chars[0] - 55 };
            let lb = if chars[1] <= 57 { chars[1] - 48 } else { chars[1] - 55 };
            hb * 16 + lb
        }).collect::<Vec<_>>()
    }

    #[rustfmt::skip]
    #[test]
    fn test_ewkb_read_for_geo_point_z() {
        ut_info!("start");
        // SELECT 'POINT(10 -20 100)'::geometry
        let ewkb = hex_to_vec("0101000080000000000000244000000000000034C00000000000005940");
        let point = GeoPointZ::read_ewkb(&mut ewkb.as_slice()).unwrap();
        assert_eq!(point, GeoPointZ { x: 10.0, y: -20.0, z: 100.0, });

        assert_eq!(GeoPointZ::point_type(), PointType::PointZ);
        ut_info!("success");
    }

    #[test]
    fn test_point_impl_for_geo_point_z() {
        use postgis::Point;
        ut_info!("start");
        let point = GeoPointZ {
            x: 40.123,
            y: -40.123,
            z: 100.0,
        };
        assert_eq!(point.x(), 40.123);
        assert_eq!(point.y(), -40.123);
        assert_eq!(point.opt_z(), Some(100.0));
        ut_info!("success");
    }

    #[rustfmt::skip]
    #[test]
    fn test_line_string_impl_for_geo_line_string() {
        use postgis::LineString;
        ut_info!("start");
        let points = vec![
            GeoPointZ { x: 40.123, y: -40.123, z: 100.0, },
            GeoPointZ { x: 41.123, y: -41.123, z: 100.0, },
            GeoPointZ { x: 42.123, y: -42.123, z: 90.0, },
            GeoPointZ { x: 40.123, y: -40.123, z: 100.0, },
        ];

        let line_string = GeoLineStringZ {
            points: points.clone(),
        };

        for (index, point) in line_string.points().enumerate() {
            assert_eq!(point, &points[index]);
        }

        ut_info!("success");
    }
}
