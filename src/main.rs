// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

slint::include_modules!();
mod crypto_price;
use crypto_price::get_solana_price;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            match get_solana_price() {
                Ok(price) => {
                    let price = format!("{:.3}", price);
                    ui.set_price(slint::SharedString::from(price).clone());
                }
                Err(_) => {
                    println!("Failed to grab price");
                }
            }
        }
    });

    ui.run()?;

    Ok(())
}
