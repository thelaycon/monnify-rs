use crate::constants::MONNIFY_API_BASE_URL;
use crate::constants::MONNIFY_INIT_TRANSACTION_ENDPOINT;
use crate::monnify_client::client::MonnfiyClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct InitializeTransactionRequest {
    pub amount: i64,
    #[serde(rename = "customerEmail")]
    pub customer_email: String,
    #[serde(rename = "paymentReference")]
    pub payment_reference: String,
    #[serde(rename = "paymentDescription")]
    pub payment_description: String,
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    #[serde(rename = "redirectUrl")]
    pub redirect_url: String,
    #[serde(rename = "contractCode")]
    pub contract_code: String,
    #[serde(rename = "paymentMethods")]
    pub payment_methods: Vec<String>,
    pub metadata: Option<HashMap<String, Value>>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseBody {
    #[serde(rename = "transactionReference")]
    pub transaction_reference: String,
    #[serde(rename = "paymentReference")]
    pub payment_reference: String,
    #[serde(rename = "merchantName")]
    pub merchant_name: String,
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "redirectUrl")]
    pub redirect_url: String,
    #[serde(rename = "enabledPaymentMethod")]
    pub enabled_payment_method: Vec<String>,
    #[serde(rename = "checkoutUrl")]
    pub checkout_url: String,
}

#[derive(Debug, Deserialize)]
pub struct InitializeTransactionResponse {
    #[serde(rename = "requestSuccessful")]
    pub request_successful: bool,
    #[serde(rename = "responseMessage")]
    pub response_message: String,
    #[serde(rename = "responseCode")]
    pub response_code: String,
    #[serde(rename = "responseBody")]
    pub response_body: ResponseBody,
}

pub struct Transaction<'a> {
    monnify_client: &'a MonnfiyClient,
}

impl<'a> Transaction<'a> {
    pub fn new(monnify_client: &'a MonnfiyClient) -> Self {
        Transaction { monnify_client }
    }

    pub async fn initialize_transaction(
        &self,
        request: InitializeTransactionRequest,
    ) -> Result<InitializeTransactionResponse, Box<dyn std::error::Error>> {
        let initialize_url: String = format!(
            "{}{}",
            MONNIFY_API_BASE_URL, MONNIFY_INIT_TRANSACTION_ENDPOINT
        );

        let token_guard = self
            .monnify_client
            .access_token
            .read()
            .map_err(|_| "Failed to acquire access token lock")?;

        let token = match &*token_guard {
            Some(token) => {
                println!("Access token found");
                token.clone()
            }
            None => {
                println!("Access token not found");
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Access token not found",
                )));
            }
        };

        // Make the API request to initialize transaction
        let response = self
            .monnify_client
            .client
            .post(initialize_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let response_body: InitializeTransactionResponse = response.json().await?;
            Ok(response_body)
        } else {
            tracing::error!("Failed to initialize transaction");
            let error_response: serde_json::Value = response.json().await?;
            println!("Response Status: {:?}", error_response);
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to initialize transaction",
            )))
        }
    }
}
