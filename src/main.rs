mod client;
mod db;

use crate::client::ClientRpc;
use fedimint_core::Amount;
use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::*;

pub fn main() {
    tracing_wasm::set_as_global_default();

    let client = ClientRpc::new();

    console_error_panic_hook::set_once();
    mount_to_body(move |cx| {
        let (info_signal, info_sender) =
            create_signal(cx, "Waiting to join federation".to_string());

        let (joined_signal, joined_sender) = create_signal(cx, None);
        let (balance_signal, balance_sender) = create_signal(cx, None);

        let client_submit = client.clone();
        let invite_code_element: NodeRef<Input> = create_node_ref(cx);
        let on_submit = move |ev: SubmitEvent| {
            // stop the page from reloading!
            ev.prevent_default();

            let invite = invite_code_element.get().expect("<input> to exist").value();
            info_sender.set(format!("Joining {}", invite));
            let client = client_submit.clone();
            spawn_local(async move {
                if let Err(e) = client.join(invite).await {
                    info_sender.set(format!("Join federation failed: {e:?}"));
                };

                let name = client.get_name().await.unwrap();
                info_sender.set(format!("Joined federation {name}"));
                joined_sender.set(Some(name));

                let balance_subscription = client.subscribe_balance().await.unwrap();
                let balance_stream_signal = create_signal_from_stream(cx, balance_subscription);
                balance_sender.set(Some(balance_stream_signal));
            });
        };

        let client_ecash = client.clone();
        let ecash_receive_element: NodeRef<Input> = create_node_ref(cx);
        let on_submit_ecash = move |ev: SubmitEvent| {
            // stop the page from reloading!
            ev.prevent_default();

            let ecash = ecash_receive_element
                .get()
                .expect("<input> to exist")
                .value();
            info_sender.set(format!("Reissuing {}", ecash));
            let client = client_ecash.clone();
            spawn_local(async move {
                if let Err(e) = client.receive(ecash).await {
                    info_sender.set(format!("Receive ecash failed: {e:?}"));
                };
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
            </form>
            {
                move || match joined_signal.get() {
                    None => view! { cx, <p>"Loading..."</p> }.into_view(cx),
                    Some(name) => {
                        view! { cx, <p>"Joined " {name}</p> }.into_view(cx)
                    }
                }
            }
            {
                move || match balance_signal.get() {
                    None => view! { cx, <p>"Balance: 0 msat"</p> }.into_view(cx),
                    Some(balance) => {
                        let current_balance = balance.get().unwrap_or(Amount::ZERO).msats;
                        view! { cx, <p>"Balance " {current_balance} " msat"</p> }.into_view(cx)
                    }
                }
            }
            <form on:submit=on_submit_ecash>
                <input
                    type="text"
                    placeholder="e-cash notes, i.e. BAQB6ijaAs0mXNoyKYvhI…"
                    node_ref=ecash_receive_element
                />
                <input
                    type="submit"
                    value="Redeem e-cash"
                />
            </form>
        }
    });
}
