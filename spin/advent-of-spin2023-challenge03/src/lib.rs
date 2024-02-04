use spin_sdk::http::{Request, Response, Router};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_advent_of_spin2023_challenge03(req: Request) -> Response {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let mut router = Router::new();
    router.post("/", api::generate_story);
    router.handle(req)
}

mod api {
    use serde::{Deserialize, Serialize};
    use spin_sdk::http::{IntoResponse, Params, Request, Response};
    use spin_sdk::llm;

    #[derive(Deserialize, Debug)]
    struct Input {
        place: String,
        characters: Vec<String>,
        objects: Vec<String>,
    }

    struct Prompt {
        text: String,
    }

    impl Prompt {
        fn new(text: String) -> Prompt {
            Prompt { text }
        }
    }

    impl From<Input> for Prompt {
        fn from(value: Input) -> Self {
            let text =
                format!(
                    "Could you let me know a Christmas story in {} about {} and {}?",
                    value.place,
                    value.characters.join(","),
                    value.objects.join(",")
                );
            Prompt::new(text)
        }
    }

    #[derive(Serialize, Debug)]
    struct Output {
        story: String,
    }

    impl Output {
        fn new(story: String) -> Output {
            Output { story }
        }
    }

    pub fn generate_story(req: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
        let input: Input = serde_json::from_slice(req.body())?;
        let story = infer(input.into())?;
        let output = Output::new(story);
        let json_string = serde_json::to_string(&output)?;
        Ok(
            Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(json_string)
                .build()
        )
    }

    fn infer(prompt: Prompt) -> anyhow::Result<String> {
        let model = llm::InferencingModel::Llama2Chat;
        let inference = llm::infer(model, &prompt.text)?;
        Ok(inference.text)
    }
}
