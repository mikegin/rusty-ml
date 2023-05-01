use rand::prelude::*;

const train: [[f64; 2]; 5] = [
    [0., 0.], 
    [1., 2.], 
    [2., 4.], 
    [3., 6.],
    [4., 8.],
];

// y = wx + b

fn cost(w: f64, b: f64) -> f64 {
    let mut result: f64 = 0.;
    for i in 0..train.len() {
        let x = train[i][0];
        let y = x * w + b;
        let mut d = train[i][1] - y;
        d *= d;
        result += d;
    }
    result /= train.len() as f64;
    result
}

#[test]
fn test() {
    let mut rng = rand::thread_rng();
    let mut w: f64 = rng.gen();
    let mut b: f64 = rng.gen();

    let eps: f64 = 1e-3;
    let rate: f64 = 1e-3;

    println!("cost initial: {}", cost(w, b));

    for i in 0..5000 {
        let c = cost(w, b);
        let dw = (cost(w + eps, b) - c) / eps;
        let db = (cost(w, b + eps) - c) / eps;

        w -= rate * dw;
        b -= rate * db;

        println!("cost = {}, w = {}, b = {}", cost(w, b), w, b);
    }

    println!("------------------------");
    let allowed_error_diff = 1e-1;
    for i in 0..train.len() {
        let expected = train[i][1];
        let result = w * train[i][0] + b;
        println!("expected = {}, result = {}", expected, result);
        assert!((expected - result).abs() < allowed_error_diff)
    }
}
