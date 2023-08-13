mod client;
mod db;

use fedimint_core::task::sleep;
use futures::stream::StreamExt;

use crate::client::ClientRpc;
use fedimint_core::Amount;
use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::*;

fn empty_view() -> impl IntoView {
    view! { cx, "" }
}

pub fn main() {
    tracing_wasm::set_as_global_default();
    console_error_panic_hook::set_once();

    mount_to_body(move |cx| {
        view! { cx, <App/> }
    })
}

//
// Client Context
//
#[derive(Clone)]
pub(crate) struct ClientContext {
    pub client: StoredValue<ClientRpc>,
    pub balance: ReadSignal<Amount>,
}

pub fn provide_client_context(cx: Scope, client: ClientRpc) {
    let client = store_value(cx, client);

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

    let context = ClientContext { client, balance };

    provide_context(cx, context);
}

//
// App component
//
#[component]
fn App(cx: Scope) -> impl IntoView {
    let join_action = create_action(cx, |invoice: &String| {
        let invoice = invoice.clone();
        async move {
            let client = ClientRpc::new();
            _ = client.join(invoice).await;
            client
        }
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

      <Show
      when=move || !joined()
        fallback=|_| empty_view()
        >
        <p>"Join a federation"</p>
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

      </Show>


      <Show when=move || join_action.pending().get()
        fallback=|_| empty_view()
        >
        <p>"Joining ..."</p>
      </Show>

      <Suspense
        fallback=move || view!{ cx, "Loading..."}
      >
      <ErrorBoundary fallback=|cx, error| view!{ cx, <p>{format!("Failed to create client: {:?}", error.get())}</p>}>
      { move || {
        join_action.value().get().map(|c| {
          // Create app context to provide ClientRpc
          // as soon as it's available
          provide_client_context(cx, c);

          let ClientContext {client, ..} = expect_context::<ClientContext>(cx);

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

          view! { cx,
            <p>{federation_label}</p>
            <Balance />
            <Receive />
            <Send />
          }
        })
      }}
      </ErrorBoundary>
      </Suspense>

    }
}

//
// Balance component
//

#[component]
fn Balance(cx: Scope) -> impl IntoView {
    let ClientContext { balance, .. } = expect_context::<ClientContext>(cx);

    let balance_text = move || format! {"{:?} msat", balance.get().msats};

    view! { cx,
      <p>"Balance: " {balance_text}</p>
    }
}

//
// ReceiveEcash component
//
#[component]
fn Receive(cx: Scope) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    let client = client.clone();
    let submit_action = create_action(cx, move |invoice: &String| {
        let invoice = invoice.clone();
        async move { client.get_value().receive(invoice).await }
    });

    let input_ref: NodeRef<Input> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // TODO: Validate value
        let value = input_ref.get().expect("<input> to exist").value();

        // set_receive_value.set(Some(value));
        submit_action.dispatch(value);
    };

    view! { cx,

        <form on:submit=on_submit>
            <input
                type="text"
                placeholder="e-cash notes, i.e. BAQB6ijaAs0mXNoyKYvhI…"
                node_ref=input_ref
            />
            <input
                type="submit"
                value="Redeem e-cash"
            />
            <Show
              when=move || !submit_action.pending().get()
              fallback=move |_| view!{ cx, "..."}
              >
              <p>{move || {
                    match submit_action.value().get() {
                    Some(result) =>
                      match result {
                        Err(error) => format!("✗ Failed to redeem e-cash: {:?}", error),
                        Ok(value) => format!("✓ Redeemed {:?} msat", value.msats)
                      }
                    None => "".to_string()
                  }
                }
              }</p>
            </Show>
        </form>

    }
}

//
// SendLN component
//
#[component]
fn Send(cx: Scope) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    let client = client.clone();
    let submit_action = create_action(cx, move |invoice: &String| {
        let invoice = invoice.clone();
        async move { client.get_value().ln_send(invoice).await }
    });

    let input_ref: NodeRef<Input> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let value = input_ref.get().expect("<input> to exist").value();
        // TODO: Validate value
        submit_action.dispatch(value);
    };

    view! { cx,
      <form on:submit=on_submit>
            <input
                type="text"
                placeholder="LN invoice, i.e. lnbcrt1p0…"
                node_ref=input_ref
            />
            <input
                type="submit"
                value="Pay LN invoice"
            />
            <Show
            when=move || !submit_action.pending().get()
            fallback=move |_| view!{ cx, "..."}
            >
            <p>{move || {
                  match submit_action.value().get() {
                  Some(result) =>
                    match result {
                      Err(error) => format!("✗ Failed to send invoice {:?}", error),
                      Ok(_) => "✓ Invoice successfully sent".to_string()
                    }
                  None => "".to_string()
                }
              }
            }</p>
          </Show>
        </form>

    }
}
