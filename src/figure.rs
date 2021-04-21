use crate::canvas::{CairoCanvas, Canvas};
use std::vec::Vec;
use crate::transformations::*;
use crate::point::Point;
use crate::color::Color;

struct Polygon {
    points: Vec<[f64; 4]>,
}

impl Polygon {
    fn new() -> Self {
        Self {
            points: Vec::new(),
        }
    }
    fn from(pts: Vec<[f64; 4]>) -> Self {
        Self {
            points: pts,
        }
    }
}

pub trait FigureImpl {
    fn transform(&mut self, matrix: [[f64; 4]; 4]);
    fn draw(&self, canvas: &mut CairoCanvas, parts: (usize, usize));
}

pub struct Figure {
    surface_points: Vec<Vec<[f64; 4]>>,
    control_points: Vec<Vec<[f64; 4]>>,
}

fn combination_precomputation() -> [[f64; 30]; 30] {
    let mut precalc: [[f64; 30]; 30] = [[1.0; 30]; 30];
    for i in 1..30 {
        for j in 1..i {
            precalc[i][j] = precalc[i - 1][j - 1] + precalc[i - 1][j];
            //print!("{} ", precalc[i][j]);
        }
        //println!();
    }
    precalc
}

impl Figure {
    pub fn new_bezier_surface(control_points: Vec<Vec<[f64; 4]>>, parts: (usize, usize)) -> Self {
        let mut surface: Vec<Vec<[f64; 4]>> = Vec::new();
        let n = crate::state::surface_order - 1;
        let prec: [[f64; 30]; 30] = combination_precomputation();

        for ui in 0..=parts.0 {
            let u = ui as f64 / parts.0 as f64;
            surface.push(Vec::new());
            for vj in 0..=parts.1 {
                let v = vj as f64 / parts.1 as f64;

                let mut res: [f64; 4] = [0.0; 4];
                for i in 0..=n {
                    let b1 = prec[n][i] as f64 *
                        v.powi(i as i32) *
                        (1.0 - v).powi(n as i32 - i as i32);
                    for j in 0..=n {
                        let b2 = prec[n][j] as f64 *
                            u.powi(j as i32) *
                            (1.0 - u).powi(n as i32 - j as i32);
                        res[0] += control_points[i][j][0] * b1 * b2;
                        res[1] += control_points[i][j][1] * b1 * b2;
                        res[2] += control_points[i][j][2] * b1 * b2;
                    }
                }
                surface[ui].push(res);
            }
        }

        Self {
            surface_points: surface,
            control_points: control_points,
        }
    }
}

fn cross_product(a: [f64; 4], b: [f64; 4], c: [f64; 4]) -> [f64; 4] {
    [
        (a[1] - c[1]) * (b[2] - c[0]) - (a[2] - c[2]) * (b[1] - c[1]),
        (a[2] - c[2]) * (b[0] - c[0]) - (a[0] - c[0]) * (b[2] - c[2]),
        (a[0] - c[0]) * (b[1] - c[1]) - (a[1] - c[1]) * (b[0] - c[0]),
        1.0,
    ]
}

fn dot_product(lhs: [f64; 4], rhs: [f64; 4]) -> f64 {
    lhs[0] * rhs[0] + lhs[1] * rhs[1] + lhs[2] * rhs[2]
}

fn angle_vectors(lhs: [f64; 4], rhs: [f64; 4]) -> f64 {
    dot_product(lhs, rhs) /
        (dot_product(lhs, lhs).sqrt() * dot_product(rhs, rhs).sqrt())
}

fn norm(vector: [f64; 4]) -> [f64; 4] {
    let norma = dot_product(vector, vector).sqrt();
    [
        vector[0] / norma,
        vector[1] / norma,
        vector[2] / norma,
        1.0,
    ]
}

fn maximum(a: f64, b: f64) -> f64{
    if a > b {
        a
    }
    else {
        b
    }
}

fn draw_tile(canvas: &mut CairoCanvas, polygon: &Polygon, color: Color) {
    let mut pts: Vec<Point> = Vec::new();

    // let normal = cross_product(
    //     polygon.points[1],
    //     polygon.points[0],
    //     polygon.points[2],
    // );

    //let angle_normal_scene = angle_vectors(norm(normal), [0.0, 0.0, -1.0, 0.0]);
    //println!("Normal {:?}", normal[2]);
    //println!("angle {:?}", angle_normal_scene);

    for point in &polygon.points {
        //println!("Point x = {}, y = {}, z = {}", point[0], point[1], point[2]);
        pts.push(Point::new(point[0] as i32 // + canvas.width() / 2
                            ,
                            point[1] as i32 // + canvas.height() / 2
        ));
    }

    canvas.set_draw_color(color);
    canvas.draw_polygon(&pts);
}


impl FigureImpl for Figure {
    fn transform(&mut self, matrix: [[f64; 4]; 4]) {
        for row in &mut self.surface_points {
            *row = mult_matrix_on_transform(&row, matrix);
        }
        for row in &mut self.control_points {
            *row = mult_matrix_on_transform(&row, matrix);
        }
    }

    fn draw(&self, canvas: &mut CairoCanvas, parts: (usize, usize)) {
        canvas.set_draw_color(Color::new(0, 0, 0));

        for i in 0..parts.0 {
            for j in 0..parts.1 {
                draw_tile(canvas, &Polygon::from(vec![
                    self.surface_points[i][j],
                    self.surface_points[i][j + 1],
                    self.surface_points[i + 1][j + 1],
                    self.surface_points[i + 1][j],
                ]),
                Color::black());
            }
        }

        for i in 0..self.control_points.len() - 1 {
            for j in 0..self.control_points[i].len() - 1 {
                draw_tile(canvas, &Polygon::from(vec![
                    self.control_points[i][j],
                    self.control_points[i][j + 1],
                    self.control_points[i + 1][j + 1],
                    self.control_points[i + 1][j],
                ]),
                Color::blue());
            }
        }

        for i in 0..self.control_points.len() {
            for j in 0..self.control_points[i].len() {
                canvas.draw_filled_circle(&Point::new(
                    self.control_points[i][j][0] as i32,
                    self.control_points[i][j][1] as i32,
                ), 5.0);

                let px = self.control_points[i][j][0];
                let py = self.control_points[i][j][1];
                //println!("{:?}", (px, py));
            }
        }
    }
}
