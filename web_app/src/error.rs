use std::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use command::CommandError;
use tracing::error;

pub type Result<T, E = Report> = color_eyre::Result<T, E>;

pub struct Report(color_eyre::Report);

impl fmt::Debug for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<E> From<E> for Report
where
    E: Into<color_eyre::Report>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for Report {
    fn into_response(self) -> Response {
        let report = self.0;

        error!("{}", format!("{:?}", report));

        if let Some(CommandError::ParseFromString(value)) =
            report.downcast_ref::<CommandError>().map(|err| err)
        {
            (
                StatusCode::BAD_REQUEST,
                format!("'{}' is not a valid command", value),
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong"),
            )
        }
        .into_response()
    }
}
