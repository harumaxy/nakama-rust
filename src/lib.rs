pub mod nakama;

pub mod util {

    use crate::nakama::{self, api::Session};
    use nakama::api::nakama_client::NakamaClient;
    use tonic::{metadata::MetadataValue, transport::*, Request};

    pub async fn connect_with_server_key(
        address: Option<&'static str>,
        server_key: Option<&'static str>,
    ) -> Result<NakamaClient<Channel>, tonic::transport::Error> {
        let address = address.unwrap_or("https://[::1]:7349");
        let server_key = server_key.unwrap_or("defaultkey");
        let channel = tonic::transport::Channel::from_static(address)
            .connect()
            .await?;
        let auth_string = format!("Basic {}", base64::encode(format!("{}:", server_key)));
        let client = NakamaClient::with_interceptor(channel, move |mut req: Request<()>| {
            req.metadata_mut()
                .insert(
                    "authorization",
                    MetadataValue::from_str(auth_string.as_str()).unwrap(),
                )
                .map(|map| println!("{:?}", map));
            Ok(req)
        });
        Ok(client)
    }

    pub async fn connect_with_session_token(
        address: Option<&'static str>,
        session: &Session,
    ) -> Result<NakamaClient<Channel>, tonic::transport::Error> {
        let address = address.unwrap_or("https://[::1]:7349");
        let token = session.token.clone();
        let channel = tonic::transport::Channel::from_static(address)
            .connect()
            .await?;
        let auth_string = format!("Bearer {}", token);
        let client = NakamaClient::with_interceptor(channel, move |mut req: Request<()>| {
            req.metadata_mut()
                .insert(
                    "authorization",
                    MetadataValue::from_str(auth_string.as_str()).unwrap(),
                )
                .map(|map| println!("{:?}", map));
            Ok(req)
        });
        Ok(client)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
