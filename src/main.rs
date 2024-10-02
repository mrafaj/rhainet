#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use rhainet::RhaiBrowserApp;
use tokio::runtime::Runtime;


#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    use std::time::Duration;


    let runtime = Runtime::new().expect("Unable to create tokio Runtime.");
    let _enter = runtime.enter();
    std::thread::spawn(move || {
        runtime.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        })
    });
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Rhai Browser", native_options, Box::new(|cc| Ok(Box::new(RhaiBrowserApp::new(cc)))))
}
