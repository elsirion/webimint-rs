use fedimint_prediction_markets_common::UnixTimestamp;

pub fn unix_timestamp_to_js_string(t: UnixTimestamp) -> String {
    let unix_milli =
        js_sys::wasm_bindgen::JsValue::from_f64((t.0 * 1000) as f64);

    js_sys::Date::new(&unix_milli)
        .to_locale_string("en-US", &js_sys::wasm_bindgen::JsValue::UNDEFINED)
        .as_string()
        .unwrap_or("Failed to get time string".to_owned())
}
