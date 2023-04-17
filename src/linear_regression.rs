use crate::maths::{covariance, mean, variance};

fn _get_a(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    covariance(&x, &y) / variance(&x)
}

fn _get_b(x: &Vec<f32>, y: &Vec<f32>, a: f32) -> f32 {
    mean(&y) - a * mean(&x)
}

pub struct LinearRegression {
    a: f32,
    b: f32,
}

impl LinearRegression {
    pub fn new(a: f32, b: f32) -> LinearRegression {
        LinearRegression { a, b }
    }
    pub fn predict_value(&self, x: f32) -> f32 {
        self.a * x + self.b
    }
    pub fn predict_vector(&self, vector: &Vec<f32>) -> Vec<f32> {
        vector.iter().map(|&x| self.predict_value(x)).collect()
    }
}

pub fn linear_regression_fn_generator(x: &Vec<f32>, y: &Vec<f32>) -> impl Fn(f32) -> f32 {
    let a = _get_a(x, y);
    let b = _get_b(x, y, a);

    move |x| a * x + b
}

pub fn linear_regression(x: &Vec<f32>, y: &Vec<f32>) -> LinearRegression {
    let a = _get_a(x, y);
    let b = _get_b(x, y, a);

    LinearRegression::new(a, b)
}
