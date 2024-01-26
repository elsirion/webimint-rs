use std::str::FromStr;

use fedimint_core::{BitcoinHash, OutPoint, TransactionId};
use leptos::*;
use tracing::info;

use crate::context::ClientContext;
use crate::prediction_markets::components::{Market, SelectMarket};
use crate::utils::empty_view;

#[component]
pub fn ViewMarket(cx: Scope) -> impl IntoView {
    let market_outpoint = create_rw_signal(cx, None);

    view! {
        cx,
        <Show
            when=move || matches!{market_outpoint.get(), None}
            fallback=|_| empty_view()
        >
            <SelectMarket market_outpoint=market_outpoint />
        </Show>
        <Show
            when=move || matches!{market_outpoint.get(), Some(_)}
            fallback=|_| empty_view()
        >
            <Market market_outpoint=create_memo(cx, move |_| market_outpoint.get().unwrap()) />
        </Show>
    }
}
