mod db;

use fedimint_client::secret::PlainRootSecretStrategy;
use fedimint_core::api::InviteCode;
use fedimint_core::db::mem_impl::MemDatabase;
use fedimint_core::task::{sleep, TaskGroup};
use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::*;
use std::str::FromStr;
use std::time::Duration;

pub fn main() {
    tracing_wasm::set_as_global_default();

    let (invite_sender, mut invite_receiver) = tokio::sync::mpsc::channel::<String>(10);
    let (info_sender, info_receiver) = tokio::sync::mpsc::channel::<String>(100);

    wasm_bindgen_futures::spawn_local(async move {
        sleep(Duration::from_millis(1000)).await;

        info_sender
            .send(format!("Awaiting join code"))
            .await
            .unwrap();

        let invite_code = loop {
            let invite_code_str = invite_receiver
                .recv()
                .await
                .expect("Sender dropped unexpectedly");
            match InviteCode::from_str(&invite_code_str) {
                Ok(invite) => break invite,
                Err(e) => {
                    info_sender
                        .send(format!("Invalid invite code: {e:?}"))
                        .await
                        .unwrap();
                }
            }
        };

        info_sender.send(format!("Starting client")).await.unwrap();
        let mut client_builder = fedimint_client::Client::builder();
        client_builder.with_database(MemDatabase::new());
        client_builder.with_primary_module(1);
        client_builder.with_invite_code(invite_code);
        let tg = TaskGroup::new();
        let client_res = client_builder.build::<PlainRootSecretStrategy>(tg).await;

        let client = match client_res {
            Ok(client) => client,
            Err(e) => {
                info_sender
                    .send(format!("Error initializing client: {e:?}"))
                    .await
                    .unwrap();
                return;
            }
        };

        let name = client
            .get_meta("federation_name")
            .unwrap_or("<unknown>".to_string());
        info_sender
            .send(format!("Client started! Connected to federation: {name}"))
            .await
            .unwrap();
    });

    let info_stream = tokio_stream::wrappers::ReceiverStream::new(info_receiver);

    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        let info_signal = create_signal_from_stream(cx, info_stream);

        let invite_code_element: NodeRef<Input> = create_node_ref(cx);
        let on_submit = move |ev: SubmitEvent| {
            // stop the page from reloading!
            ev.prevent_default();

            // here, we'll extract the value from the input
            let value = invite_code_element
                .get()
                // event handlers can only fire after the view
                // is mounted to the DOM, so the `NodeRef` will be `Some`
                .expect("<input> to exist")
                // `NodeRef` implements `Deref` for the DOM element type
                // this means we can call`HtmlInputElement::value()`
                // to get the current value of the input
                .value();
            invite_sender.blocking_send(value).expect("Can send");
        };

        view! { cx,
            <p>"Status: " {info_signal}</p>
            <form on:submit=on_submit>
                <input
                    type="text"
                    placeholder="Invite Code, i.e. fed11jpr3lgm8t…"
                    node_ref=invite_code_element
                />
                <input
                    type="submit"
                    value="Join Federation"
                />
            </form>
        }
    });
}
