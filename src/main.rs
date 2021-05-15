#![forbid(unsafe_code)]

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = warboss_waaghit_lib::OwaaghApp::default();

    //app.army_setups_manager;
    eframe::run_native(Box::new(app));
}
