use serde::{Deserialize, Serialize};
use rocket::request::{FromRequest, Outcome};

use rocket::Request;

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