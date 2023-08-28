use leptos::*;
use rexie::ObjectStore;
use std::time::Duration;
use tracing::info;
use wasm_bindgen::JsValue;

pub fn main() {
    tracing_wasm::set_as_global_default();
    console_error_panic_hook::set_once();

    spawn_local(async {
        let db = rexie::Rexie::builder("test")
            .version(1)
            .add_object_store(ObjectStore::new("store").auto_increment(false))
            .build()
            .await
            .expect("Could not open IndexedDB");

        loop {
            let tx = db
                .transaction(&["store"], rexie::TransactionMode::ReadWrite)
                .expect("Could not start IndexedDB transaction");

            tx.store("store")
                .expect("Could not get IndexedDB store")
                .put(&JsValue::from_str("bar"), Some(&JsValue::from_str("foo")))
                .await
                .expect("Could not put value into IndexedDB");

            gloo_timers::future::sleep(Duration::from_secs(1)).await;

            let value = tx
                .store("store")
                .expect("Could not get IndexedDB store")
                .get(&JsValue::from_str("foo"))
                .await
                .expect("Could not put value into IndexedDB");

            info!("value read: {value:?}");

            tx.commit()
                .await
                .expect("Could not commit IndexedDB transaction");
        }
    });

    mount_to_body(move |cx| {
        view! { cx, "running" }
    })
}
