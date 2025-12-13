use crate::constants::MONNIFY_API_BASE_URL;
use crate::constants::{MONNIFY_INIT_BANK_TRANSFER_ENDPOINT, MONNIFY_INIT_TRANSACTION_ENDPOINT};
use crate::monnify_client::client::MonnfiyClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeTransactionRequest {
    pub amount: i64,
    pub customer_email: String,
    pub payment_reference: String,
    pub payment_description: String,
    pub currency_code: String,
    pub redirect_url: String,
    pub contract_code: String,
    pub payment_methods: Vec<String>,
    pub metadata: Option<HashMap<String, Value>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeTransactionResponseBody {
    pub transaction_reference: String,
    pub payment_reference: String,
    pub merchant_name: String,
    pub api_key: String,
    pub redirect_url: String,
    pub enabled_payment_method: Vec<String>,
    pub checkout_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeTransactionResponse {
    pub request_successful: bool,
    pub response_message: String,
    pub response_code: String,
    pub response_body: InitializeTransactionResponseBody,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayWithBankTransferRequest {
    pub transaction_reference: String,
    pub bank_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayWithBankAccountResponse {
    pub request_successful: bool,
    pub response_message: String,
    pub response_code: String,
    pub response_body: PayWithBankAccountBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayWithBankAccountBody {
    pub account_number: String,
    pub account_name: String,
    pub bank_name: String,
    pub bank_code: String,
    pub account_duration_seconds: u32,
    pub ussd_payment: String,
    pub request_time: String,
    pub expires_on: String,
    pub transaction_reference: String,
    pub payment_reference: String,
    pub amount: f64,
    pub fee: f64,
    pub total_payable: f64,
    pub collection_channel: String,
    pub product_information: Option<String>,
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

        let token = self.monnify_client.get_access_token();
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

    pub async fn pay_with_bank_transfer(
        &self,
        pay_with_bank_transfer_request: PayWithBankTransferRequest,
    ) -> Result<PayWithBankAccountResponse, Box<dyn std::error::Error>> {
        // Make requests to pay with bank transfer. Bank code and payment reference from initialize transacton is needed
        let bank_payment_endpoint: String = format!(
            "{}{}",
            MONNIFY_API_BASE_URL, MONNIFY_INIT_BANK_TRANSFER_ENDPOINT
        );

        let token = self.monnify_client.get_access_token();

        let response = self
            .monnify_client
            .client
            .post(bank_payment_endpoint)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token))
            .json(&pay_with_bank_transfer_request)
            .send()
            .await?;

        if response.status().is_success() {
            let response_body: PayWithBankAccountResponse = response.json().await?;
            Ok(response_body)
        } else {
            tracing::error!("Failed to make bank transfer");
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to make bank transfer",
            )))
        }
    }
}
