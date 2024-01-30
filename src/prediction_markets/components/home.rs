
use leptos::*;

use crate::context::ClientContext;
use crate::prediction_markets::components::{PayoutControls, NewMarket, ViewMarket};
use crate::utils::empty_view;

#[component]
pub fn PredictionMarketsHome(cx: Scope) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    let general_consensus_resource = create_resource(
        cx,
        || (),
        move |_| async move {
            match client.get_value().get_general_consensus().await {
                Ok(gc) => {
                    provide_context(cx, gc.to_owned());
                    Some(gc)
                }
                Err(e) => {
                    error!("failure to get general consensus: {e}");
                    None
                }
            }
        },
    );

    let tab = create_rw_signal(cx, Tab::ClientPayoutControl);

    view! { cx,
        <Show
            when=move || matches!{general_consensus_resource.read(cx), Some(Some(_))}
            fallback=|_| empty_view()
        >
            <div class="flex">
                <button
                    on:click=move |_| tab.set(Tab::ClientPayoutControl)
                    class={move || format!("my-1 border-b-2 p-3
                    font-body font-semibold 
                    leading-tight hover:text-blue-500 {active}", 
                    active = if tab.get() == Tab::ClientPayoutControl  {"text-blue-400 border-blue-400"} else {"text-gray-400 border-gray-200 hover:border-gray-700"} )}
                >
                    "Payout Controls"
                </button>
                <button
                    on:click=move |_| tab.set(Tab::NewMarket)
                    class={move || format!("my-1 border-b-2 p-3
                    font-body font-semibold  
                    leading-tight hover:text-blue-500 {active}", 
                    active = if tab.get() == Tab::NewMarket  {"text-blue-400 border-blue-400"} else {"text-gray-400 border-gray-200 hover:border-gray-700"} )}
                >
                    "New Market"
                </button>
                <button
                    on:click=move |_| tab.set(Tab::ViewMarket)
                    class={move || format!("my-1 border-b-2 p-3
                    font-body font-semibold  
                    leading-tight hover:text-blue-500 {active}", 
                    active = if tab.get() == Tab::ViewMarket  {"text-blue-400 border-blue-400"} else {"text-gray-400 border-gray-200 hover:border-gray-700"} )}
                >
                    "View Market"
                </button>
            </div>

            <div>
                <Show
                    when=move || matches!{tab.get(), Tab::ClientPayoutControl}
                    fallback=|_| empty_view()
                >
                    <PayoutControls />
                </Show>
                <Show
                    when=move || matches!{tab.get(), Tab::NewMarket}
                    fallback=|_| empty_view()
                >
                    <NewMarket />
                </Show>
                <Show
                    when=move || matches!{tab.get(), Tab::ViewMarket}
                    fallback=|_| empty_view()
                >
                    <ViewMarket />
                </Show>
            </div>
        </Show>
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Tab {
    ClientPayoutControl,
    NewMarket,
    ViewMarket,
}
