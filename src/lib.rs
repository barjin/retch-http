use std::collections::HashMap;
use std::str::FromStr;

use header_generator::{header_generator::generate_headers};
mod header_generator;

use napi::bindgen_prelude::Buffer;
use napi::Error;
use reqwest::header::HeaderName;
use url::Url;

#[macro_use]
extern crate napi_derive;

#[napi(string_enum)]
pub enum Browser {
  Firefox,
  Chrome,
}

#[napi(string_enum)]
pub enum OperatingSystem {
  Windows,
  Linux,
  MacOS,
  Android,
  IOS,
}

#[napi(object)]
pub struct EngineOptions{
  pub browser: Option<Browser>,
  pub ignore_tls_errors: Option<bool>,
}

#[napi(object)]
pub struct FetchOptions{
  #[napi(object)]
  pub headers: HashMap<String, String>
}

#[napi(object)]
pub struct FetchResponse {
  pub body: Option<Buffer>,
  pub body_used: bool,
  pub headers: HashMap<String, String>,
  pub ok: bool,
  pub redirected: bool,
  pub status: u16,
  pub status_text: String,
  pub r#type: String,
  pub url: String,
}

#[napi]
pub struct Retcher {
  engine: reqwest::Client,
  browser: Browser,
}
 
#[napi]
impl Retcher {
  #[napi(constructor)]
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

  #[napi]
  pub async fn retch(&self, url: String, options: Option<FetchOptions>) -> Result<FetchResponse, Error> {
    self.get(url, options).await
  }

  async fn get(&self, url: String, options: Option<FetchOptions>) -> Result<FetchResponse, Error> {
    let url = Url::parse(&url).unwrap();
    let host = url.domain().unwrap();
    let protocol = url.scheme();

    let custom_headers = match options {
      Some(options) => options.headers,
      None => HashMap::new(),
    };

    let protocol_error: Option<Error> = match protocol {
      "http" => None,
      "https" => None,
      _ => Some(Error::new(napi::Status::InvalidArg, format!("{protocol} is not a valid HTTP protocol."))),
    };

    if protocol_error.is_some() {
      return Err(protocol_error.unwrap());
    }

    let mut headers = generate_headers(&host.to_string(), self.browser, protocol == "https");

    for (key, value) in custom_headers.iter() {
      headers.insert(HeaderName::from_str(key).unwrap(), value.parse().unwrap());
    }

    let response = self.engine.get(url)
      .headers(headers)
      .send()
      .await;

    if response.is_err() {
      return Err(Error::new(napi::Status::GenericFailure, format!("{:?}", response.unwrap_err())));
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
      return Err(Error::new(napi::Status::GenericFailure, format!("{:?}", body.unwrap_err())));
    }

    let body = body.unwrap();
    if body.is_empty() {
      return Ok(result)
    } else {
      result.body = Some(Buffer::from(body.to_vec()));
      return Ok(result)
    }
  }
}
