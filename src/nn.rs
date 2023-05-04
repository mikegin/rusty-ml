const or_gate: [[f64; 3]; 4] = [
    [0., 0., 0.], 
    [1., 0., 1.], 
    [0., 1., 1.], 
    [1., 1., 1.],
];

const xor_gate: [[f64; 3]; 4] = [
    [0., 0., 0.], 
    [1., 0., 1.], 
    [0., 1., 1.], 
    [1., 1., 0.],
];

//helper for tests to convert constant arrays to vectors
fn transform_a_to_v_matrix(a: [[f64; 3]; 4]) -> Vec<Vec<f64>> {
    let mut v: Vec<Vec<f64>> = (0..a.len()).map(|x| vec![]).collect();
    for i in 0..a.len() {
        v[i] = a[i].to_vec();
    }
    v
}

#[derive(Default, Debug)]
pub struct Matrix {
    rows: Vec<f32>,
    cols: Vec<f32>
}

impl Matrix {
    pub fn sum(&mut self, other: &Matrix) {
        for i in 0..self.rows.len() {
            for j in 0..self.cols.len() {
                let other_i = match other.rows.get(i) {
                    Some(x) => *x,
                    None => 0.
                };
                let other_j = match other.cols.get(j) {
                    Some(x) => *x,
                    None => 0.
                };
                
                self.rows.insert(i, *self.rows.get(i).unwrap() + other_i);
                self.cols.insert(j, *self.cols.get(j).unwrap() + other_j);

            }
        }
    }
    pub fn dot(&mut self, other: &Matrix) {
        for i in 0..self.rows.len() {
            for j in 0..self.cols.len() {
                let other_i = match other.rows.get(i) {
                    Some(x) => *x,
                    None => 0.
                };
                let other_j = match other.cols.get(j) {
                    Some(x) => *x,
                    None => 0.
                };

                let self_i = self.rows.get(i).unwrap();
                let self_j = self.cols.get(j).unwrap();
                
                self.rows.insert(i, self_i + *self_i * other_i);
                self.cols.insert(j, *self_j * other_j);

            }
        }
    }
}

pub struct NeuralNetwork {
    weights: Matrix,
    biases: Matrix,
    activations: Matrix
}

impl NeuralNetwork {
    pub fn forward(&self) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_sum() {
        let mut m1 = Matrix::default();
        m1.rows.push(1.);
        m1.cols.push(2.);

        
        let mut m2 = Matrix::default();
        m2.rows.push(3.);
        m2.cols.push(4.);

        m1.sum(&m2);

        assert_eq!(*m1.rows.get(0).unwrap(), 4.);
        assert_eq!(*m1.cols.get(0).unwrap(), 6.);

        assert_eq!(*m2.rows.get(0).unwrap(), 3.);
        assert_eq!(*m2.cols.get(0).unwrap(), 4.);
    }


    
    #[test]
    fn test_or() {
        let v = transform_a_to_v_matrix(or_gate);

    }    
}

