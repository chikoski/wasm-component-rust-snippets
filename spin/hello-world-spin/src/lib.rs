use spin_sdk::http::{Request, Response, Router};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_hello_world_spin(req: Request) -> Response {
    let mut router = Router::new();
    router.get("/", api::greet);
    router.get("/:name", api::greet);
    router.handle(req)
}

mod api {
    use spin_sdk::http::{IntoResponse, Params, Request};

    pub fn greet(_: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
        let name = params.get("name").unwrap_or("Spin");
        let greetings = format!("Hello, {}", name);
        Ok(http::Response::builder()
            .status(200)
            .header("content-type", "text/plain")
            .body(greetings)?)
    }
}
