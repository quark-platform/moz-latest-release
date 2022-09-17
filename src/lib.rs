use version::{get_source_url, get_version_from_target};
use worker::*;

mod utils;
mod version;

const SOURCE_HELP: &'static str = include_str!("../docs/source.html");
const INDEX_HELP: &'static str = include_str!("../docs/index.html");

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/", |_, _| Response::from_html(INDEX_HELP))
        .get_async("/version/:target", |_, ctx| async move {
            if let Some(target) = ctx.param("target") {
                let version = get_version_from_target(target).await;

                return if let Ok(version) = version {
                    Response::ok(version)
                } else {
                    let error = version.err().unwrap(); // Will always be an error
                    Response::error(error.to_string(), 500)
                };
            }

            Response::error("Bad Request", 400)
        })
        .get("/source", |_, _| Response::from_html(SOURCE_HELP))
        .get_async("/source/:target", |_, ctx| async move {
            if let Some(target) = ctx.param("target") {
                let version = get_version_from_target(target).await;

                return if let Ok(version) = version {
                    Response::redirect_with_status(Url::parse(&get_source_url(version))?, 302)
                } else {
                    let error = version.err().unwrap(); // Will always be an error
                    Response::error(format!("Error finding version: {}", error.to_string()), 500)
                };
            }

            Response::error("Bad Request", 400)
        })
        .get_async("/source/:target/url", |_, ctx| async move {
            if let Some(target) = ctx.param("target") {
                let version = get_version_from_target(target).await;

                return if let Ok(version) = version {
                    Response::ok(get_source_url(version))
                } else {
                    let error = version.err().unwrap(); // Will always be an error
                    Response::error(format!("Error finding version: {}", error.to_string()), 500)
                };
            }

            Response::error("Bad Request", 400)
        })
        .run(req, env)
        .await
}
