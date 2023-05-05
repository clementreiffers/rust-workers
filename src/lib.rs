mod factorial;
mod linear_regression;
mod maths;
mod utils;

use crate::factorial::compute_multiple_factorial_request;
use crate::linear_regression::compute_linear_regression_get;
use serde_json::json;
use worker::*;

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

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    utils::set_panic_hook();
    Router::new()
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .post_async("/form/:field", compute_form)
        .get("/worker-version", compute_worker_version)
        .get("/factorial/:number", compute_multiple_factorial_request)
        .get("/factorial", compute_multiple_factorial_request)
        .get("/linear-regression", compute_linear_regression_get)
        .run(req, env)
        .await
}
