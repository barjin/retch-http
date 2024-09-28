use std::time::Duration;
use serde_json::json;
use serde::{Deserialize, Serialize};

use rocket::{Error, Ignite, Request, Rocket};
use tokio::{sync::OnceCell, task::JoinHandle};
use rocket::request::{FromRequest, Outcome};

// static SERVER: OnceCell<JoinHandle<Result<Rocket<Ignite>, Error>>> = OnceCell::const_new();

#[derive(Serialize, Deserialize)]
pub struct RequestHeaders(pub Vec<(String, String)>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestHeaders {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let mut headers = RequestHeaders(Vec::new());
        
        for header in req.headers().iter() {
            headers.0.push((header.name().to_string(), header.value().to_string())); 
        }

        Outcome::Success(headers)
    }
}


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



