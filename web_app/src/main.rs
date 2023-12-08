mod cli;
mod error;
mod route;
mod state;
mod trace_layer;

use std::{io::IsTerminal, net::SocketAddr, sync::Arc};

use async_nats::ServerAddr;
use axum::{routing::post, Router};
use clap::Parser;
use cli::Cli;
use error::Result;
use state::Nats;
use tower_http::trace::TraceLayer;
use tracing::{trace, info};
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    trace!("Setup color_eyre");
    color_eyre::config::HookBuilder::default()
        .theme(if !std::io::stderr().is_terminal() {
            color_eyre::config::Theme::new()
        } else {
            color_eyre::config::Theme::dark()
        })
        .install()?;

    trace!("Setup cli");
    let cli = Cli::parse();
    cli.instrumentation.setup()?;

    trace!("Setup app");
    let app = Router::new()
        .route("/request-reply/:message", post(route::request_reply))
        .route("/fnf/:command", post(route::fire_and_forget))
        .with_state(Arc::new(
            Nats::new(ServerAddr::from_url(Url::parse(&format!(
                "nats://{}:{}",
                cli.nats_ip, cli.nats_port
            ))?)?)
            .await?,
        ))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace_layer::trace_layer_make_span_with)
                .on_request(trace_layer::trace_layer_on_request)
                .on_response(trace_layer::trace_layer_on_response),
        );

    let bind = SocketAddr::new(cli.rest_ip.parse()?, cli.rest_port);

    info!("Start listening on {}", &bind);

    axum::Server::bind(&bind)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}
