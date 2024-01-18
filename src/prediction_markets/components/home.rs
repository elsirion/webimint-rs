use std::str::FromStr;

use fedimint_core::{BitcoinHash, OutPoint, TransactionId};
use leptos::html::Output;
use leptos::*;

use crate::context::ClientContext;
use crate::prediction_markets::components::{Market, PayoutControlPublicKey};
use crate::utils::empty_view;

#[component]
pub fn PredictionMarketsHome(cx: Scope) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

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
        <PayoutControlPublicKey />
        <div class="h-2"/>
        <input on:input=move |ev| market_txid_input.set(event_target_value(&ev)) />

        <Show when=move || market_outpoint_from_input().is_some() fallback=|_| empty_view()>
            <Market market_outpoint=market_outpoint />
        </Show>
    }
}
