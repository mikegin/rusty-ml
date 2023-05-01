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

fn cost(m: [[f64; 3]; 4], w1: f64, w2: f64, b: f64) -> f64 {
    let mut result: f64 = 0.;
    for i in 0..m.len() {
        let x1: f64 = m[i][0];
        let x2: f64 = m[i][1];
        let y: f64 = sigmoid(x1 * w1 + x2 * w2 + b);
        // println!("cost calc: {}, y: {}", x1 * w1 + x2 * w2 + b, y);
        let d = m[i][2] - y;
        result += d * d;
    }
    result /= m.len() as f64;
    result
}

fn learn(training_set: [[f64; 3]; 4] ) {
    let mut rng = rand::thread_rng();
    let mut w1: f64 = rng.gen();
    let mut w2: f64 = rng.gen();
    let mut b: f64 = rng.gen();

    let eps: f64 = 1e-1;
    let rate: f64 = 1e-1;

    // println!("cost initial: {}", cost(training_set, w1, w2, b));

    for i in 0..100000 {
        let c = cost(training_set, w1, w2, b);
        let dw1 = (cost(training_set, w1 + eps, w2, b) - c) / eps;
        let dw2 = (cost(training_set, w1, w2 + eps, b) - c) / eps;
        let db = (cost(training_set, w1, w2, b + eps) - c) / eps;
        // println!("dw1 = {}, dw2 = {}, db = {}", w1, w2, b);
        w1 -= rate * dw1;
        w2 -= rate * dw2;
        b -= rate * db;

        // println!("cost = {}, w1 = {}, w2 = {}, b = {}", cost(training_set, w1, w2, b), w1, w2, b);
    }

    println!("------------------------");
    let allowed_error_diff = 1e-1;
    for i in 0..training_set.len() {
        let expected = training_set[i][2];
        let result = sigmoid(w1 * training_set[i][0] + w2 * training_set[i][1] + b);
        println!("expected = {}, result = {}", expected, result);
        assert!((expected - result).abs() < allowed_error_diff)
    }
}



#[test]
fn test_or() {
    learn(or_gate);
}

#[test]
fn test_and() {
    learn(and_gate);
}

#[test]
fn test_nand() {
    learn(nand_gate);
}

#[test]
fn test_xor() {
    //fails! cannot be done with 1 neuron...
    learn(xor_gate);
}