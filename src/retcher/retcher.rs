use std::collections::HashMap;
use std::str::FromStr;

use crate::header_generator::header_generator::HeaderGeneratorOptions;

use super::super::header_generator::header_generator::generate_headers;

use reqwest::header::HeaderName;
use url::Url;

#[derive(PartialEq, Debug, Clone)]
pub enum Browser {
  Firefox,
  Chrome,
}

/// EngineOptions is a struct holding additional options for the engine.
/// 
/// These are used globally for all requests made with the given `Retcher` instance.
pub struct EngineOptions{
  /// An optional `Browser` enum that holds the browser to impersonate.
  pub browser: Option<Browser>,
  /// An optional `bool` that holds whether to ignore TLS errors.
  pub ignore_tls_errors: Option<bool>,
}

/// FetchOptions is a struct holding additional options for the fetch request.
pub struct FetchOptions{
  /// A `HashMap` that holds custom HTTP headers. These are added to the default headers and should never overwrite them.
  pub headers: HashMap<String, String>
}

pub struct FetchResponse {
  pub body: Option<Vec<u8>>,
  pub body_used: bool,
  pub headers: HashMap<String, String>,
  pub ok: bool,
  pub redirected: bool,
  pub status: u16,
  pub status_text: String,
  pub r#type: String,
  pub url: String,
}

#[derive(Debug, Clone)]
pub struct FetchError {
  message: String,
}

/// Retcher is the main struct used to make (impersonated) requests.
/// 
/// It uses `reqwest::Client` to make requests and holds info about the impersonated browser.
pub struct Retcher {
  engine: reqwest::Client,
  /// A `Browser` enum that holds the browser to impersonate.
  pub browser: Browser,
}

impl Retcher {
  /// Creates a new `Retcher` instance with the given `EngineOptions`.
  pub fn new(options: EngineOptions) -> Self {
    let mut engine = reqwest::ClientBuilder::new()
      .http1_title_case_headers();

    if options.ignore_tls_errors.unwrap_or(false) {
      engine = engine
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true);
    }

    Retcher { 
      engine: engine.build().unwrap(), 
      browser: options.browser.unwrap_or(Browser::Firefox),
    }
  }

  /// Calling `retch` with an URL and optional options will make a request to the URL and return a `FetchResponse`.
  /// 
  /// The API is supposed to follow the `fetch` API in JavaScript as closely as possible.
  /// 
  /// # Arguments
  /// 
  /// * `url` - A `String` that holds the URL to make a request to.
  /// * `options` - An optional `FetchOptions` struct that holds additional options for the request.
  pub async fn retch(&self, url: String, options: Option<FetchOptions>) -> Result<FetchResponse, FetchError> {
    // TODO!!! Implement other HTTP methods
    self.get(url, options).await
  }

  /// Calling `get` with an URL and optional options will make a request to the URL and return a `FetchResponse`.
  async fn get(&self, url: String, options: Option<FetchOptions>) -> Result<FetchResponse, FetchError> {
    let url = Url::parse(&url).unwrap();

    let host = url.host_str().unwrap();
    let protocol = url.scheme();

    let custom_headers = match options {
      Some(options) => options.headers,
      None => HashMap::new(),
    };

    let protocol_error: Option<FetchError> = match protocol {
      "http" => None,
      "https" => None,
      _ => Some(FetchError{
        message: "Unsupported protocol".to_string(),
      }),
    };

    if protocol_error.is_some() {
      return Err(protocol_error.unwrap());
    }

    let mut headers = generate_headers(HeaderGeneratorOptions {
      host: host.to_string(), 
      browser: self.browser.clone(), 
      https: protocol == "https",
      custom_headers: Some(custom_headers.clone()),
    });

    for (key, value) in custom_headers.iter() {
      headers.insert(HeaderName::from_str(key).unwrap(), value.parse().unwrap());
    }

    let response = self.engine.get(url)
      .headers(headers)
      .send()
      .await;

    if response.is_err() {
      return Err(FetchError{
        message: format!("{:?}", response.err().unwrap()),
      });
    }

    let response = response.unwrap();

    let mut headers = HashMap::new();

    for (key, value) in response.headers().iter() {
      headers.insert(key.to_string(), value.to_str().unwrap().to_string());
    }

    let mut result = FetchResponse {
      body: None,
      body_used: true,
      headers: headers,
      ok: response.status().is_success(),
      redirected: response.status().is_redirection(),
      status: response.status().as_u16(),
      status_text: response.status().canonical_reason().unwrap().to_string(),
      url: response.url().to_string(),
      r#type: "basic".to_string(),
    };

    let body = response.bytes().await;

    if body.is_err() {
      return Err(FetchError {
        message: format!("{:?}", body.err().unwrap()),
      });
    }

    let body = body.unwrap();
    if body.is_empty() {
      return Ok(result)
    } else {
      result.body = Some(body.into());
      return Ok(result)
    }
  }
}
