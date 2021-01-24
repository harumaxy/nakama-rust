use nakama_rust_gen::nakama::api::Session;
use nakama_rust_gen::{
    nakama::api::{nakama_client::NakamaClient, AccountEmail, AuthenticateEmailRequest},
    util::connect_with_session_token,
};
use tonic::transport::Channel;
use tonic::{Request, Response, Status};

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

pub async fn make_session_client() -> Result<NakamaClient<Channel>, tonic::transport::Error> {
    let mut client = nakama_rust_gen::util::connect_with_server_key(None, None)
        .await
        .unwrap();
    let session = email_authenticate(&mut client).await.unwrap().into_inner();
    connect_with_session_token(None, &session).await
}
