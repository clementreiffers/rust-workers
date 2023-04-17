use serde_json::json;
use std::collections::HashMap;
use worker::*;
mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

async fn compute_form(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    if let Some(name) = ctx.param("field") {
        let form = req.form_data().await?;
        match form.get(name) {
            Some(FormEntry::Field(value)) => {
                return Response::from_json(&json!({ name: value }));
            }
            Some(FormEntry::File(_)) => {
                return Response::error("`field` param in form shouldn't be a File", 422);
            }
            None => return Response::error("Bad Request", 400),
        }
    }

    Response::error("Bad Request", 400)
}

fn compute_worker_version(_: Request, ctx: RouteContext<()>) -> Result<Response> {
    let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
    Response::ok(version)
}

fn factorial(n: u32) -> u32 {
    (1..n).product()
}

fn multiple_factorial(n: u32) -> Vec<u32> {
    (0..n).map(factorial).collect::<Vec<u32>>()
}

fn compute_multiple_factorial_request(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    if let Some(number_req) = ctx.param("number") {
        let number: u32 = number_req.trim().parse().expect("Please type a number!");
        multiple_factorial(number);
        Response::ok("done with number: ".to_owned() + number_req)
    } else {
        Response::ok("Please give a number in the request")
    }
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    utils::set_panic_hook();
    let router = Router::new();
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .post_async("/form/:field", compute_form)
        .get("/worker-version", compute_worker_version)
        .get("/factorial/:number", compute_multiple_factorial_request)
        .run(req, env)
        .await
}
