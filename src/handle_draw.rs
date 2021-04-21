use crate::figure::{FigureImpl, Figure};
use crate::state::State;
use crate::canvas::{CairoCanvas, Canvas};
use crate::transformations::{TransformMatrix, mult_matrix_on_transform};
use std::cmp::min;


use std::f64::consts::PI;
fn to_radians(angle: f64) -> f64 {
    angle / 180.0 * PI
}

pub fn handle_draw(canvas: &mut CairoCanvas, state: &mut State) {
    let width = canvas.width();
    let height = canvas.height();

    let coefficient = min(width, height) as f64 / 600.0;

    let center = min(width, height) as f64 / 2.1;
    let transformation =
        TransformMatrix::new()
        .move_by_vector([-0.5, -0.5, -0.5, 1.0])
        .rotate_ox(to_radians(state.rotate_ox))
        .rotate_oy(to_radians(state.rotate_oy))
        .rotate_oz(to_radians(state.rotate_oz))
        .zoom(state.zoom * 2.5 * coefficient)
        .move_by_vector([center, center, center, 1.0]);

    let back_transform =
        TransformMatrix::new()
        .move_by_vector([-center, -center, -center, 1.0])
        .zoom(1.0 / state.zoom / 2.5 / coefficient)
        .rotate_oz(-to_radians(state.rotate_oz))
        .rotate_oy(-to_radians(state.rotate_oy))
        .rotate_ox(-to_radians(state.rotate_ox))
        .move_by_vector([0.5, 0.5, 0.5, 1.0]);

    state.back_transform = back_transform;

    for i in 0..state.control_points.len() {
        state.transformed_control_points[i] =
            mult_matrix_on_transform(&state.control_points[i], transformation.mtx);
    }

    let bezier_surface = Figure::new_bezier_surface(
        state.transformed_control_points.clone(),
        (state.parts_ox as usize, state.parts_oy as usize));

    bezier_surface.draw(canvas, (state.parts_ox as usize, state.parts_oy as usize));
}
