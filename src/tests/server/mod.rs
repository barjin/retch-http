use std::time::Duration;
use serde_json::json;

use rocket::{Error, Ignite, Rocket};
use tokio::task::JoinHandle;

pub mod request_headers;

use request_headers::RequestHeaders;

#[get("/")]
fn hello() -> String {
    "Hello, world!".into()
}

#[get("/headers")]
fn headers(headers: RequestHeaders) -> String {
    json!(headers.0).to_string()
}

// [TODO!!!] - Server is run each time a test is run, but it should be run only once for all tests
pub async fn get_server() -> JoinHandle<Result<Rocket<Ignite>, Error>> {
    let server = rocket::build()
        .mount("/", routes![hello, headers]);

    let handle = tokio::spawn(server.launch());
    tokio::time::sleep(Duration::from_millis(20)).await;
    handle
}



