use crate::maths::{covariance, mean, variance};
use crate::verify_form_send;
use worker::{Request, Response, Result, RouteContext};

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

pub fn compute_linear_regression(req: Request, ctx: RouteContext<()>) {
    // todo: implement
    // verify_form_send(req.clone());
    // if let Some(number_req) = ctx.param("number") {
    //     let number: u32 = number_req.trim().parse().expect("Please type a number!");
    //     let predictor = linear_regression_fn_generator(&vec![0.0, 0.0], &vec);
    // } else {
    // }
    // Response("done")
}
