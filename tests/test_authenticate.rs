use monnify::monnify_client::client::Client;

#[tokio::test]
async fn test_authenticate() {
    let test_api_key: String = String::from("MK_TEST_GC3B8XG2XX");
    let test_secret_key: String = String::from("A663NRZA544DDPEM7KDN7Z8HRV6YXD8S");
    let client = Client::new(test_api_key, test_secret_key);
    let response = client.generate_access_token().await.map_err(|err| {
        eprintln!("Error: {}", err);
    });
    assert_eq!(response.unwrap().response_successful, true);
}
