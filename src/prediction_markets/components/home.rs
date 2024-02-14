use fedimint_prediction_markets_common::config::GeneralConsensus;
use leptos::*;
use secp256k1::PublicKey;
use tracing::warn;
use tracing::error;

use crate::context::ClientContext;
use crate::prediction_markets::components::{NewMarket, PayoutControls, ViewMarket};
use crate::utils::empty_view;

#[derive(Debug, Clone)]
pub struct PredictionMarketsStaticDataContext {
    pub client_payout_control: PublicKey,
    pub general_consensus: GeneralConsensus,
}

#[component]
pub fn PredictionMarketsHome() -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>();

    let static_data_resource = create_resource(
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
                PredictionMarketsStaticDataContext {
                    client_payout_control,
                    general_consensus,
                },
            );

            Ok(())
        },
    );

    let sync_and_withdraw_available_bitcoin = create_action(move |()| async move {
        _ = client
            .get_value()
            .sync_orders(true, None, None)
            .await
            .map_err(|e| warn!("Error syncing orders: {e}"));

        _ = client
            .get_value()
            .send_order_bitcoin_balance_to_primary_module()
            .await
            .map_err(|e| warn!("Error withdrawing order balance: {e}"));

        _ = client
            .get_value()
            .send_payout_control_bitcoin_balance_to_primary_module()
            .await
            .map_err(|e| warn!("Error withdrawing payout control balance: {e}"))
    });
    sync_and_withdraw_available_bitcoin.dispatch(());

    let tab = create_rw_signal(Tab::ClientPayoutControl);

    view! { 
        <Show
            when=move || matches!{static_data_resource.get(), Some(Ok(()))}
            fallback=|| empty_view()
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
                    fallback=|| empty_view()
                >
                    <PayoutControls />
                </Show>
                <Show
                    when=move || matches!{tab.get(), Tab::NewMarket}
                    fallback=|| empty_view()
                >
                    <NewMarket />
                </Show>
                <Show
                    when=move || matches!{tab.get(), Tab::ViewMarket}
                    fallback=|| empty_view()
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
