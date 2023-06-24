use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let re = Regex::new(r"^  version: (.*)$").unwrap();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("api")
        .join("rpi-gpio-api")
        .join("openapi.yaml");
    let file = File::open(filename).unwrap();

    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        if let Some(caps) = re.captures(&line) {
            println!("cargo:rustc-env=_GPIO_API_VERSION={}", &caps[0]);
            return;
        }
    }
}
