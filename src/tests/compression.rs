use std::{collections::HashMap, iter::zip};

use serde_json::json;

use crate::retcher::retcher::{Browser, EngineOptions, FetchOptions, Retcher};
use super::server::{get_server, request_headers::RequestHeaders};
use super::server::compression::{Payload, BODY, CompressionMethod};


macro_rules! compression_tests {
    ($($name:ident,)*) => {
    $(
        #[tokio::test]
        async fn $name() {
            get_server().await;

            let retcher = Retcher::new(EngineOptions {
                browser: Some(Browser::Chrome),
                ignore_tls_errors: Some(true),
            });
        
            let response = retcher.retch("http://127.0.0.1:8000/compression".into(), Some(FetchOptions{
                headers: HashMap::from_iter(vec![
                    ("accept-encoding".to_string(), format!("{:?}", CompressionMethod::$name)),
                ]),
            })).await;
        
            let response = match response {
                Ok(response) => response.body.unwrap_or(Vec::new()),
                Err(e) => panic!("{:?}", e),
            };
        
            let response = match String::from_utf8(response) {
                Ok(response) => response,
                Err(e) => panic!("{:?}", e),
            };
        
            let expected = Payload {
                body: BODY,
                encoding: CompressionMethod::$name,
            };
        
            assert_eq!(response, json!(expected).to_string());
        }
    )*
    }
}

compression_tests! {
    gzip,
    // deflate, -- TODO!!! - Fix deflate compression
    br,
    zstd,
}
