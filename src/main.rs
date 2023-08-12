mod client;
mod db;

use futures::stream::StreamExt;
use tracing::warn;

use crate::client::ClientRpc;
use fedimint_core::task::sleep;
use fedimint_core::Amount;
use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::*;

pub fn main() {
    tracing_wasm::set_as_global_default();
    console_error_panic_hook::set_once();

    mount_to_body(move |cx| {
        view! { cx, <App/> }
    })
}

//
// App Context
//
#[derive(Clone)]
pub(crate) struct AppContext {
    pub client: StoredValue<ClientRpc>,
    pub client_r: ReadSignal<Option<ClientRpc>>,
    pub client_w: WriteSignal<Option<ClientRpc>>,
    pub balance: ReadSignal<Amount>,
    pub name: ReadSignal<Option<String>>,
}

// TODO: tracing lib

pub fn provide_app_context(cx: Scope) {
    let client = store_value(cx, ClientRpc::new());

    let (client_r, client_w) = create_signal::<Option<ClientRpc>>(cx, None);

    let (name, set_name) = create_signal::<Option<String>>(cx, None);

    create_effect(cx, move |_| async move {
        if let Some(client) = client_r.get() {
            let name = client.get_name().await.unwrap();
            set_name.set(Some(name.clone()));
        };
    });

    let (balance, set_balance) = create_signal(cx, Amount::ZERO);
    create_effect(cx, move |_| async move {
        if let Some(client) = client_r.get() {
            log!("create effect client");
            wasm_bindgen_futures::spawn_local(async move {
                let mut update_b_stream = loop {
                    let balance_res = client.clone().subscribe_balance().await;

                    match balance_res {
                        Ok(balance) => {
                            break balance;
                        }
                        Err(e) => {
                            warn!("client could not get balance: {e:?}");
                            sleep(std::time::Duration::from_secs(1)).await;
                        }
                    }
                };

                while let Some(amount) = update_b_stream.next().await {
                    log!("balance update: {}", amount.msats);
                    set_balance.set(amount);
                }
            });
        };
    });

    let context = AppContext {
        client,
        balance,
        client_r,
        client_w,
        name,
    };

    provide_context(cx, context);
}

//
// App component
//
#[component]
fn App(cx: Scope) -> impl IntoView {
    provide_app_context(cx);

    let AppContext {
        client_w: c_w,
        name,
        ..
    } = expect_context::<AppContext>(cx);

    let (info, set_info) = create_signal(cx, "".to_string());

    //
    // join
    //
    let (invite_code, set_invite_code) = create_signal::<Option<String>>(cx, None);

    // let client_submit = client.clone();

    // TODO: Proper error handling, return an `anyhow::Result` instead of Option<String>
    let join_resource: Resource<Option<String>, Option<()>> = create_resource_with_initial_value(
        cx,
        move || invite_code.get(),
        move |value| async move {
            log!("join_resource {value:?}");

            match value {
                None => {
                    log!("no invite code");
                    return None;
                }
                Some(value) => {
                    log!("calling join");
                    let c = ClientRpc::new();
                    // let c = c.get_value();
                    // TODO: Error handling
                    _ = c.join(value).await;
                    c_w.set(Some(c.clone()));

                    return Some(());
                }
            }
        },
        None,
    );

    let invite_code_element: NodeRef<Input> = create_node_ref(cx);

    let on_submit_join = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let invite = invite_code_element.get().expect("<input> to exist").value();
        // Trigger `join_resource` by updating invite code
        set_invite_code.set(Some(invite));
    };

    let joined = move || join_resource.read(cx).flatten().is_some();

    // Update info depending on joined state
    create_effect(cx, move |_| {
        if join_resource.loading().get() {
            set_info.set("Joining Federation... (pls wait)".to_string());
        } else {
            if joined() {
                set_info.set(format!(
                    "Joined {}",
                    name.get().unwrap_or("unknown".to_string())
                ));
            } else {
                set_info.set("Waiting to join federation...".to_string());
            }
        }
    });

    view! { cx,
      <p>"Status: " {info}</p>
      <form on:submit=on_submit_join>
          // TODO: Validate invite code. Listen to `on:change`
          <input
              type="text"
              node_ref=invite_code_element
              placeholder="Invite Code, i.e. fed11jpr3lgm8t…"
              prop:disabled=joined()
          />
          <input
              type="submit"
              value="Join Federation"
              prop:disabled=joined()
          />
      </form>

      <Show
        when=joined
        fallback=|_| view! { cx,
          <></>
        }
      >
        <Balance />
        <ReceiveEcash set_info />
        <SendLN set_info />
      </Show>

    }
}

