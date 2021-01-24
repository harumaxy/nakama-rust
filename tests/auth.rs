use nakama_rust_gen::nakama::api::{
    nakama_client::NakamaClient, AccountDevice, AccountEmail, AuthenticateDeviceRequest,
    AuthenticateEmailRequest, Session,
};
use tonic::{transport::Channel, Request, Response, Status};

pub async fn email_authenticate(
    client: &mut NakamaClient<Channel>,
) -> Result<Response<Session>, Status> {
    let email_auth_request = Request::new(AuthenticateEmailRequest {
        create: Some(true),
        account: Some(AccountEmail {
            email: "harumaxy@gmail.com".into(),
            password: "passw0rd".into(),
            ..Default::default()
        }),
        username: "max".into(),
    });

    let response = client.authenticate_email(email_auth_request).await;
    response
}

#[tokio::test]
async fn email_authenticate_test() {
    let mut client = nakama_rust_gen::util::connect_with_server_key(None, None)
        .await
        .unwrap();
    let response = email_authenticate(&mut client).await;

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
