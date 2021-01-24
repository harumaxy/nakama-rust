use nakama_rust_gen::nakama::api::{AccountDevice, AuthenticateDeviceRequest};
use tonic::Request;

mod test_utils;

#[tokio::test]
async fn email_authenticate_test() {
    let mut client = nakama_rust_gen::util::connect_with_server_key(None, None)
        .await
        .unwrap();
    let response = test_utils::email_authenticate(&mut client).await;

    assert!(response
        .map(|session| println!("Ok： {:?}", session))
        .map_err(|err| println!("Err： {}", err))
        .is_ok())
}

#[tokio::test]
async fn device_authenticate() {
    let id = uuid::Uuid::new_v4().to_string();
    println!("{}", id);
    let device_auth_request = Request::new(AuthenticateDeviceRequest {
        create: Some(true),
        username: id.clone(),
        account: Some(AccountDevice {
            id,
            ..Default::default()
        }),
    });

    let mut client = nakama_rust_gen::util::connect_with_server_key(None, None)
        .await
        .unwrap();
    let response = client.authenticate_device(device_auth_request).await;

    assert!(response
        .map(|session| println!("Ok： {:?}", session))
        .map_err(|err| println!("Err： {}", err))
        .is_ok())
}
