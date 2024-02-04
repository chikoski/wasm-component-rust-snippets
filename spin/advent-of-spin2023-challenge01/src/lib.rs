use spin_sdk::http::{Request, Response, Router};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_advent_of_spin2023_challenge01(req: Request) -> Response {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let mut router = Router::new();
    router.get("/data", api::show_whishlist);
    router.post("/data", api::update_whishlist);
    router.handle(req)
}

mod api {
    use anyhow::Ok;
    use spin_sdk::{
        http::{IntoResponse, Params, Request, Response},
        key_value::Store,
    };

    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    struct Whishlist {
        value: String,
    }

    impl Whishlist {
        fn new(value: String) -> Whishlist {
            Whishlist { value }
        }
        fn empty() -> Whishlist {
            Whishlist::new(String::new())
        }
    }

    pub fn update_whishlist(req: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
        let store = Store::open_default()?;
        let posted_json_string = String::from_utf8(req.body().to_vec())?;
        let posted: Whishlist = serde_json::from_str(&posted_json_string)?;

        store.set_json(req.query(), &posted)?;
        let json_string = serde_json::to_string(&posted)?;

        Ok(Response::builder()
            .status(201)
            .header("content-type", "application/json")
            .body(json_string)
            .build())
    }

    pub fn show_whishlist(req: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
        let store = Store::open_default()?;
        let whilshlist = store.get_json(req.query())?.unwrap_or(Whishlist::empty());
        let json_string = serde_json::to_string(&whilshlist)?;

        Ok(Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(json_string)
            .build())
    }
}
