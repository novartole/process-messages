mod cli;

use std::str::from_utf8;

use async_nats::ServerAddr;
use clap::Parser;
use cli::Cli;
use color_eyre::{eyre, Result};
use futures::StreamExt;
use tracing::{debug, info, trace};
use url::Url;

const NATS_WORKING_QUEUE: &str = "nats.wq";

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.instrumentation.setup()?;

    info!("Connect to NATS");

    let addr = ServerAddr::from_url(Url::parse(&format!(
        "nats://{}:{}",
        cli.nats_ip, cli.nats_port
    ))?)?;
    debug!("Info: {:#?}", addr);

    let client = async_nats::connect(addr).await?;

    while let Some(api_msg) = client.subscribe("nats.request-reply").await?.next().await {
        info!("Got a message from {}", api_msg.subject);
        debug!("Message: {:?}", api_msg);

        trace!("Send a reqeust to {}", NATS_WORKING_QUEUE);
        debug!(
            "{}",
            from_utf8(&api_msg.payload).map_or_else(
                |e| format!("Can't create string slice from request payload: {}", e),
                |s| format!("Request payload: {}", s),
            )
        );

        let res = client
            .request(NATS_WORKING_QUEUE, api_msg.payload.clone())
            .await?;

        trace!("Got a response from {}", NATS_WORKING_QUEUE);
        debug!(
            "{}",
            from_utf8(&api_msg.payload).map_or_else(
                |e| format!("Can't create string slice from response payload: {}", e),
                |s| format!("Response payload: {}", s),
            )
        );

        info!("Publish the response to {}", api_msg.reply.clone().unwrap());
        client
            .publish(
                api_msg
                    .reply
                    .ok_or_else(|| eyre::eyre!("No reply for publish found"))?,
                res.payload,
            )
            .await?;
    }

    Ok(())
}
