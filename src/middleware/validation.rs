use std::env;

use async_trait::async_trait;
use lazy_regex::lazy_regex;
use lazy_regex::Lazy;
use lazy_regex::Regex;
use reqwest_middleware::ClientBuilder;
use reqwest_middleware::ClientWithMiddleware;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use reqwest::Client;
use reqwest::Response;
use serde::Deserialize;

use crate::models::errors::Error;

const APILAYER_KEY_KEY: &str = "APILAYER_KEY";
const APILAYER_BASE_URL: &str = "https://api.apilayer.com/number_verification/validate?number=";

#[async_trait]
trait Validation {
    fn is_name_valid(name: String) -> bool;

    fn is_email_valid(&self, email: String) -> bool;

    fn is_phone_no_valid_fallback(&self, phone_no: i64) -> bool;

    async fn get_valid_phone_no(&self, phone_no: i64) -> Result<bool, Error>;
}

struct ValidationMiddleware {
    email_regex: Lazy<Regex>,
    de_phone_regex: Lazy<Regex>,
    http_client: ClientWithMiddleware,
    api_layer_key: String,
}

impl ValidationMiddleware {
    fn new() -> Self {
        let api_layer_key: String = env::var(APILAYER_KEY_KEY)
            .unwrap_or_else(|_| panic!("Missing environment variable: {APILAYER_KEY_KEY}"));

        let retry_policy: ExponentialBackoff =
            ExponentialBackoff::builder().build_with_max_retries(3);

        ValidationMiddleware {
            email_regex: lazy_regex!(
                r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})"
            ),
            de_phone_regex: lazy_regex!(r"49[0-9]{9,10}"),
            http_client: ClientBuilder::new(Client::new())
                .with(RetryTransientMiddleware::new_with_policy(retry_policy))
                .build(),
            api_layer_key: api_layer_key,
        }
    }
}

impl Default for ValidationMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Validation for ValidationMiddleware {
    fn is_name_valid(name: String) -> bool {
        name.is_empty() || name.len() > 255
    }

    fn is_email_valid(&self, email: String) -> bool {
        self.email_regex.is_match(&email)
    }

    fn is_phone_no_valid_fallback(&self, phone_no: i64) -> bool {
        self.de_phone_regex.is_match(&phone_no.to_string())
    }

    async fn get_valid_phone_no(&self, phone_no: i64) -> Result<bool, Error> {
        let url: String = format!("{APILAYER_BASE_URL}{phone_no}");
        let result: Response = self
            .http_client
            .get(url)
            .header("apikey", self.api_layer_key.clone())
            .send()
            .await?;

        if result.status().is_success() {
            result
                .json::<NumberVerificationAPIResponse>()
                .await
                .map(|typed_response: NumberVerificationAPIResponse| {
                    typed_response.valid && typed_response.country_code == "DE"
                })
                .map_err(|err: reqwest::Error| Error::ReqwestMiddleware(err.to_string()))
        } else {
            if result.status().is_client_error() {
                Err(Error::ReqwestMiddleware(format!(
                    "Unexpected status_code: {}",
                    result.status().as_str()
                )))
            } else {
                Ok(self.is_phone_no_valid_fallback(phone_no))
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct NumberVerificationAPIResponse {
    valid: bool,
    number: String,
    local_format: String,
    international_format: String,
    country_prefix: String,
    country_code: String,
    country_name: String,
    location: String,
    carrier: String,
    line_type: String,
}
