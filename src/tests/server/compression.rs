use async_compression::tokio::bufread::{GzipEncoder, BrotliEncoder, DeflateEncoder, ZstdEncoder};
use rocket::response::{self, Responder};
use serde::{Deserialize, Serialize};
use rocket::request::{FromRequest, Outcome};

use rocket::Request;
use serde_json::json;
use tokio::io::AsyncReadExt;

#[derive(Serialize, Deserialize, Debug)]
pub enum CompressionMethod {
    unknown,
    gzip,
    deflate,
    br,
    zstd,
}

pub static BODY: &'static str = "This is the data to be compressed!";

struct CompressedData {
    data: Vec<u8>,
    encoding: CompressionMethod,
}

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub body: &'static str,
    pub encoding: CompressionMethod,
}

impl Payload {
    pub fn new(encoding: CompressionMethod) -> Self {
        Payload {
            body: BODY,
            encoding,
        }
    }

    pub async fn compress(self) -> CompressedData {
        let json_serialized = json!(self).to_string();

        let buf_read = std::io::Cursor::new(json_serialized.as_bytes());

        let mut compressed: Vec<u8> = Vec::new();

        let compression_result = match self.encoding {
            CompressionMethod::gzip => GzipEncoder::new(buf_read).read_to_end(&mut compressed).await,
            CompressionMethod::deflate => DeflateEncoder::new(buf_read).read_to_end(&mut compressed).await,
            CompressionMethod::br => BrotliEncoder::new(buf_read).read_to_end(&mut compressed).await,
            CompressionMethod::zstd => ZstdEncoder::new(buf_read).read_to_end(&mut compressed).await,
            _ => panic!("Unknown compression method"),
        };

        match compression_result {
            Ok(_) => CompressedData { 
                data: compressed, 
                encoding: self.encoding
            },
            Err(e) => panic!("{:?}", e),
        }
    }
}

impl<'r> Responder<'r, 'static> for CompressedData {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        response::Response::build()
            .header(rocket::http::Header::new("Content-Encoding", format!("{:?}", self.encoding)))
            .header(rocket::http::Header::new("Content-Type", "application/json"))
            .sized_body(self.data.len(), std::io::Cursor::new(self.data))
            .ok()

    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CompressionMethod {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let accept_encoding = req.headers().get("accept-encoding").next();

        let accept_encoding = match accept_encoding {
            Some(accept_encoding) => accept_encoding.to_string(),
            None => "".to_string(),
        };

        let compresssion_method = match accept_encoding {
            _ if accept_encoding.to_lowercase().contains("gzip") => CompressionMethod::gzip,
            _ if accept_encoding.to_lowercase().contains("deflate") => CompressionMethod::deflate,
            _ if accept_encoding.to_lowercase().contains("br") => CompressionMethod::br,
            _ if accept_encoding.to_lowercase().contains("zstd") => CompressionMethod::zstd,
            _ => CompressionMethod::unknown,
        };

        Outcome::Success(compresssion_method)
    }
}

#[get("/compression")]
pub async fn compression_route(compresssion_method: CompressionMethod) -> CompressedData {
    let payload = Payload::new(compresssion_method);
    payload.compress().await
}