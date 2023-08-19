use fedimint_core::Amount;
use leptos::*;

use crate::context::ClientContext;

//
// Balance component
//
#[component]
pub fn Balance(cx: Scope, #[prop(optional, into)] class: String) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);
    let balance_resource = create_local_resource(
        cx,
        || (),
        move |()| async move {
            let balance_stream = match client.get_value().subscribe_balance().await {
                Ok(balance) => balance,
                Err(e) => {
                    // TODO: maybe better error handling?
                    warn!("client could subscribe to balance: {e:?}");
                    std::future::pending().await
                }
            };
            create_signal_from_stream(cx, balance_stream)
        },
    );
    let balance = move || match balance_resource.read(cx) {
        None => view! { cx, <p>"Loading..."</p> }.into_view(cx),
        Some(balance) => {
            let balance_msat = move || balance.get().unwrap_or(Amount::ZERO).msats;
            view! { cx, {balance_msat} " msat" }.into_view(cx)
        }
    };

    view! { cx,
      <div class=class>
        <h2 class="text-xl leading-tight w-full font-body font-semibold  pb-4 mb-4 text-gray-400 border-b-2 border-gray-200">"Balance"</h2>
        <h3 class="text-4xl">{balance}</h3>
      </div>
    }
}
