#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = warboss_waaghit_lib::OwaaghApp::default();

    //app.army_setups_manager;
    eframe::run_native(Box::new(app));
}
