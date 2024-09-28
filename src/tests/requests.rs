use std::iter::zip;

use crate::retcher::retcher::{Browser, EngineOptions, Retcher};
use super::server::{get_server, RequestHeaders};

#[tokio::test]
async fn requests_pass() {
    get_server().await;

    let retcher = Retcher::new(EngineOptions {
        browser: Some(Browser::Chrome),
        ignore_tls_errors: Some(true),
    });

    let response = retcher.retch("http://127.0.0.1:8000".into(), None).await;
    
    let body = match response {
        Ok(response) => response.ok,
        Err(e) => panic!("{:?}", e),
    };

    assert_eq!(body, true);
}

#[tokio::test]
async fn http_headers_pass() {
    get_server().await;

    let retcher = Retcher::new(EngineOptions {
        browser: Some(Browser::Chrome),
        ignore_tls_errors: Some(true),
    });

    let response = retcher.retch("http://127.0.0.1:8000/headers".into(), None).await;

    let headers: RequestHeaders = match response {
        Ok(response) => serde_json::from_str(String::from_utf8(response.body.unwrap()).unwrap().as_str()).unwrap(),
        Err(e) => panic!("{:?}", e),
    };

    let expected = vec![
        ("host", "127.0.0.1"),
        ("connection", "keep-alive"),
        ("upgrade-insecure-requests", "1"),
        ("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"),
        ("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"),
        ("accept-encoding", "gzip, deflate"),
        ("accept-language", "en-US,en;q=0.9"),
    ];

    assert_eq!(headers.0.len(), expected.len());

    for pair in zip(headers.0, expected) {
        let (header, (key, value)) = pair;
        assert_eq!(header.0, key);
        assert_eq!(header.1, value);
    }
}
