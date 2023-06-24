use axum::response::{IntoResponse, Json, Response};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Version {
    version: String,
    api_version: String,
}

pub async fn get() -> Response {
    Json(Version {
        version: env!("CARGO_PKG_VERSION").to_string(),
        api_version: env!("_GPIO_API_VERSION").to_string(),
    })
    .into_response()
}
