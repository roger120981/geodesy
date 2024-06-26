use crate::coordinate::*;

/// Generic 4D coordinate tuple, with no fixed interpretation of the elements
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Coor4D(pub [f64; 4]);

impl CoordinateTuple for Coor4D {
    fn new(fill: f64) -> Self {
        Coor4D([fill; 4])
    }

    fn dim(&self) -> usize {
        4
    }

    fn nth_unchecked(&self, n: usize) -> f64 {
        self.0[n]
    }

    fn set_nth_unchecked(&mut self, n: usize, value: f64) {
        self.0[n] = value;
    }
}

// ----- C O N S T R U C T O R S ---------------------------------------------

/// Constructors
impl Coor4D {
    /// A `Coor4D` from latitude/longitude/height/time, with the angular input in degrees
    #[must_use]
    pub fn geo(latitude: f64, longitude: f64, height: f64, time: f64) -> Coor4D {
        Coor4D([longitude.to_radians(), latitude.to_radians(), height, time])
    }

    /// A `Coor4D` from longitude/latitude/height/time, with the angular input in seconds
    /// of arc. Mostly for handling grid shift elements.
    #[must_use]
    pub fn arcsec(longitude: f64, latitude: f64, height: f64, time: f64) -> Coor4D {
        Coor4D([
            longitude.to_radians() / 3600.,
            latitude.to_radians() / 3600.,
            height,
            time,
        ])
    }

    /// A `Coor4D` from longitude/latitude/height/time, with the angular input in degrees
    #[must_use]
    pub fn gis(longitude: f64, latitude: f64, height: f64, time: f64) -> Coor4D {
        Coor4D([longitude.to_radians(), latitude.to_radians(), height, time])
    }

    /// A `Coor4D` from longitude/latitude/height/time, with the angular input in radians
    #[must_use]
    pub fn raw(first: f64, second: f64, third: f64, fourth: f64) -> Coor4D {
        Coor4D([first, second, third, fourth])
    }

    /// A `Coor4D` from latitude/longitude/height/time, with
    /// the angular input in the ISO-6709 DDDMM.mmmmm format
    #[must_use]
    pub fn iso_dm(latitude: f64, longitude: f64, height: f64, time: f64) -> Coor4D {
        let longitude = angular::iso_dm_to_dd(longitude);
        let latitude = angular::iso_dm_to_dd(latitude);
        Coor4D::geo(latitude, longitude, height, time)
    }

    /// A `Coor4D` from latitude/longitude/height/time, with the
    /// angular input in the ISO-6709 DDDMMSS.sssss format
    #[must_use]
    pub fn iso_dms(latitude: f64, longitude: f64, height: f64, time: f64) -> Coor4D {
        let longitude = angular::iso_dms_to_dd(longitude);
        let latitude = angular::iso_dms_to_dd(latitude);
        Coor4D::geo(latitude, longitude, height, time)
    }

    /// A `Coor4D` consisting of 4 `NaN`s
    #[must_use]
    pub fn nan() -> Coor4D {
        Coor4D([f64::NAN, f64::NAN, f64::NAN, f64::NAN])
    }

    /// A `Coor4D` consisting of 4 `0`s
    #[must_use]
    pub fn origin() -> Coor4D {
        Coor4D([0., 0., 0., 0.])
    }

    /// A `Coor4D` consisting of 4 `1`s
    #[must_use]
    pub fn ones() -> Coor4D {
        Coor4D([1., 1., 1., 1.])
    }
}

// ----- T E S T S ---------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::{Add, Div, Mul};

    #[test]
    fn distances() {
        let lat = angular::dms_to_dd(55, 30, 36.);
        let lon = angular::dms_to_dd(12, 45, 36.);
        let dms = Coor4D::geo(lat, lon, 0., 2020.);
        let geo = Coor4D::geo(55.51, 12.76, 0., 2020.);
        let e = Ellipsoid::default();
        assert!(e.distance(&geo, &dms) < 1e-10);
    }

    #[test]
    fn coord() {
        let c = Coor4D::raw(12., 55., 100., 0.).to_radians();
        let d = Coor4D::gis(12., 55., 100., 0.);
        assert_eq!(c, d);
        assert_eq!(d.x(), 12f64.to_radians());
        let e = d.to_degrees();
        assert_eq!(e.x(), c.to_degrees().x());
    }

    #[test]
    fn array() {
        let b = Coor4D::raw(7., 8., 9., 10.);
        let c = [b.x(), b.y(), b.z(), b.t(), f64::NAN, f64::NAN];
        assert_eq!(b.x(), c[0]);
    }

    #[test]
    fn arithmetic() {
        let a = Coor4D([1., 2., 3., 4.]);
        let b = Coor4D([4., 3., 2., 1.]);
        let t = Coor4D([12., 12., 12., 12.]);

        let c = a.add(b);
        assert_eq!(c, Coor4D([5., 5., 5., 5.]));

        let e = t.div(b);
        assert_eq!(e, Coor4D([3., 4., 6., 12.]));

        assert_eq!(e.mul(b), t);
    }
}
