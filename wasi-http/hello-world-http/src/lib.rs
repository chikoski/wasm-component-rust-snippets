wit_bindgen::generate!({
    world: "wasi:http/proxy@0.2.0",
    generate_all
});

use crate::exports::wasi::http::incoming_handler::Guest;
use crate::wasi::http::types::{FieldKey, FieldValue, Headers, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam};
struct Component;

impl Guest for Component {
    fn handle(_: IncomingRequest, response_out: ResponseOutparam) {
        let headers = [
            (
                FieldKey::from("Content-Type"),
                FieldValue::from("text/html")
            )
        ];
        let headers = Headers::from_list(&headers).unwrap();
        let response = OutgoingResponse::new(headers);
        response.set_status_code(200).unwrap();
        let body = response.body().unwrap();
        let stream = body.write().unwrap();
        stream.blocking_write_and_flush(b"Hello, world!").unwrap();
        drop(stream);
        OutgoingBody::finish(body, None).unwrap();
        ResponseOutparam::set(response_out, Ok(response));
    }
}

export!(Component);
