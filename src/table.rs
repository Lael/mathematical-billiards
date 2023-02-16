use crate::geometry::{Circle, Line, PointLike, Ray};

pub struct PolygonTable<T: PointLike> {
    n: usize,
    vertices: Vec<T>,
    vertex_times: Vec<f64>,
    perimeter: f64,
}

// Used for simply-connected regions in/around which billiards may be played.
pub trait Table<T: PointLike> {
    fn point(&self, point_time: f64) -> T;

    // The anti-clockwise tangent heading (in the hyperbolic case, the Poincaré model is used).
    fn tangent_heading(&self, point_time: f64) -> f64;

    // chord_start_time refers to the parametrization of the boundary of the table
    // chord_heading is absolute (in the hyperbolic case, the Poincaré model is used).
    fn chord_end(&self, chord_start_time: f64, chord_heading: f64) -> (f64, f64);

    fn right_tangent_point(&self, point: &T) -> Result<T, String>;
    fn left_tangent_point(&self, point: &T) -> Result<T, String>;

    fn right_tangent_to_circle(&self, circle: &Circle<T>) -> Line<T>;
}

impl<T: PointLike> PolygonTable<T> {
    fn new(vertices: Vec<T>) -> Result<PolygonTable<T>, String> {
        let n = vertices.len();
        if n < 3 {
            return Err("Polygon: polygon requires at least 3 vertices".to_string());
        }

        let mut perimeter: f64 = 0.0;
        let mut current_vertex = &vertices[0];
        let mut vertex_times: Vec<f64> = vec![0.0; n];
        for i in 0..n {
            vertex_times[i] = perimeter;
            let v = &vertices[(i + 1) % n];
            let d = current_vertex.distance_to(v);
            perimeter += d;
            current_vertex = v;
        }
        return Ok(PolygonTable { n, vertices, vertex_times, perimeter });
    }
}

impl<T: PointLike> Table<T> for PolygonTable<T> {
    fn point(&self, point_time: f64) -> T {
        let fixed_time = point_time.rem_euclid(1.) * self.perimeter;
        for i in 0..self.n {
            if fixed_time == self.vertex_times[i] {
                return self.vertices[i].clone();
            }
            if fixed_time < self.vertex_times[i] {
                let vertex1 = &self.vertices[i - 1];
                let vertex2 = &self.vertices[i];
                let distance = fixed_time - self.vertex_times[i - 1];
                return Ray::from_two_points(vertex1.clone(), vertex2.clone()).point_along(distance);
            }
        }
        todo!()
    }

    fn tangent_heading(&self, point_time: f64) -> f64 {
        todo!()
    }

    fn chord_end(&self, chord_start_time: f64, chord_heading: f64) -> (f64, f64) {
        todo!()
    }

    fn right_tangent_point(&self, point: &T) -> Result<T, String> {
        todo!()
    }

    fn left_tangent_point(&self, point: &T) -> Result<T, String> {
        todo!()
    }

    fn right_tangent_to_circle(&self, circle: &Circle<T>) -> Line<T> {
        todo!()
    }
}
