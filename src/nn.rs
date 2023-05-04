const or_gate: [[f32; 3]; 4] = [
    [0., 0., 0.], 
    [1., 0., 1.], 
    [0., 1., 1.], 
    [1., 1., 1.],
];

const xor_gate: [[f32; 3]; 4] = [
    [0., 0., 0.], 
    [1., 0., 1.], 
    [0., 1., 1.], 
    [1., 1., 0.],
];

//helper for tests to convert constant arrays to vectors
fn transform_a_to_v_matrix(a: [[f32; 3]; 4]) -> Vec<Vec<f32>> {
    let mut v: Vec<Vec<f32>> = (0..a.len()).map(|x| vec![]).collect();
    for i in 0..a.len() {
        v[i] = a[i].to_vec();
    }
    v
}

fn assert_matrix_size_eq(a: &Vec<Vec<f32>>, b: &Vec<Vec<f32>>) {
    assert_eq!(a.len(), b.len());
    assert!(a.len() > 0);
    
    for i in 0..a.len() {
        assert_eq!(a[i].len(), b[i].len())
    }
}

fn matrix_sum_mut(dst: &mut Vec<Vec<f32>>, src: &Vec<Vec<f32>>) {
    assert_matrix_size_eq(dst, src);

    for (i, row) in dst.iter_mut().enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col += src
                                .get(i)
                                .unwrap()
                                .get(j)
                                .unwrap();
        }
    }
}

fn matrix_dot(a: &Vec<Vec<f32>>, b: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    assert!(a.len() > 0);
    assert_eq!(a[0].len(), b.len());

    let mut result = vec![vec![0.; b[0].len()]; a.len()];

    let n = a[0].len();

    for i in 0..result.len() {
        for j in 0..result[i].len() {
            for k in 0..n {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_sum_mut() {
        let mut m1 = vec![
            vec![1., 2.]
        ];

        let m2 = vec![
            vec![3., 4.]
        ];

        matrix_sum_mut(&mut m1, &m2);

        assert_eq!(m1[0][0], 4.);
        assert_eq!(m1[0][1], 6.);
    }   

    #[test]
    fn test_matrix_dot_1() {
        let mut m1 = vec![
            vec![1., 2.]
        ];

        let m2 = vec![
            vec![3.],
            vec![4.]
        ];

        let expected = vec![
            vec![11.]
        ];

        let m3 = matrix_dot(&m1, &m2);
        
        for i in 0..m3.len() {
            for j in 0..m3[i].len() {
                assert_eq!(expected[i][j], m3[i][j]);
            }
        }
    }

    #[test]
    fn test_matrix_dot_2() {
        let mut m1 = vec![
            vec![1., 2.]
        ];

        let m2 = vec![
            vec![3., 5.],
            vec![4., 6.]
        ];

        let expected = vec![
            vec![11., 17.]
        ];

        let m3 = matrix_dot(&m1, &m2);
        
        for i in 0..m3.len() {
            for j in 0..m3[i].len() {
                assert_eq!(expected[i][j], m3[i][j]);
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_matrix_dot_invalid_matrices_row_col() {
        let mut m1 = vec![
            vec![1., 2.]
        ];

        let m2 = vec![
            vec![3., 5.],
        ];

        matrix_dot(&m1, &m2);
    }

    #[test]
    #[should_panic]
    fn test_matrix_dot_invalid_empty_matrix() {
        let mut m1 = vec![
            vec![]
        ];

        let m2 = vec![
            vec![],
        ];

        matrix_dot(&m1, &m2);
    }



    
    #[test]
    fn test_or() {
        let v = transform_a_to_v_matrix(or_gate);

    }    
}

