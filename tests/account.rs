use nakama_rust::nakama::api::{GetUsersRequest, UpdateAccountRequest};

mod test_utils;

// Fetch self account
#[tokio::test]
async fn get_account() {
    let mut client = test_utils::make_session_client().await.unwrap();
    let response = client.get_account(()).await;
    assert!(response
        .map(|self_account| println!("Ok： {:?}", self_account))
        .map_err(|err| println!("Err： {}", err))
        .is_ok())
}

// List users by usernames, ids, facebookids
#[tokio::test]
async fn list_users() {
    let mut client = test_utils::make_session_client().await.unwrap();
    let req = GetUsersRequest {
        usernames: ["max".into()].into(),
        ..Default::default()
    };
    let response = client.get_users(req).await;
    assert!(response
        .map(|users| println!("Ok： {:?}", users))
        .map_err(|err| println!("Err： {}", err))
        .is_ok())
}

// Update self account info
#[tokio::test]
async fn update_account() {
    let mut client = test_utils::make_session_client().await.unwrap();
    let req = UpdateAccountRequest {
        display_name: Some("Demon King 😈".into()),
        ..Default::default()
    };
    let response = client.update_account(req).await;
    assert!(response
        .map(|none| println!("Ok： {:?}", none))
        .map_err(|err| println!("Err： {}", err))
        .is_ok())
}
