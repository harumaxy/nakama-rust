mod test_utils;

// fetch self account
#[tokio::test]
async fn get_account() {
    let mut client = test_utils::make_session_client().await.unwrap();
    let response = client.get_account(()).await;
    assert!(response
        .map(|account_response| println!("Ok： {:?}", account_response))
        .map_err(|err| println!("Err： {}", err))
        .is_ok())
}
