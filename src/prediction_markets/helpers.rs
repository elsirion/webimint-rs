use fedimint_prediction_markets_common::{Seconds, UnixTimestamp};
use tracing::info;

pub fn unix_timestamp_to_js_string(t: UnixTimestamp) -> String {
    let unix_milli =
        js_sys::wasm_bindgen::JsValue::from_f64((t.0 * 1000) as f64);

    js_sys::Date::new(&unix_milli)
        .to_locale_string("en-US", &js_sys::wasm_bindgen::JsValue::UNDEFINED)
        .as_string()
        .unwrap_or("Failed to get time string".to_owned())
}

pub fn js_string_to_unix_timestamp(s: String) -> UnixTimestamp {
    let unix_milli = js_sys::Date::parse(s.as_ref());

    UnixTimestamp((unix_milli / 1000f64) as Seconds)
}