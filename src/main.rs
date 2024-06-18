#![allow(non_snake_case)]

use dioxus::prelude::*;
use hjkoi_web::components;
use tracing::Level;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    tracing::info!("Launching app");
    launch(components::App);
}
