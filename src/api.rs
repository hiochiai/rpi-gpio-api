mod device;
mod error;
mod gpio;
mod version;

pub use device::*;
pub use error::*;

use axum::{routing::get, Router};

pub fn new_router() -> Router {
    Router::new()
        .route("/api/device", get(device::get))
        .route("/api/gpio/:number", get(gpio::get).put(gpio::set))
        .route("/api/version", get(version::get))
}