//
// Balance component
//
#[component]
fn Balance(cx: Scope) -> impl IntoView {
    let AppContext { balance, .. } = expect_context::<AppContext>(cx);

    view! { cx,
    <p>"Balance: " {move || balance.get().msats} " msat"</p> }
    .into_view(cx)
}

//
// ReceiveEcash component
//
#[component]
fn ReceiveEcash(cx: Scope, set_info: WriteSignal<String>) -> impl IntoView {
    let AppContext { client_r, .. } = expect_context::<AppContext>(cx);

    let (ecash_receive, set_ecash_receive) = create_signal::<Option<String>>(cx, None);
    let ecash_receive_element: NodeRef<Input> = create_node_ref(cx);

    let client_receive = client_r.clone();

    let _ecash_receive_resource = create_resource_with_initial_value(
        cx,
        move || ecash_receive.get(),
        move |value| async move {
            log!("ecash_resource {:?}", value);

            match value {
                None => {
                    log!("no receive value");
                    return None;
                }
                Some(value) => {
                    log!("calling receive");

                    let c = client_receive.get().expect("client to exist");

                    if let Err(e) = c.receive(value).await {
                        set_info.set(format!("Receive ecash failed: {e:?}"));
                        return None;
                    };

                    return Some(());
                }
            }
        },
        None,
    );

    let on_submit_ecash = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // TODO: Validate value
        let value = ecash_receive_element
            .get()
            .expect("<input> to exist")
            .value();

        // Trigger `join_resource` by updating invite code
        set_ecash_receive.set(Some(value));
    };

    view! { cx,

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
}

//
// SendLN component
//
#[component]
fn SendLN(cx: Scope, set_info: WriteSignal<String>) -> impl IntoView {
    provide_app_context(cx);

    let AppContext { client, .. } = expect_context::<AppContext>(cx);

    let (ln_send, set_ln_send) = create_signal::<Option<String>>(cx, None);
    let ln_send_element: NodeRef<Input> = create_node_ref(cx);
    let client_ln_send = client.clone();

    let _ln_send_resource = create_resource_with_initial_value(
        cx,
        move || ln_send.get(),
        move |value| async move {
            log!("ln_send_resource {:?}", value);

            match value {
                None => {
                    log!("no send value");
                    return None;
                }
                Some(value) => {
                    log!("calling send");

                    let c = client_ln_send.get_value();

                    if let Err(e) = c.ln_send(value).await {
                        set_info.set(format!("LN send failed: {e:?}"));
                        return None;
                    };

                    return Some(());
                }
            }
        },
        None,
    );

    let on_submit_ln_send = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let value = ln_send_element.get().expect("<input> to exist").value();
        // TODO: Validate value

        // Trigger `join_resource` by updating invite code
        set_ln_send.set(Some(value));
    };

    view! { cx,
      <form on:submit=on_submit_ln_send>
            <input
                type="text"
                placeholder="LN invoice, i.e. lnbcrt1p0…"
                node_ref=ln_send_element
            />
            <input
                type="submit"
                value="Pay LN invoice"
            />
        </form>

    }
}
