use worker::{Request, Response, Result, RouteContext};

fn factorial(n: u32) -> u32 {
    (1..n).product()
}

fn multiple_factorial(n: u32) -> Vec<u32> {
    (0..n).map(factorial).collect::<Vec<u32>>()
}

pub fn compute_multiple_factorial_request(_: Request, ctx: RouteContext<()>) -> Result<Response> {
    if let Some(number_req) = ctx.param("number") {
        let number: u32 = number_req.trim().parse().expect("Please type a number!");
        multiple_factorial(number);
        Response::ok("done with number: ".to_owned() + number_req)
    } else {
        Response::ok("Please give a number in the request")
    }
}
