use num_complex::Complex64;

pub trait PointLike: PartialEq<Self> + Clone + Copy {
    fn distance_to(&self, other: &Self) -> f64;
    fn heading_to(&self, other: &Self) -> f64;
    fn invert(&self, other: &Self) -> Self;
}

#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct HyperPoint {
    poincare: Complex64,
    klein: Complex64,
}

impl HyperPoint {
    pub fn from_poincare(poincare: Complex64) -> Result<HyperPoint, String> {
        let norm_sqr = poincare.norm_sqr();
        if norm_sqr > 1.0 {
            return Err("HyperPoint: Poincare model requires point inside disk".to_string());
        }
        let klein: Complex64 = poincare.scale(2.0 / (1.0 + norm_sqr));
        return Ok(HyperPoint { poincare, klein });
    }

    pub fn from_klein(klein: Complex64) -> Result<HyperPoint, String> {
        let norm_sqr = klein.norm_sqr();
        if norm_sqr > 1.0 {
            return Err("HyperPoint: Klein model requires point inside disk".to_string());
        }
        let poincare: Complex64 = klein.scale(1.0 / (1.0 + (1.0 + norm_sqr).sqrt()));
        return Ok(HyperPoint { poincare, klein });
    }
}

impl PointLike for HyperPoint {
    fn distance_to(&self, other: &HyperPoint) -> f64 {
        todo!()
    }

    fn heading_to(&self, other: &HyperPoint) -> f64 {
        todo!()
    }

    fn invert(&self, other: &HyperPoint) -> HyperPoint {
        todo!()
    }
}

impl PointLike for Complex64 {
    fn distance_to(&self, other: &Complex64) -> f64 {
        todo!()
    }

    fn heading_to(&self, other: &Complex64) -> f64 {
        todo!()
    }

    fn invert(&self, other: &Complex64) -> Complex64 {
        todo!()
    }
}

// struct Line {
//     // ax + by + c = 0
//     a: f64,
//     b: f64,
//     c: f64,
// }
//
// impl Line {
//     fn through_points(p1: Complex64, p2: Complex64) -> Result<Line, String> {
//         if p1.eq(&p2) {
//             return Err("Line: need distinct points".to_string());
//         }
//         if p1.is_infinite() || p2.is_infinite() {
//             return Err("Line: line with infinite endpoint is not well-defined".to_string());
//         }
//         let diff = p2.sub(p1);
//         let normal = (diff.div(diff.norm())).mul(Complex64::i());
//         let a = normal.re;
//         let b = normal.im;
//         let c = -a * p1.re - b * p1.im;
//         return Ok(Line { a, b, c });
//     }
//
//     fn from_src_dir(src: Complex64, dir: f64) -> Result<Line, String> {
//         Line::through_points(src, Complex64::from_polar(1., dir))
//     }
// }

// struct Ray<T: PointLike> {
//     src: T,
//     dir: f64,
// }
//
// struct Geodesic<T: PointLike> {
//     p1: T,
//     p2: T,
// }

// impl<T: PointLike> Geodesic<T> {
//     fn new(p1: T, p2: T) -> Result<Geodesic<T>, String> {
//         if p1.eq(&p2) {
//             return Err("Geodesic: degenerate geodesic".to_string());
//         }
//         Ok(Geodesic { p1, p2 })
//     }
// }
//
// impl GeodesicLike<Complex64> for Geodesic<Complex64> {
//     fn intersect(&self, other: Self) -> Option<Complex64> {
//         todo!()
//     }
//
//     fn length(&self) -> f64 {
//         self.p2.sub(self.p2).norm()
//     }
//
//     fn heading(&self) -> f64 {
//         self.p2.sub(self.p2).arg()
//     }
// }
//
// impl GeodesicLike<HyperPoint> for Geodesic<HyperPoint> {
//     fn intersect(&self, other: Self) -> Option<HyperPoint> {
//         todo!()
//     }
//
//     fn length(&self) -> f64 {
//         todo!()
//     }
//
//     fn heading(&self) -> f64 {
//         todo!()
//     }
// }

pub fn reflect(incident: f64, tangent: f64) -> f64 {
    todo!()
}

pub struct Line<T: PointLike> {
    point1: T,
    point2: T,
}

impl<T: PointLike> Line<T> {
    pub fn from_two_points(point1: T, point2: T) -> Line<T> {
        if point1.eq(&point2) {
            panic!("Line: need distinct points");
        }
        Line { point1, point2 }
    }

    pub fn intersect(&self, other: &Self) -> Option<T> {
        todo!()
    }

    pub fn start_heading(&self) -> f64 {
        todo!()
    }
}

pub struct Ray<T: PointLike> {
    start: T,
    direction: f64,
}

impl<T: PointLike> Ray<T> {
    pub fn from_two_points(point1: T, point2: T) -> Ray<T> {
        if point1.eq(&point2) {
            panic!("Line: need distinct points");
        }
        Ray { start: point1, direction: point1.heading_to(&point2) }
    }

    pub fn point_along(&self, distance: f64) -> T {
        todo!()
    }
}

pub struct Segment<T: PointLike> {
    point1: T,
    point2: T,
}


pub struct Circle<T> {
    center: T,
    radius: f64,
}

pub fn fourth_circle<T: PointLike>(tangency: &T, g1: &Line<T>, g2: &Line<T>) -> Circle<T> {
    todo!()
}
