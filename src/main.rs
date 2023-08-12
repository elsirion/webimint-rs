mod client;
mod db;

use fedimint_core::task::sleep;
use futures::stream::StreamExt;
use tracing::trace;

use crate::client::ClientRpc;
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
    pub balance: ReadSignal<Amount>,
    pub send_invoice: WriteSignal<Option<String>>,
}

pub fn provide_app_context(cx: Scope, client: ClientRpc) {
    let client = store_value(cx, client);
    let (send_invoice, set_send_invoice) = create_signal::<Option<String>>(cx, None);

    let (balance, set_balance) = create_signal(cx, Amount::ZERO);

    wasm_bindgen_futures::spawn_local(async move {
        let mut balance_stream = loop {
            // log!("balance_stream");
            let balance_res = client.get_value().subscribe_balance().await;

            match balance_res {
                Ok(balance) => {
                    log!("balance_stream OK");
                    break balance;
                }
                Err(e) => {
                    warn!("client could not get balance: {e:?}");
                    sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        };

        while let Some(amount) = balance_stream.next().await {
            log!("balance stream update: {}", amount.msats);
            set_balance.set(amount);
        }
    });

    // let _ = create_resource_with_initial_value(
    //     cx,
    //     move || send_invoice.get(),
    //     move |value| async move {
    //         log!("ln_send_resource {:?}", value);

    //         match value {
    //             None => {
    //                 log!("no send value");
    //                 return None;
    //             }
    //             Some(value) => {
    //                 log!("calling send");

    //                 if let Err(_) = client.get_value().ln_send(value).await {
    //                     return None;
    //                 };

    //                 return Some(());
    //             }
    //         }
    //     },
    //     None,
    // );

    let context = AppContext {
        client,
        balance,
        send_invoice: set_send_invoice,
    };

    provide_context(cx, context);
}

#[derive(Clone)]
pub(crate) struct TestContext {
    pub name: Option<String>,
}

pub fn provide_test_context(cx: Scope, name: String) {
    trace!("provide_test_context");
    let context = TestContext { name: Some(name) };
    provide_context(cx, context);
}

//
// App component
//
#[component]
fn App(cx: Scope) -> impl IntoView {
    async fn join_request(invite: String) -> ClientRpc {
        let client = ClientRpc::new();
        // TODO: Handle result
        _ = client.join(invite).await;
        client
    }
    let join_action = create_action(cx, |invoice: &String| {
        let invoice = invoice.clone();
        join_request(invoice)
    });

    let invite_code_element: NodeRef<Input> = create_node_ref(cx);

    let on_submit_join = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let invite = invite_code_element.get().expect("<input> to exist").value();
        // Trigger `join_resource` by updating invite code
        join_action.dispatch(invite);
    };

    let joined = move || join_action.value().get().is_some();

    view! { cx,
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


      <Show when=move || join_action.pending().get()
        fallback=|_| view! { cx, "" }
        >
        <p>"Joining ..."</p>
      </Show>

      <Suspense
        fallback=move || view! { cx,
          <p>"Loading..."</p>
        }
      >
      { move || {
        join_action.value().get().map(|c| {
          // Create app context to provide ClientRpc
          // as soon as it's available
          provide_app_context(cx, c);

          let AppContext {client, ..} = expect_context::<AppContext>(cx);

          // get name of the federation
          let name_resource = create_resource(
            cx,
            || (),
            move |_| async move {
                client.get_value().get_name().await
            },
          );

        let federation_label = move || {
          name_resource
              .read(cx)
              .map(|value|
                {
                  match value {
                    Err(error) => format!("Failed to get federation name {error:?}"),
                    Ok(value) => format!("Joined {value:?}")
                  }
                })
              // This loading state will only show before the first load
              .unwrap_or_else(|| "Loading...".into())
      };

          // provide_test_context(cx, "world".to_string());
          // let TestContext {name, ..} = expect_context::<TestContext>(cx);
          view! { cx,
            <p>{federation_label}</p>
            <Balance />
            <ReceiveEcash />
            // <SendLN />
          }
        })
      }}
      </Suspense>

    }
}

//
// Balance component
//

#[component]
fn Balance(cx: Scope) -> impl IntoView {
    let AppContext { balance, .. } = expect_context::<AppContext>(cx);

    let balance_text = move || format! {"{:?} msat", balance.get().msats};

    view! { cx,
      <p>"Balance: " {balance_text}</p>
    }
}

//
// ReceiveEcash component
//
#[component]
fn ReceiveEcash(cx: Scope) -> impl IntoView {
    let AppContext { client, .. } = expect_context::<AppContext>(cx);

    let (receive_value, set_receive_value) = create_signal::<Option<String>>(cx, None);

    let _ = create_resource(
        cx,
        move || receive_value.get(),
        move |value| async move {
            match value {
                None => {
                    log!("no receive value");
                    return None;
                }
                Some(value) => {
                    log!("calling receive");

                    if let Err(_) = client.get_value().receive(value).await {
                        return None;
                    };

                    return Some(());
                }
            }
        },
    );

    let ecash_receive_element: NodeRef<Input> = create_node_ref(cx);

    let on_submit_ecash = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // TODO: Validate value
        let value = ecash_receive_element
            .get()
            .expect("<input> to exist")
            .value();

        set_receive_value.set(Some(value));
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
fn SendLN(cx: Scope) -> impl IntoView {
    let AppContext { send_invoice, .. } = expect_context::<AppContext>(cx);

    let ln_send_element: NodeRef<Input> = create_node_ref(cx);

    let on_submit_ln_send = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let value = ln_send_element.get().expect("<input> to exist").value();
        // TODO: Validate value

        // Trigger `join_resource` by updating invite code
        send_invoice.set(Some(value));
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
