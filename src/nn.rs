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

#[derive(Debug)]
pub struct Matrix {
    inner: Vec<Vec<f32>>
}

impl Matrix {
    pub fn new() -> Self {
       Self {
        inner: vec![vec![]]
       }
    }

    pub fn sum(&mut self, other: &Matrix) {
        assert_eq!(self.inner.len(), other.inner.len());
        assert!(self.inner.len() > 0);
        assert_eq!(self.inner.get(0).unwrap().len(), other.inner.get(0).unwrap().len());
        assert!(self.inner.get(0).unwrap().len() > 0);

        for (i, row) in self.inner.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                *col += other.inner
                                    .get(i)
                                    .unwrap()
                                    .get(j)
                                    .unwrap();
            }
        }
    }
    pub fn dot(&mut self, other: &Matrix) {
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
        let mut m1 = Matrix::new();
        m1.inner.get_mut(0).unwrap().push(1.);
        m1.inner.get_mut(0).unwrap().push(2.);
        
        let mut m2 = Matrix::new();
        m2.inner.get_mut(0).unwrap().push(3.);
        m2.inner.get_mut(0).unwrap().push(4.);

        m1.sum(&m2);

        assert_eq!(*m1.inner.get(0).unwrap().get(0).unwrap(), 4.);
        assert_eq!(*m1.inner.get(0).unwrap().get(1).unwrap(), 6.);

        assert_eq!(*m2.inner.get(0).unwrap().get(0).unwrap(), 3.);
        assert_eq!(*m2.inner.get(0).unwrap().get(1).unwrap(), 4.);
    }

    #[test]
    fn matrix_dot() {

    }


    
    #[test]
    fn test_or() {
        let v = transform_a_to_v_matrix(or_gate);

    }    
}

