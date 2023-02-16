use crate::geometry::{fourth_circle, Line, PointLike, reflect};
use crate::table::{PolygonTable, Table};

enum Flavor {
    Regular,
    Symplectic,
}

enum Geometry {
    Affine,
    Hyperbolic,
}

struct InnerBilliardsState {
    start_time: f64,
    heading: f64,
}

struct OuterBilliardsState<T: PointLike> {
    point: T,
}

struct PolygonalInnerBilliards<T: PointLike> {
    table: PolygonTable<T>,
    flavor: Flavor,
}

impl<T: PointLike> PolygonalInnerBilliards<T> {
    fn next_state(&self, state: &InnerBilliardsState) -> InnerBilliardsState {
        match self.flavor {
            Flavor::Regular => {
                let (chord_end_time, chord_end_heading) = self.table.chord_end(
                    state.start_time,
                    state.heading,
                );
                let end_tangent_heading = self.table.tangent_heading(chord_end_time);
                let new_heading = reflect(chord_end_heading, end_tangent_heading);
                InnerBilliardsState { start_time: chord_end_time, heading: new_heading }
            }
            Flavor::Symplectic => {
                let (chord_end_time, chord_end_heading) = self.table.chord_end(
                    state.start_time,
                    state.heading,
                );
                let (new_end_time, _) = self.table.chord_end(state.start_time, chord_end_heading);
                let old_start_point = self.table.point(state.start_time);
                let new_end_point = self.table.point(new_end_time);
                let new_heading = old_start_point.heading_to(&new_end_point);
                InnerBilliardsState { start_time: chord_end_time, heading: new_heading }
            }
        }
    }
    fn iterate(&self, state: InnerBilliardsState, steps: usize) -> Vec<T> {
        let mut vertices: Vec<T> = Vec::new();
        let mut current_state = state;
        vertices.push(self.table.point(current_state.start_time));
        for _ in 0..steps {
            current_state = self.next_state(&current_state);
            vertices.push(self.table.point(current_state.start_time));
        }
        vertices
    }
}

struct PolygonalOuterBilliards<T: PointLike> {
    table: PolygonTable<T>,
    flavor: Flavor,
}

impl<T: PointLike> PolygonalOuterBilliards<T> {
    fn next_state(&self, state: &OuterBilliardsState<T>) -> Result<OuterBilliardsState<T>, String> {
        // Choose vertex
        // Regular: invert through vertex
        // Symplectic: find tangent circle, etc.
        match self.flavor {
            Flavor::Regular => {
                let tangent_point_result = self.table.right_tangent_point(&state.point);
                return match tangent_point_result {
                    Ok(tangent_point) => {
                        let new_point = tangent_point.invert(&state.point);
                        Ok(OuterBilliardsState { point: new_point })
                    }
                    Err(error) => Err(error),
                };
            }
            Flavor::Symplectic => {
                let right_tangent_point_result = self.table.right_tangent_point(&state.point);
                let right_tangent_point = match right_tangent_point_result {
                    Ok(tangent_point) => tangent_point,
                    Err(error) => return Err(error),
                };
                let left_tangent_point_result = self.table.left_tangent_point(&state.point);
                let left_tangent_point = match left_tangent_point_result {
                    Ok(tangent_point) => tangent_point,
                    Err(error) => return Err(error),
                };
                let left_geodesic = Line::from_two_points(left_tangent_point.clone(), state.point.clone());
                let right_geodesic = Line::from_two_points(right_tangent_point.clone(), state.point.clone());
                let circle = fourth_circle(&right_tangent_point, &right_geodesic, &left_geodesic);
                let new_tangent_geodesic = self.table.right_tangent_to_circle(&circle);
                let intersection = right_geodesic.intersect(&new_tangent_geodesic);
                match intersection {
                    None => Err("No intersection".to_string()),
                    Some(point) => Ok(OuterBilliardsState { point }),
                }
            }
        }
    }

    fn iterate(&self, state: OuterBilliardsState<T>, steps: usize) -> Vec<T> {
        let mut vertices: Vec<T> = Vec::new();
        let mut current_state = state;
        vertices.push(current_state.point.clone());
        for _ in 0..steps {
            current_state = match self.next_state(&current_state) {
                Ok(state) => state,
                Err(_) => panic!("Failed to iterate outer billiards")
            };
            vertices.push(current_state.point.clone());
        }
        vertices
    }
    fn preimages() { todo!() }
    fn regions() { todo!() }
}
