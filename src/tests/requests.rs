use std::{collections::HashMap, iter::zip};

use crate::retcher::retcher::{Browser, EngineOptions, FetchOptions, Retcher};
use super::server::{get_server, request_headers::RequestHeaders};

#[tokio::test]
async fn default_requests() {
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
async fn default_http_headers() {
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

static CUSTOM_USER_AGENT: &str = "Custom-User-Agent!";
static CUSTOM_ACCEPT_LANGUAGE: &str = "cs-CZ";
static CUSTOM_RANDOM_HEADER: (&str, &str) = ("random-header", "random-header-value");

#[tokio::test]
async fn custom_http_headers() {
    get_server().await;

    let retcher = Retcher::new(EngineOptions {
        browser: Some(Browser::Chrome),
        ignore_tls_errors: Some(true),
    });

    let custom_headers: Vec<(String, String)> = vec![
        ("User-Agent".into(), CUSTOM_USER_AGENT.to_string()),
        ("Accept-Language".into(), "cs-CZ".into()),
        (CUSTOM_RANDOM_HEADER.0.to_string(), CUSTOM_RANDOM_HEADER.1.to_string()),
    ];

    let response = retcher.retch("http://127.0.0.1:8000/headers".into(), Some(FetchOptions{
        headers: HashMap::from_iter(custom_headers.into_iter()),
    })).await;

    let headers: RequestHeaders = match response {
        Ok(response) => serde_json::from_str(String::from_utf8(response.body.unwrap()).unwrap().as_str()).unwrap(),
        Err(e) => panic!("{:?}", e),
    };

    let expected = vec![
        ("host", "127.0.0.1"),
        ("connection", "keep-alive"),
        ("upgrade-insecure-requests", "1"),
        ("user-agent", CUSTOM_USER_AGENT),
        ("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"),
        ("accept-encoding", "gzip, deflate"),
        ("accept-language", CUSTOM_ACCEPT_LANGUAGE),
        (CUSTOM_RANDOM_HEADER.0, CUSTOM_RANDOM_HEADER.1),
    ];

    assert_eq!(headers.0.len(), expected.len());

    for pair in zip(headers.0, expected) {
        let (header, (key, value)) = pair;
        assert_eq!(header.0, key);
        assert_eq!(header.1, value);
    }
}
