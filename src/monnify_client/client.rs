use crate::constants::MONNIFY_API_BASE_URL;
use crate::constants::MONNIFY_AUTHENTICATION_ENDPOINT;
use base64::Engine;
use base64::engine::general_purpose;
use reqwest;
use serde::Deserialize;

#[derive(Default, Deserialize, Debug, Clone)]
pub struct ResponseBody {
    #[serde(rename = "accessToken")]
    pub access_token: String,

    #[serde(rename = "expiresIn")]
    pub expires_in: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AccessTokenResponse {
    #[serde(rename = "requestSuccessful")]
    pub response_successful: bool,

    #[serde(rename = "responseMessage")]
    pub response_message: String,

    #[serde(rename = "responseCode")]
    pub response_code: String,

    #[serde(rename = "responseBody")]
    pub response_body: ResponseBody,
}

#[derive(Default)]
pub struct Client {
    api_key: String,
    secret_key: String,
}

impl Client {
    pub fn new(api_key: String, secret_key: String) -> Self {
        Self {
            api_key,
            secret_key,
        }
    }

    pub async fn generate_access_token(
        &self,
    ) -> Result<AccessTokenResponse, Box<dyn std::error::Error>> {
        let encoded_api_key =
            general_purpose::STANDARD.encode(format!("{}:{}", self.api_key, self.secret_key));

        let client = reqwest::Client::new();
        let res = client
            .post(format!(
                "{}{}",
                MONNIFY_API_BASE_URL, MONNIFY_AUTHENTICATION_ENDPOINT
            ))
            .header("Authorization", format!("Basic {}", encoded_api_key))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        println!("Response status: {}", res.status());

        if res.status().is_success() {
            let response_body: AccessTokenResponse = res.json().await?;
            Ok(response_body)
        } else {
            let error_text = res.text().await?;
            Err(format!("Authentication failed: {}", error_text).into())
        }
    }
}
