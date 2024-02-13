use leptos::{component, create_action, window, IntoView};
use tracing::{info, warn};

use crate::utils::empty_view;

#[component]
pub fn ServiceWorker(#[prop(into)] path: String) -> impl IntoView {
    let register_action = create_action(move |script_url: &String| {
        let script_url = script_url.to_owned();
        async move {
            info!("Registering service worker: {}", script_url);
            let promise = window()
                .navigator()
                .service_worker()
                .register(script_url.as_str());
            if let Err(e) = wasm_bindgen_futures::JsFuture::from(promise).await {
                warn!("Service worker registration failed: {:?}", e);
            }
            info!("Service worker registered");
        }
    });

    register_action.dispatch(path);

    empty_view()
}
