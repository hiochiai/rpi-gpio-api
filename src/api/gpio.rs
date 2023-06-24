use axum::{
    debug_handler,
    extract::Path,
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};

use crate::api;

#[derive(Serialize, Deserialize, Debug)]
pub struct Gpio {
    function: Function,
    level: Level,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Function {
    Input,
    Output,
    Alt0,
    Alt1,
    Alt2,
    Alt3,
    Alt4,
    Alt5,
}

impl From<Function> for rppal::gpio::Mode {
    fn from(v: Function) -> Self {
        match v {
            Function::Input => rppal::gpio::Mode::Input,
            Function::Output => rppal::gpio::Mode::Output,
            Function::Alt0 => rppal::gpio::Mode::Alt0,
            Function::Alt1 => rppal::gpio::Mode::Alt1,
            Function::Alt2 => rppal::gpio::Mode::Alt2,
            Function::Alt3 => rppal::gpio::Mode::Alt3,
            Function::Alt4 => rppal::gpio::Mode::Alt4,
            Function::Alt5 => rppal::gpio::Mode::Alt5,
        }
    }
}

impl From<rppal::gpio::Mode> for Function {
    fn from(v: rppal::gpio::Mode) -> Self {
        match v {
            rppal::gpio::Mode::Input => Function::Input,
            rppal::gpio::Mode::Output => Function::Output,
            rppal::gpio::Mode::Alt0 => Function::Alt0,
            rppal::gpio::Mode::Alt1 => Function::Alt1,
            rppal::gpio::Mode::Alt2 => Function::Alt2,
            rppal::gpio::Mode::Alt3 => Function::Alt3,
            rppal::gpio::Mode::Alt4 => Function::Alt4,
            rppal::gpio::Mode::Alt5 => Function::Alt5,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Level {
    Low,
    High,
}

impl From<bool> for Level {
    fn from(high: bool) -> Self {
        if high {
            Level::High
        } else {
            Level::Low
        }
    }
}

pub async fn get(Path(gpio_number): Path<String>) -> Response {
    let gpio = match get_gpio(&gpio_number) {
        Ok(gpio) => gpio,
        Err(resp) => return resp,
    };

    let mode = gpio.mode();

    Json(Gpio {
        function: mode.into(),
        level: gpio.into_io(mode).is_high().into(),
    })
    .into_response()
}

#[debug_handler]
pub async fn set(Path(gpio_number): Path<String>, Json(req): Json<Gpio>) -> Response {
    let gpio = match get_gpio(&gpio_number) {
        Ok(gpio) => gpio,
        Err(resp) => return resp,
    };

    let mut gpio = gpio.into_io(req.function.into());
    gpio.set_reset_on_drop(false);

    match req.level {
        Level::High => gpio.set_high(),
        Level::Low => gpio.set_low(),
    }

    Json(req).into_response()
}

fn get_gpio(gpio_number: &str) -> Result<rppal::gpio::Pin, Response> {
    let gpio = match rppal::gpio::Gpio::new() {
        Ok(gpio) => gpio,
        Err(e) => {
            return Err(api::Error::new_internal_server_error(e.to_string()).into_response());
        }
    };

    let num = match gpio_number.parse::<u8>() {
        Ok(num) => num,
        Err(e) => {
            return Err(
                api::Error::new_not_found(gpio_number.to_string(), e.to_string()).into_response(),
            );
        }
    };

    let gpio = match gpio.get(num) {
        Ok(gpio) => gpio,
        Err(e) => {
            return Err(
                api::Error::new_not_found(format!("{}", num), e.to_string()).into_response()
            );
        }
    };

    Ok(gpio)
}
