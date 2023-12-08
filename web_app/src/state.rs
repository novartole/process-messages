use std::str::from_utf8;

use crate::error::Result;
use async_nats::{jetstream, ServerAddr};

use command::Command;
use tracing::{info, debug};

const NATS_REQUEST_REPLY: &str = "nats.request-reply";
const NATS_FNF: &str = "nats.fnf";

pub struct Nats {
    client: async_nats::client::Client,
    jetstream: async_nats::jetstream::Context,
}

impl Nats {
    pub async fn new(addr: ServerAddr) -> Result<Self> {
        info!("Connect to NATS");
        debug!("Info: {:#?}", addr);
        let client = async_nats::connect(addr).await?;

        let jetstream = jetstream::new(client.clone());

        Ok(Self { client, jetstream })
    }

    pub async fn request(&self, message: String) -> Result<String> {
        info!("Send a request to {}", NATS_REQUEST_REPLY);
        debug!("Request payload: {}", message);

        let res = from_utf8(
            &self
                .client
                .request(NATS_REQUEST_REPLY, message.into())
                .await?
                .payload,
        )?
        .to_string();

        info!("Got a response from {}", NATS_REQUEST_REPLY);
        debug!("Response payload: {}", res);

        Ok(res)
    }

    pub async fn publish(&self, command: Command) -> Result<()> {
        info!("Publishing to {}", NATS_FNF);
        debug!("Message payload: {:?}", command);

        self.jetstream
            .publish(
                NATS_FNF,
                serde_json::to_vec(&serde_json::json!(&command))?.into(),
            )
            .await?
            .await?;

        info!("Published to {}", NATS_FNF);

        Ok(())
    }
}
