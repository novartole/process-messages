mod cli;

use std::{str::from_utf8, sync::Arc};

use crate::cli::Cli;
use async_nats::{jetstream, Client, ServerAddr};
use clap::Parser;
use color_eyre::{eyre, Result};
use command::Command;
use futures::StreamExt;
use tokio::sync::RwLock;
use tracing::{debug, info, trace};
use url::Url;

const NATS_WQ: &str = "nats.wq";
const NATS_QUEUE_GROUP: &str = "QUEUE_WORKERS";

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

    let mut subscribtion = client
        .queue_subscribe(NATS_WQ, NATS_QUEUE_GROUP.into())
        .await?;

    let rw_command = Arc::new(RwLock::new(Command::default()));
    let worker_id = Arc::new(cli.worker_id);

    spawn_command_processor(
        client.clone(),
        Arc::clone(&rw_command),
        Arc::clone(&worker_id),
    )
    .await;

    info!("Start worker #{}", worker_id);
    debug!(
        "Subscribed to '{}' with group '{}'",
        NATS_WQ, NATS_QUEUE_GROUP
    );

    while let Some(msg) = subscribtion.next().await {
        info!("Got a request from {}", msg.subject);
        debug!("Request: {:?}", msg);

        trace!("Process message");
        debug!(
            "{}",
            from_utf8(&msg.payload).map_or_else(
                |e| format!("Can't create string slice from message payload: {}", e),
                |s| format!("Message: {}", s),
            )
        );

        let res = rw_command
            .read()
            .await
            .call_on(from_utf8(&msg.payload)?.to_string());

        trace!("Got a result");
        debug!("Result {}", res);

        info!("Publish the result to {}", msg.reply.clone().unwrap());
        client
            .publish(
                msg.reply
                    .ok_or_else(|| eyre::eyre!("No reply for publish found"))?,
                res.into(),
            )
            .await?;
    }

    Ok(())
}

async fn spawn_command_processor(
    client: Client,
    rw_command: Arc<RwLock<Command>>,
    worker_id: Arc<String>,
) {
    info!("Start command processor");

    tokio::spawn(async move {
        let jetstream = jetstream::new(client);

        let mut stream = jetstream
            .create_stream(jetstream::stream::Config {
                name: "COMMANDS".to_string(),
                retention: jetstream::stream::RetentionPolicy::Limits,
                subjects: vec!["nats.fnf".to_string()],
                max_messages: 1,
                ..Default::default()
            })
            .await?;
        trace!("Stream created");
        debug!("Stream info: {:#?}", stream.info().await?);

        let mut consumer = stream
            .create_consumer(jetstream::consumer::pull::Config {
                durable_name: Some(format!("processor-fnf-{}", worker_id)),
                ..Default::default()
            })
            .await?;
        trace!("Consumer created");
        debug!("Consumer info: {:#?}", consumer.info().await?);

        info!("Processor-fnf spawned");

        while let Some(Ok(msg)) = consumer.messages().await?.next().await {
            info!("Processor-fnf got a message");
            debug!("Message: {:?}", msg);

            let mut command = rw_command.write().await;
            let msg_command = serde_json::from_slice::<Command>(&msg.payload)?;
            if command.ne(&msg_command) {
                trace!("Update command");
                debug!("Old: {:?}, new: {:?}", command, msg_command);

                *command = msg_command;
            } else {
                trace!("No update needed");
            }

            info!("Processor-fnf processed the message");
        }

        Ok::<(), async_nats::Error>(())
    });
}
