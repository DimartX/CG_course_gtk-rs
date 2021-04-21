use std::collections::HashMap;
use gtk::prelude::*;
use crate::transformations::TransformMatrix;

// Shared state for communication between buttons and drawingarea
pub struct State {
    pub control_points: Vec<Vec<[f64; 4]>>,
    pub transformed_control_points: Vec<Vec<[f64; 4]>>,
    pub mouse_x:  f64,
    pub mouse_y:  f64,
    pub rotate_ox: f64,
    pub rotate_oy: f64,
    pub rotate_oz: f64,
    pub parts_ox: i32,
    pub parts_oy: i32,
    pub zoom: f64,
    pub point_chosen_num: (i32, i32),
    pub back_transform: TransformMatrix,
}

pub const surface_order: usize = 4;

fn make_default_surface() -> Vec<Vec<[f64; 4]>> {
    let mut controls: Vec<Vec<[f64; 4]>> = Vec::new();
    for i in 0..surface_order {
        controls.push(Vec::new());
        for j in 0..surface_order {
            controls[i].push([
                (i + 1) as f64 / 4.0,
                (j + 1) as f64 / 4.0,
                (((i + 123) * (j + 321)) % 500) as f64 / 400.0,
                1.0]);
        }
    }
    controls
}

impl State {
    pub fn new(buttons: &HashMap<String, gtk::SpinButton>) -> Self {
        State {
            control_points: make_default_surface(),
            transformed_control_points: make_default_surface(),
            mouse_x:  0.0,
            mouse_y:  0.0,
            rotate_ox: 0.0,
            rotate_oy: 0.0,
            rotate_oz: 0.0,
            parts_ox: buttons.get("parts_ox").unwrap().get_value_as_int(),
            parts_oy: buttons.get("parts_oy").unwrap().get_value_as_int(),
            zoom:     buttons.get("zoom").unwrap().get_value(),
            point_chosen_num: (-1, -1),
            back_transform: TransformMatrix::new(),
        }
    }
}
