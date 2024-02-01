use leptos::*;

use super::PredictionMarketsStaticDataContext;
use crate::context::ClientContext;
use crate::prediction_markets::components::{NewMarket, PayoutControls, ViewMarket};
use crate::utils::empty_view;

#[component]
pub fn PredictionMarketsHome(cx: Scope) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    let static_data_resource = create_resource(
        cx,
        || (),
        move |_| async move {
            let Ok(client_payout_control) = client.get_value().get_client_payout_control().await
            else {
                error!("failed to get client payout control");
                return Err(());
            };
            let Ok(general_consensus) = client.get_value().get_general_consensus().await else {
                error!("failed to get general consensus");
                return Err(());
            };

            provide_context(
                cx,
                PredictionMarketsStaticDataContext {
                    client_payout_control,
                    general_consensus,
                },
            );

            Ok(())
        },
    );

    let tab = create_rw_signal(cx, Tab::ClientPayoutControl);

    view! { cx,
        <Show
            when=move || matches!{static_data_resource.read(cx), Some(Ok(()))}
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
