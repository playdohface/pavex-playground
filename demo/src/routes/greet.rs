use pavex::{request::path::PathParams, response::Response};

use crate::user_agent::UserAgent;

#[PathParams]
pub struct GreetParams {
    name: String,
}

pub fn greet(params: PathParams<GreetParams>, useragent: UserAgent) -> Response {
    if let UserAgent::Known(agent) = useragent {
        let GreetParams { name } = params.0;
        Response::ok().set_typed_body(format! {"Howdy {name}, or shall I say {agent}?"})
    } else {
        Response::unauthorized().set_typed_body("You must set the User-Agent Header")
    }
}
