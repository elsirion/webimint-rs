use fedimint_core::Amount;
use leptos::logging::*;
use leptos::*;

use crate::context::ClientContext;

//
// Balance component
//
#[component]
pub fn Balance(#[prop(optional, into)] class: String) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>();
    let balance_resource = create_local_resource(
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
            create_signal_from_stream(balance_stream)
        },
    );
    let balance = move || match balance_resource.get() {
        None => view! { <p>"Loading..."</p> }.into_view(),
        Some(balance) => {
            let balance_msat = move || balance.get().unwrap_or(Amount::ZERO).msats;
            view! { {balance_msat} " msat" }.into_view()
        }
    };

    view! {
      <div class=class>
        <h2 class="text-xl leading-tight w-full font-body font-semibold  pb-4 mb-4 text-gray-400 border-b-2 border-gray-200">"Balance"</h2>
        <h3 class="text-4xl">{balance}</h3>
      </div>
    }
}
