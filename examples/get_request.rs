//! This example shows how to use urlencoded to parse GET request parameters.
extern crate iron;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use urlencoded::UrlEncodedQuery;

fn log_params(req: &mut Request) -> IronResult<Response> {
    // Extract the decoded data as multimap, using the UrlEncodedQuery plugin.
    match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref multimap) => println!("Parsed POST request body:\n {:?}", multimap),
        Err(ref e) => println!("{:?}", e)
    };

    Ok(Response::with((status::Ok, "Hello!")))
}

// Test out the server with `curl -i "http://localhost:3000/?name=franklin&name=trevor"`
fn main() {
    Iron::new(log_params).http("127.0.0.1:3000").unwrap();
}
