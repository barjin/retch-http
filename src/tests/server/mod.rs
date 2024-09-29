use std::time::Duration;
use rocket::{Error, Ignite, Rocket};
use tokio::task::JoinHandle;

pub mod request_headers;
pub mod compression;

use request_headers::headers;
use compression::compression_route;

#[get("/")]
fn hello() -> String {
    "Hello, world!".into()
}

// [TODO!!!] - Server is run each time a test is run, but it should be run only once for all tests
pub async fn get_server() -> JoinHandle<Result<Rocket<Ignite>, Error>> {
    let server = rocket::build()
        .mount("/", routes![
            hello, 
            headers, 
            compression_route
        ]);

    let handle = tokio::spawn(server.launch());
    tokio::time::sleep(Duration::from_millis(50)).await;
    handle
}



