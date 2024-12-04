#[allow(warnings)]
mod bindings;

use bindings::{exports::wasi::http::incoming_handler::{Guest, IncomingRequest, ResponseOutparam}, wasi::http::types::{FieldKey, FieldValue, Headers, OutgoingBody, OutgoingResponse}};

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

bindings::export!(Component with_types_in bindings);
