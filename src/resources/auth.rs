use crate::constants::MONNIFY_API_BASE_URL;
use crate::constants::MONNIFY_AUTHENTICATION_ENDPOINT;
use crate::monnify_client::client::MonnfiyClient;
use base64::Engine;
use base64::engine::general_purpose;
use serde::Deserialize;
use tracing;

#[derive(Default, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
    pub access_token: String,
    pub expires_in: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessTokenResponse {
    pub request_successful: bool,
    pub response_message: String,
    pub response_code: String,
    pub response_body: ResponseBody,
}

pub struct Auth<'a> {
    monnify_client: &'a MonnfiyClient,
}

impl<'a> Auth<'a> {
    pub fn new(monnify_client: &'a MonnfiyClient) -> Self {
        Self { monnify_client }
    }

    pub async fn generate_access_token(
        &self,
    ) -> Result<AccessTokenResponse, Box<dyn std::error::Error>> {
        let encoded_api_key = general_purpose::STANDARD.encode(format!(
            "{}:{}",
            self.monnify_client.api_key, self.monnify_client.secret_key
        ));

        let res = self
            .monnify_client
            .client
            .post(format!(
                "{}{}",
                MONNIFY_API_BASE_URL, MONNIFY_AUTHENTICATION_ENDPOINT
            ))
            .header("Authorization", format!("Basic {}", encoded_api_key))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        tracing::info!("Response status: {}", res.status());

        if res.status().is_success() {
            let response_body: AccessTokenResponse = res.json().await?;
            let jwt_token = &response_body.response_body.access_token;

            if let Ok(mut lock) = self.monnify_client.access_token.write() {
                *lock = Some(jwt_token.to_owned());
                tracing::info!("Access token generated successfully");
            }
            Ok(response_body)
        } else {
            let error_text = res.text().await?;
            Err(format!("Authentication failed: {}", error_text).into())
        }
    }
}
