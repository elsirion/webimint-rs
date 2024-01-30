use std::collections::BTreeMap;
use std::str::FromStr;

use fedimint_core::{OutPoint, TransactionId};
use fedimint_prediction_markets_common::UnixTimestamp;
use leptos::*;

use crate::context::ClientContext;
use crate::utils::empty_view;
use crate::prediction_markets::helpers::unix_timestamp_to_js_string;

#[component]
pub fn SelectMarket(
    cx: Scope,
    market_outpoint: RwSignal<Option<OutPoint>>,
    saved_markets: Memo<Option<Vec<(OutPoint, UnixTimestamp)>>>,
) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    let form_market_txid = move |txid: String| {
        let Ok(txid) = TransactionId::from_str(txid.as_ref()) else {
            return;
        };
        market_outpoint.set(Some(OutPoint { txid, out_idx: 0 }));
    };

    let saved_markets_sorted_by_saved_timestamp = create_memo(cx, move |_| {
        saved_markets.get().map(|mut v| {
            v.sort_by(|(_, a), (_, b)| b.cmp(a));

            v
        })
    });

    view! {
        cx,
        <div class="flex items-center gap-2 border-[1px] p-2">
            <label>"Go to market by txid:"</label>
            <input type="text" class="flex-grow" on:input=move |ev| form_market_txid(event_target_value(&ev)) />
        </div>
        <Show
            when=move || matches!{saved_markets.get(), Some(_)}
            fallback=|_| empty_view()
        >
            <div class="flex-col border-[1px] p-2 mt-1">
                <h1 class="text-center border-b text-lg">"Saved markets"</h1>
                <table class="p-2 w-[100%]">
                    <thead>
                        <th>"Market"</th>
                        <th>"Saved Timestamp"</th>
                    </thead>
                    <For
                        each=move || saved_markets_sorted_by_saved_timestamp.get().unwrap()
                        key=|(saved_market_outpoint, _)| saved_market_outpoint.to_owned()
                        view=move |cx, (saved_market_outpoint, saved_market_saved_timestamp)| {
                            let market_resource = create_resource(
                                cx,
                                || (),
                                move |()| async move {
                                    client
                                        .get_value()
                                        .get_market(saved_market_outpoint, true)
                                        .await
                                },
                            );
                            let get_market_name = move || {
                                market_resource.read(cx).map(|market| {
                                    Some(market.ok()??.information.title)
                                })
                                .flatten()
                                .unwrap_or(saved_market_outpoint.to_string())

                            };

                            view!{
                                cx,
                                <tr
                                    class="cursor-pointer"
                                    on:click=move |_| market_outpoint.set(Some(saved_market_outpoint))
                                >
                                    <td class="border-[1px] p-2">{get_market_name}</td>
                                    <td class="border-[1px] p-2">{unix_timestamp_to_js_string(saved_market_saved_timestamp)}</td>
                                </tr>
                            }
                        }
                    />
                </table>
            </div>
        </Show>
    }
}
