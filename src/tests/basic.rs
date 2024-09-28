use super::super::retcher::retcher::{Retcher, Browser, EngineOptions};

#[test]
fn browser_param_passing() {
    let retcher = Retcher::new(EngineOptions {
        browser: Some(Browser::Chrome),
        ignore_tls_errors: None,
    });

    assert_eq!(retcher.browser, Browser::Chrome);
    
    let retcher = Retcher::new(EngineOptions {
        browser: Some(Browser::Firefox),
        ignore_tls_errors: None,
    });

    assert_eq!(retcher.browser, Browser::Firefox);
}
