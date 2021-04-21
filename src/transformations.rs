// affine transformations

pub struct TransformMatrix {
    pub mtx: [[f64; 4]; 4],
}

fn multiply(lhs: [[f64; 4]; 4], rhs: [[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut result: [[f64; 4]; 4] = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result[i][j] += lhs[i][k] * rhs[k][j];
            }
        }
    }
    result
}


pub fn mult_matrix_on_transform(lhs: &Vec<[f64; 4]>, rhs: [[f64; 4]; 4]) -> Vec<[f64; 4]> {
    let mut result: Vec<[f64; 4]> = vec![[0.0; 4]; lhs.len()];
    for i in 0..lhs.len() {
        for j in 0..4 {
            for k in 0..4 {
                result[i][j] += lhs[i][k] * rhs[k][j];
                //println!("left {:?}, right {:?}", result[i][j], lhs[i][k] * rhs[k][j]);
            }
        }
    }
    result
}


impl TransformMatrix {
    pub fn new() -> Self {
        Self {
            mtx:
            [[1.0, 0.0, 0.0, 0.0],
             [0.0, 1.0, 0.0, 0.0],
             [0.0, 0.0, 1.0, 0.0],
             [0.0, 0.0, 0.0, 1.0]]
        }
    }

    pub fn stretch(&mut self, x: f64, y: f64, z: f64) -> Self {
        Self {
            mtx: multiply(
                self.mtx,

                [[x,   0.0, 0.0, 0.0],
                 [0.0,   y, 0.0, 0.0],
                 [0.0, 0.0,   z, 0.0],
                 [0.0, 0.0, 0.0, 1.0]]
            )
        }
    }

    pub fn rotate_ox(&mut self, angle: f64) -> Self {
        Self {
            mtx: multiply(
                self.mtx,
                [
                    [1.0,          0.0,         0.0, 0.0],
                    [0.0,  angle.cos(), angle.sin(), 0.0],
                    [0.0, -angle.sin(), angle.cos(), 0.0],
                    [0.0,          0.0,         0.0, 1.0],
                ]
            )
        }
    }

    pub fn rotate_oz(&mut self, angle: f64) -> Self {
        Self {
            mtx: multiply(
                self.mtx,
                [
                    [angle.cos(), 0.0, -angle.sin(), 0.0],
                    [0.0,         1.0,          0.0, 0.0],
                    [angle.sin(), 0.0,  angle.cos(), 0.0],
                    [0.0,         0.0,          0.0, 1.0],
                ]
            )
        }
    }

    pub fn rotate_oy(&mut self, angle: f64) -> Self {
        Self {
            mtx: multiply(
                self.mtx,
                [
                    [ angle.cos(), angle.sin(), 0.0, 0.0],
                    [-angle.sin(), angle.cos(), 0.0, 0.0],
                    [         0.0,         0.0, 1.0, 0.0],
                    [         0.0,         0.0, 0.0, 1.0],
                ]
            )
        }
    }

    pub fn move_by_vector(&mut self, [l, m, n, _]: [f64; 4]) -> Self {
        Self {
            mtx: multiply(
                self.mtx,
                [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [  l,   m,   n, 1.0],
                ]
            )
        }
    }

    pub fn zoom(&mut self, zoom: f64) -> Self {
        Self {
            mtx: multiply(
                self.mtx,
                [
                    [ zoom,  0.0,  0.0, 0.0],
                    [  0.0, zoom,  0.0, 0.0],
                    [  0.0,  0.0, zoom, 0.0],
                    [  0.0,  0.0,  0.0, 1.0],
                ]
            )
        }
    }

}
