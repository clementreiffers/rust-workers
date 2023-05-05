use crate::maths::{covariance, mean, variance};
use worker::*;

fn _get_a(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    covariance(&x, &y) / variance(&x)
}

fn _get_b(x: &Vec<f32>, y: &Vec<f32>, a: f32) -> f32 {
    mean(&y) - a * mean(&x)
}

fn linear_regression_fn_generator(x: &Vec<f32>, y: &Vec<f32>) -> impl Fn(f32) -> f32 {
    let a = _get_a(x, y);
    let b = _get_b(x, y, a);

    move |x| a * x + b
}

pub fn compute_linear_regression_get(_: Request, _: RouteContext<()>) -> Result<Response> {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    let predictor = linear_regression_fn_generator(&x, &y);
    Response::ok(format!("{:?}", predictor(1.0)))
}
