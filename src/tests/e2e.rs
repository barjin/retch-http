use super::super::retcher::retcher::{Retcher, Browser, EngineOptions};

#[tokio::test]
async fn e2e_request() {
    let retcher = Retcher::new(EngineOptions {
        browser: Some(Browser::Chrome),
        ignore_tls_errors: Some(true),
    });

    let response = retcher.retch("https://www.example.com/".to_string(), None).await.unwrap();

    assert_eq!(response.ok, true);
    assert_eq!(response.status, 200);
    assert_eq!(response.body_used, true);
    assert_eq!(response.body.is_some(), true);
    assert_eq!(response.body.unwrap().len() > 0, true);
    assert_eq!(response.headers.len() > 0, true);
    assert_eq!(response.url, "https://www.example.com/");
    assert_eq!(response.r#type, "basic");
    assert_eq!(response.redirected, false);
    assert_eq!(response.status_text, "OK");
}
