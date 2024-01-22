use std::str::FromStr;

use fedimint_core::{BitcoinHash, OutPoint, TransactionId};
use leptos::*;

use crate::context::ClientContext;
use crate::prediction_markets::client::{
    PredictionMarketsRpcRequest, PredictionMarketsRpcResponse,
};
use crate::prediction_markets::components::{ClientPayoutControl, Market};
use crate::utils::empty_view;

#[component]
pub fn PredictionMarketsHome(cx: Scope) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    let general_consensus_resource = create_resource(
        cx,
        || (),
        move |_| async move {
            match client
                .get_value()
                .get_general_consensus()
                .await
            {
                Ok(gc) => {
                    provide_context(cx, gc.to_owned());
                    Some(gc)
                }
                _ => None,
            }
        },
    );

    let market_txid_input = create_rw_signal(cx, String::new());
    let market_outpoint_from_input = move || {
        Some(OutPoint {
            txid: TransactionId::from_str(&market_txid_input.get()).ok()?,
            out_idx: 0,
        })
    };

    let market_outpoint = create_memo(cx, move |_| {
        market_outpoint_from_input().unwrap_or(OutPoint {
            txid: TransactionId::all_zeros(),
            out_idx: 0,
        })
    });

    view! { cx,
        <Show
            when=move || matches!{general_consensus_resource.read(cx), Some(Some(_))}
            fallback=|_| empty_view()
        >
            <ClientPayoutControl />
            <div class="h-2"/>
            <p>enter market txid</p>
            <input on:input=move |ev| market_txid_input.set(event_target_value(&ev)) />

            <Show when=move || market_outpoint_from_input().is_some() fallback=|_| empty_view()>
                <Market market_outpoint=market_outpoint />
            </Show>
        </Show>
    }
}
