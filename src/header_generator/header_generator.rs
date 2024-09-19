use std::str::FromStr;
use reqwest::header::{HeaderMap, HeaderName};
use super::super::Browser;

#[derive(Default)]
struct Header {
    key: String,
    value: String,
    is_https: Option<bool>,
    is_http1: Option<bool>
}

pub fn generate_headers(host: &String, browser: Browser, https: bool) -> HeaderMap {
    let firefox_headers: Vec<Header> = vec![
        Header { key: "Host".into(), value: host.as_str().into(), is_http1: Some(true), ..Header::default() },
        Header { key: "User-Agent".into(), value: "Mozilla/5.0 (X11; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0".into(), ..Header::default() }, 
        Header { key: "Accept".into(), value: "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/png,image/svg+xml,*/*;q=0.8".into(), ..Header::default() }, 
        Header { key: "Accept-Language".into(), value: "en,cs;q=0.7,en-US;q=0.3".into(), ..Header::default() }, 
        Header { key: "Accept-Encoding".into(), value: "gzip, deflate".into(), ..Header::default() }, 
        Header { key: "sec-fetch-dest".into(), value: "document".into(), is_https: Some(true), ..Header::default() }, 
        Header { key: "sec-fetch-mode".into(), value: "navigate".into(), is_https: Some(true), ..Header::default() }, 
        Header { key: "sec-fetch-site".into(), value: "none".into(), is_https: Some(true), ..Header::default() }, 
        Header { key: "sec-fetch-user".into(), value: "?1".into(), is_https: Some(true), ..Header::default() }, 
        Header { key: "Connection".into(), value: "keep-alive".into(), ..Header::default() }, 
        Header { key: "Upgrade-Insecure-Requests".into(), value: "1".into(), ..Header::default() }, 
        Header { key: "Priority".into(), value: "u=0, i".into(), ..Header::default() },
    ];

    // [TODO!]
    // Note that not all requests are made the same:
    //  - on forced (Ctrl+R) reloads, Chrome sets Cache-Control: max-age=0
    //  - when the URL is in the address bar (but not submitted yet), Chrome sets `Purpose: prefetch` and `Sec-Purpose: prefetch`
    let chrome_headers: Vec<Header> = vec![
        Header { key: "sec-ch-ua".into(), value: "\"Google Chrome\";v=\"125\", \"Chromium\";v=\"125\", \"Not.A/Brand\";v=\"24\"".into(), is_https: Some(true), ..Header::default() },
        Header { key: "sec-ch-ua-mobile".into(), value: "?0".into(), is_https: Some(true), ..Header::default() },
        Header { key: "sec-ch-ua-platform".into(), value: "Linux".into(), is_https: Some(true), ..Header::default() },
        Header { key: "Host".into(), value: host.into(), is_http1: Some(true), ..Header::default() },
        Header { key: "Connection".into(), value: "keep-alive".into(), ..Header::default() },
        Header { key: "Upgrade-Insecure-Requests".into(), value: "1".into(), ..Header::default() },
        Header { key: "User-Agent".into(), value: "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36".into(), ..Header::default() },
        Header { key: "Accept".into(), value: "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".into(), ..Header::default() },
        Header { key: "sec-fetch-site".into(), value: "none".into(), is_https: Some(true), ..Header::default() }, 
        Header { key: "sec-fetch-mode".into(), value: "navigate".into(), is_https: Some(true), ..Header::default() }, 
        Header { key: "sec-fetch-user".into(), value: "?1".into(), is_https: Some(true), ..Header::default() }, 
        Header { key: "sec-fetch-dest".into(), value: "document".into(), is_https: Some(true), ..Header::default() }, 
        Header { key: "Accept-Encoding".into(), value: "gzip, deflate".into(), ..Header::default() },
        Header { key: "Accept-Language".into(), value: "en-US,en;q=0.9".into(), ..Header::default() },
    ];

    let mut headers = HeaderMap::new();

    let source_headers = match browser {
        Browser::Chrome => chrome_headers,
        _ => firefox_headers, // Default to Firefox
    };

    for Header { key, value, is_https, is_http1 } in source_headers.iter() {
        if is_https.is_some() && !https {
            continue;
        }

        // [TODO!!] - HTTPS != !HTTP1
        if is_http1.is_some() && https {
            continue;
        }

        headers.insert(HeaderName::from_str(key).unwrap(), value.parse().unwrap());
    }

    headers
}
