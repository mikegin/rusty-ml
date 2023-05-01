use rand::prelude::*;

const or_gate: [[f64; 3]; 4] = [
    [0., 0., 0.], 
    [1., 0., 1.], 
    [0., 1., 1.], 
    [1., 1., 1.],
];

const and_gate: [[f64; 3]; 4] = [
    [0., 0., 0.], 
    [1., 0., 0.], 
    [0., 1., 0.], 
    [1., 1., 1.],
];

const nand_gate: [[f64; 3]; 4] = [
    [0., 0., 1.], 
    [1., 0., 1.], 
    [0., 1., 1.], 
    [1., 1., 0.],
];

const xor_gate: [[f64; 3]; 4] = [
    [0., 0., 0.], 
    [1., 0., 1.], 
    [0., 1., 1.], 
    [1., 1., 0.],
];

fn sigmoid(v: f64) -> f64 {
    1. / (1. + std::f64::consts::E.powf(-1. * v))
}

fn cost(training_data: &Vec<Vec<f64>>, weights: &Vec<f64>, biases: &Vec<f64>) -> f64 {
    let mut result: f64 = 0.;
    for i in 0..training_data.len() {
        let mut y: f64 = 0.;
        // pretty much matrix multiplication here
        for xi in 0..training_data[i].len() {
            y += training_data[i][xi] * weights[xi];
        }
        y += biases[i];
        y = sigmoid(y);
        let d = training_data[i][training_data[i].len() - 1] - y; // last element in training data is expected result
        result += d*d;

    }
    result /= training_data.len() as f64;
    result
}

fn learn(training_data: Vec<Vec<f64>>) {
    assert!(training_data.len() > 0);
    assert!(training_data[0].len() > 0);

    let mut rng = rand::thread_rng();
    let mut weights: Vec<f64> = (0..training_data[0].len()).map(|_| rng.gen()).collect();
    let mut biases: Vec<f64> = (0..training_data.len()).map(|_| rng.gen()).collect();

    let eps: f64 = 1e-1;
    let rate: f64 = 1e-1;

    for i in 0..10000 {
        let c = cost(&training_data, &weights, &biases);

        //update weights
        let mut new_weights = weights.clone(); //TODO: optimize?
        for wi in 0..weights.len() {
            let old = weights[wi];
            weights[wi] = weights[wi] + eps;
            let d = (cost(&training_data, &weights, &biases) - c) / eps;
            weights[wi] = old;
            new_weights[wi] -= rate * d;
        }

        //update biases
        let mut new_biases = biases.clone();
        for bi in 0..biases.len() {
            let old = biases[bi];
            biases[bi] = biases[bi] + eps;
            let d = (cost(&training_data, &weights, &biases) - c) / eps;
            biases[bi] = old;
            new_biases[bi] -= rate * d;
        }

        //set new weights and biases
        weights = new_weights;
        biases = new_biases;


        // println!("cost = {}, w1 = {}, w2 = {}, b = {}", cost(training_set, w1, w2, b), w1, w2, b);
    }

    println!("------------------------");
    let allowed_error_diff = 1e-1;
    for i in 0..training_data.len() {
        let expected = training_data[i][training_data[i].len() - 1];
        let mut result: f64 = 0.;
        for j in 0..training_data[i].len() {
            result += training_data[i][j] * weights[j]; 
        }
        result += biases[i];
        result = sigmoid(result);

        println!("expected = {}, result = {}", expected, result);
        assert!((expected - result).abs() < allowed_error_diff)
    }
}


fn transform_a_to_v_matrix(a: [[f64; 3]; 4]) -> Vec<Vec<f64>> {
    let mut v: Vec<Vec<f64>> = (0..a.len()).map(|x| vec![]).collect();
    for i in 0..a.len() {
        v[i] = a[i].to_vec();
    }
    v
}

#[test]
fn test_or() {
    let v_or_gate: Vec<Vec<f64>> = transform_a_to_v_matrix(or_gate);
    learn(v_or_gate);
}

#[test]
fn test_and() {
    let v_and_gate: Vec<Vec<f64>> = transform_a_to_v_matrix(and_gate);
    learn(v_and_gate);
}

#[test]
fn test_nand() {
    let v_nand_gate: Vec<Vec<f64>> = transform_a_to_v_matrix(nand_gate);
    learn(v_nand_gate);
}

#[test]
fn test_xor() {
    //passes this time :)
    let v_xor_gate: Vec<Vec<f64>> = transform_a_to_v_matrix(xor_gate);
    learn(v_xor_gate);
}