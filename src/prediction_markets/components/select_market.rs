use std::str::FromStr;

use fedimint_core::{OutPoint, TransactionId};
use leptos::*;

use crate::context::ClientContext;

#[component]
pub fn SelectMarket(cx: Scope, market_outpoint: RwSignal<Option<OutPoint>>) -> impl IntoView {
    let form_market_txid = move |txid: String| {
        let Ok(txid) = TransactionId::from_str(txid.as_ref()) else {
            return;
        };
        market_outpoint.set(Some(OutPoint { txid, out_idx: 0 }));
    };

    view! {
        cx,
        <label>"Enter market Txid: "</label>
        <input type="text" on:input=move |ev| form_market_txid(event_target_value(&ev)) />
        <br />
        <span>"Or select market from saved markets"</span>
        
    }
}
