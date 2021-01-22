pub mod nakama;

pub mod util {

    use nakama::api::nakama_client::NakamaClient;
    use tonic::{metadata::MetadataValue, transport::*, Request};

    use crate::nakama;

    pub async fn connect_with_authrization(
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
