use spin_sdk::http::{Request, Response, Router};
use spin_sdk::http_component;

mod payload_optimizer;
mod family;

/// A simple Spin HTTP component.
#[http_component]
fn handle_advent_of_spin2023_challenge02(req: Request) -> Response {
    let mut router = Router::new();
    router.post("/", api::calculate_maximum_children);
    router.handle(req)
}

mod api {
    use serde::{Deserialize, Serialize};
    use spin_sdk::http::{IntoResponse, Params, Request, Response};

    use crate::family::Family;
    use crate::payload_optimizer::{optimize_payload, OptimizedPayload};

    #[derive(Deserialize)]
    struct Input {
        pub kids: Vec<u32>,
        pub weight: Vec<u32>,
        pub capacity: u32,
    }

    impl Into<Vec<Family>> for Input {
        fn into(self) -> Vec<Family> {
            self.kids.iter().zip(self.weight).map(|(kids, weight)| {
                Family::new(*kids, weight)
            }).collect()
        }
    }

    #[derive(Serialize)]
    struct Output {
        kids: u32,
    }

    impl From<OptimizedPayload> for Output {
        fn from(value: OptimizedPayload) -> Self {
            Output { kids: value.kids }
        }
    }

    pub fn calculate_maximum_children(
        req: Request,
        _: Params,
    ) -> anyhow::Result<impl IntoResponse> {
        let input: Input = serde_json::from_slice(req.body())?;
        let capacity = input.capacity;
        let mut family_list: Vec<Family> = input.into();
        let optimized = optimize_payload(&mut family_list, capacity);
        let json_string = serde_json::to_string(&Output::from(optimized))?;
        Ok(
            Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(json_string)
                .build()
        )
    }
}
