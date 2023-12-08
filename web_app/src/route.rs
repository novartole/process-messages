use std::sync::Arc;

use crate::error::Result;
use crate::state::Nats;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

pub async fn request_reply(
    Path(message): Path<String>,
    State(nats): State<Arc<Nats>>,
) -> Result<impl IntoResponse> {
    Ok(nats.request(message).await?)
}

pub async fn fire_and_forget(
    Path(command): Path<String>,
    State(nats): State<Arc<Nats>>,
) -> Result<impl IntoResponse> {
    Ok(nats.publish(command.try_into()?).await?)
}
