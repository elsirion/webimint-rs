use std::str::FromStr;

use fedimint_core::{BitcoinHash, OutPoint, TransactionId};
use leptos::*;
use tracing::info;

use crate::context::ClientContext;
use crate::prediction_markets::components::Market;
use crate::utils::empty_view;

#[component]
pub fn ViewMarket(cx: Scope) -> impl IntoView {
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

    view! {
        cx,
        <p>"enter market txid"</p>
        <input type="text"
            on:input=move |ev| market_txid_input.set(event_target_value(&ev))
        />
        <Show
            when=move || matches!{market_outpoint_from_input(), Some(_)}
            fallback=|_| empty_view()
        >
            <Market market_outpoint=market_outpoint />
        </Show>
    }
}
