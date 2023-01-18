use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

use num_complex::Complex64;

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

struct Line {
    // ax + by + c = 0
    a: f64,
    b: f64,
    c: f64,
}

impl Line {
    fn through_points(p1: Complex64, p2: Complex64) -> Result<Line, String> {
        if p1.eq(&p2) {
            return Err("Line: need distinct points".to_string());
        }
        if p1.is_infinite() || p2.is_infinite() {
            return Err("Line: line with infinite endpoint is not well-defined".to_string());
        }
        let diff = p2.sub(p1);
        let normal = (diff.div(diff.norm())).mul(Complex64::i());
        let a = normal.re;
        let b = normal.im;
        let c = -a * p1.re - b * p1.im;
        return Ok(Line { a, b, c });
    }
}

struct Geodesic<T> {
    p1: T,
    p2: T,
}

struct Ray<T> {
    p: T,
    dir: f64,
}

trait Intersect<T> {
    fn intersect(&self, other: Self) -> Option<T>;
}

impl Intersect<Complex64> for Geodesic<Complex64> {
    fn intersect(&self, other: Self) -> Option<Complex64> {
        todo!()
    }
}

impl Intersect<HyperPoint> for Geodesic<HyperPoint> {
    fn intersect(&self, other: Self) -> Option<HyperPoint> {
        todo!()
    }
}
