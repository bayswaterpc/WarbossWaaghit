#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

mod resources_panel;
mod advanced_search_container;
mod app;
pub mod army_build;
pub mod army_setups_folder;
pub mod army_setups_manager;
mod ca_game;
mod central_panel_state;
pub mod factions;
mod misc_folders;
pub mod ymd_hms_dash_format;

pub use app::OwaaghApp;

// ----------------------------------------------------------------------------
// When compiling for web:

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    let app = OwaaghApp::default();
    eframe::start_web(canvas_id, Box::new(app))
}
