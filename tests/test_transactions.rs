use monnify::monnify_client::client::MonnfiyClient;
use monnify::resources::transactions::InitializeTransactionRequest;
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::test]
async fn test_initialize_trasaction() {
    let test_api_key: String = String::from("MK_TEST_GC3B8XG2XX");
    let test_secret_key: String = String::from("A663NRZA544DDPEM7KDN7Z8HRV6YXD8S");
    let client = MonnfiyClient::new(test_api_key, test_secret_key);
    let _response = client.auth().generate_access_token().await.map_err(|err| {
        eprintln!("Error: {}", err);
    });
    let transaction = InitializeTransactionRequest {
        amount: 1000,
        customer_email: String::from("test@example.com"),
        payment_reference: Uuid::new_v4().simple().to_string(),
        payment_description: String::from("Test Payment"),
        currency_code: String::from("NGN"),
        contract_code: String::from("5867418298"),
        redirect_url: String::from("https://example.com/redirect"),
        payment_methods: vec![String::from("CARD"), String::from("ACCOUNT_TRANSFER")],
        metadata: Some(HashMap::new()),
    };

    println!("Transaction Request: {:?}", transaction);

    let response = client
        .transaction()
        .initialize_transaction(transaction)
        .await
        .map_err(|err| {
            println!("Error: {}", err);
        });

    assert_eq!(response.unwrap().request_successful, true);
}
