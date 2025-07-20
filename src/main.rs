mod embed;
mod callback;
mod playwav;

use rdev::listen;
use std::env::consts::OS;

fn main() {
    // Cross-platform OS detection
    if OS == "windows" {
        println!("Running on Windows...");
        run_listener();
    // } else if std::env::var("WAYLAND_DISPLAY").is_ok() {
    //     println!("Wayland detected.");
    //     wayland();
    } else {
        println!("Assuming X11...");
        run_listener();
    }
}

/// Starts listening for global keyboard events and triggers the callback function.
///
/// This function attempts to start listening for keyboard events using the `rdev` library.
/// If an error occurs during the setup of the event listener, it will be printed to the console.
fn run_listener() {
    if let Err(error) = listen(callback::callback) {
        println!("Error: {:?}", error);
    }
}

