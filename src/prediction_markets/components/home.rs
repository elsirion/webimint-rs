use std::str::FromStr;

use fedimint_core::{TransactionId, BitcoinHash, OutPoint};
use leptos::*;

use crate::context::ClientContext;
use crate::prediction_markets::components::{Market, PayoutControlPublicKey};

#[component]
pub fn PredictionMarketsHome(cx: Scope) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    let market_txid_input = create_rw_signal(cx, String::new());
    let market_outpoint = move || OutPoint {
        txid: TransactionId::from_str(&market_txid_input.get())
            .unwrap_or(TransactionId::all_zeros()),
        out_idx: 0,
    };

    view! { cx,
        <PayoutControlPublicKey />
        <div class="h-2"/>
        <input on:input=move |ev| market_txid_input.set(event_target_value(&ev)) />
        <Market outpoint=market_outpoint />
    }
}
