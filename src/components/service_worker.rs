use leptos::{component, create_action, create_effect, IntoView, SignalGet};
use leptos_use::{
    use_service_worker_with_options, ServiceWorkerRegistrationError, UseServiceWorkerOptions,
};
use tracing::{info, warn};

use crate::utils::empty_view;

#[component]
pub fn ServiceWorker(#[prop(into)] path: String) -> impl IntoView {
    let register_action = create_action(move |script_url: &String| {
        let script_url = script_url.to_owned();
        async move {
            info!("Registering service worker: {}", script_url);

            let handle = use_service_worker_with_options(
                UseServiceWorkerOptions::default().script_url(script_url),
            );

            create_effect(move |_| match handle.registration.get() {
                Ok(_) => info!("Service worker registered"),
                Err(ServiceWorkerRegistrationError::Js(e)) => {
                    warn!("Service worker registration failed: {:?}", e)
                }
                Err(ServiceWorkerRegistrationError::NeverQueried) => {}
            });
        }
    });

    register_action.dispatch(path);

    empty_view()
}
