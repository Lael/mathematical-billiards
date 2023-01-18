use num_complex::Complex64;

use crate::geometry::HyperPoint;

pub struct Polygon<T> {
    vertices: Vec<T>,
}

trait PolygonTable<T> {
    fn point(t: f64) -> T;
    fn tangent_heading(t: f64) -> T;
}

impl<T> Polygon<T> {
    fn new(vertices: Vec<T>) -> Result<Polygon<T>, String> {
        if vertices.len() < 3 {
            return Err("Polygon: polygon requires at least 3 vertices".to_string());
        }
        return Ok(Polygon { vertices });
    }
}

impl PolygonTable<Complex64> for Polygon<Complex64> {
    fn point(t: f64) -> Complex64 {
        todo!()
    }

    fn tangent_heading(t: f64) -> Complex64 {
        todo!()
    }
}

impl PolygonTable<HyperPoint> for Polygon<HyperPoint> {
    fn point(t: f64) -> HyperPoint {
        todo!()
    }

    fn tangent_heading(t: f64) -> HyperPoint {
        todo!()
    }
}
