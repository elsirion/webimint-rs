mod client;
mod db;

use crate::client::ClientRpc;
use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::*;

pub fn main() {
    tracing_wasm::set_as_global_default();

    let client = ClientRpc::new();

    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        let (info_signal, info_sender) =
            create_signal(cx, "Waiting to join federation".to_string());

        let (joined_signal, joined_sender) = create_signal(cx, None);

        let invite_code_element: NodeRef<Input> = create_node_ref(cx);
        let on_submit = move |ev: SubmitEvent| {
            // stop the page from reloading!
            ev.prevent_default();

            let invite = invite_code_element.get().expect("<input> to exist").value();
            info_sender.set(format!("Joining {}", invite));
            let client = client.clone();
            spawn_local(async move {
                if let Err(e) = client.join(invite).await {
                    info_sender.set(format!("Join federation failed: {e:?}"));
                };

                let name = client.get_name().await.unwrap();
                info_sender.set(format!("Joined federation {name}"));
                joined_sender.set(Some(name));
            });
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
                {
                    move || match joined_signal.get() {
                        None => view! { cx, <p>"Loading..."</p> }.into_view(cx),
                        Some(name) => {
                            view! { cx, <p>"Joined " {name}</p> }.into_view(cx)
                        }
                    }
                }
            </form>
        }
    });
}
