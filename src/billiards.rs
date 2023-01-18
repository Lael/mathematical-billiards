use num_complex::Complex64;

use crate::geometry::HyperPoint;
use crate::polygon::Polygon;

enum Duality {
    Inner,
    Outer,
}

enum Flavor {
    Regular,
    Symplectic,
}

enum Geometry {
    Affine,
    Hyperbolic,
}

struct InnerBilliards<T> {
    table: Polygon<T>,
    flavor: Flavor,
    geometry: Geometry,
}

struct OuterBilliards<T> {
    table: Polygon<T>,
    flavor: Flavor,
    geometry: Geometry,
}

enum Billiards<T> {
    Inner(InnerBilliards<T>),
    Outer(OuterBilliards<T>),
}

struct InnerBilliardsState {
    t: f64,
    heading: f64,
}

struct OuterBilliardsState<T> {
    position: T,
}

enum BilliardsState<T> {
    Inner(InnerBilliardsState),
    Outer(OuterBilliardsState<T>),
}

impl Billiards<Complex64> {
    pub fn new(table: Polygon<Complex64>, duality: Duality, flavor: Flavor) -> Billiards<Complex64> {
        return match duality {
            Duality::Inner => Billiards::Inner(InnerBilliards { table, flavor, geometry: Geometry::Affine }),
            Duality::Outer => Billiards::Outer(OuterBilliards { table, flavor, geometry: Geometry::Affine }),
        };
    }
}

impl Billiards<HyperPoint> {
    pub fn new(table: Polygon<HyperPoint>, duality: Duality, flavor: Flavor) -> Billiards<HyperPoint> {
        return match duality {
            Duality::Inner => Billiards::Inner(InnerBilliards { table, flavor, geometry: Geometry::Hyperbolic }),
            Duality::Outer => Billiards::Outer(OuterBilliards { table, flavor, geometry: Geometry::Hyperbolic }),
        };
    }
}

trait BilliardsOperations<T> {
    fn next_state(&self, state: BilliardsState<T>) -> BilliardsState<T>;
    fn prev_state(&self, state: BilliardsState<T>) -> BilliardsState<T>;
}

impl<T> BilliardsOperations<T> for InnerBilliards<T> {
    fn next_state(&self, state: BilliardsState<T>) -> BilliardsState<T> {
        // Regular:
        // intersect chord with polygon, reflect to obtain new angle
        // Symplectic:
        // intersect chord with polygon, find tangent heading, intersect new chord with polygon,
        // find heading between first and second chord endpoints
        todo!()
    }

    fn prev_state(&self, state: BilliardsState<T>) -> BilliardsState<T> {
        // Same but all in reverse
        todo!()
    }
}

impl<T> BilliardsOperations<T> for OuterBilliards<T> {
    fn next_state(&self, state: BilliardsState<T>) -> BilliardsState<T> {
        // Choose vertex
        // Regular: invert through vertex
        // Symplectic: find tangent circle, etc.
        todo!()
    }

    fn prev_state(&self, state: BilliardsState<T>) -> BilliardsState<T> {
        // Same but all in reverse
        todo!()
    }
}
