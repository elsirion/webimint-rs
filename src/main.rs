mod client;
mod db;

mod components;
mod context;
mod utils;

mod prediction_markets;

use components::App;
use leptos::*;

pub fn main() {
    tracing_wasm::set_as_global_default();
    console_error_panic_hook::set_once();

    mount_to_body(move || {
        view! { <App/> }
    })
}
