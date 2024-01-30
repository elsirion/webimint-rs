use std::collections::HashMap;
use std::str::FromStr;

use fedimint_core::{BitcoinHash, OutPoint, TransactionId};
use leptos::*;
use tracing::{info, warn};

use crate::context::ClientContext;
use crate::prediction_markets::components::{Market, SelectMarket};
use crate::utils::empty_view;

#[component]
pub fn ViewMarket(cx: Scope) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    let market_outpoint: RwSignal<Option<OutPoint>> = create_rw_signal(cx, None);

    let get_saved_markets_resource = create_resource(
        cx,
        || (()),
        move |()| async move { client.get_value().get_saved_markets().await },
    );
    let saved_markets_set_memo = create_memo(cx, move |_| {
        let Some(Ok(saved_markets)) = get_saved_markets_resource.read(cx) else {
            return None;
        };
        let set = saved_markets
            .into_iter()
            .map(|(k, _)| (k, ()))
            .collect::<HashMap<OutPoint, ()>>();

        Some(set)
    });
    let market_saved = create_memo(cx, move |_| {
        saved_markets_set_memo.with(|r| {
            let Some(b) = r else { return false };
            let Some(m) = market_outpoint.get() else {
                return false;
            };

            b.contains_key(&m)
        })
    });
    let save_market_button_text = move || match market_saved.get() {
        true => "Unsave Market",
        false => "Save Market",
    };

    let save_market_button_action = create_action(cx, move |()| {
        let client = client.get_value();
        let market_outpoint = market_outpoint.get_untracked().unwrap();

        async move {
            match market_saved.get_untracked() {
                true => client.unsave_market(market_outpoint).await,
                false => client.save_market(market_outpoint).await,
            }
            .map_err(|e| warn!("Error saving market: {e:?}"))
            .unwrap();

            get_saved_markets_resource.refetch();
        }
    });

    view! {
        cx,
        <Show
            when=move || matches!{get_saved_markets_resource.read(cx), Some(_)}
            fallback=|_| empty_view()
        >
        
            <Show
                when=move || matches!{market_outpoint.get(), None}
                fallback=|_| empty_view()
            >
                <SelectMarket
                    market_outpoint=market_outpoint
                    saved_markets=create_memo(cx, move |_| {
                        get_saved_markets_resource
                            .read(cx)
                            .map(|r| r.map_err(|e| warn!("Error getting saved markets: {e:?}"))
                            .unwrap()
                        )
                    })
                />
            </Show>
            <Show
                when=move || matches!{market_outpoint.get(), Some(_)}
                fallback=|_| empty_view()
            >
                <div class="flex justify-end gap-2 text-xl">
                    <button on:click=move |_| save_market_button_action.dispatch(())>
                        {save_market_button_text}
                    </button>
                    <button on:click=move |_| market_outpoint.set(None)>
                        "X"
                    </button>
                </div>
                <Market market_outpoint=create_memo(cx, move |_| market_outpoint.get().unwrap()) />
            </Show>

        </Show>
    }
}
