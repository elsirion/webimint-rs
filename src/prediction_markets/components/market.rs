use anyhow::anyhow;
use fedimint_core::OutPoint;
use fedimint_prediction_markets_common::config::GeneralConsensus;
use fedimint_prediction_markets_common::Outcome;
use leptos::*;

use crate::context::ClientContext;
use crate::prediction_markets::components::{CandlestickChart, NewOrder};
use crate::prediction_markets::helpers::unix_timestamp_to_js_string;
use crate::utils::empty_view;

#[component]
pub fn Market(cx: Scope, market_outpoint: Memo<OutPoint>) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);
    let general_consensus = expect_context::<GeneralConsensus>(cx);

    let get_market_resource = create_resource(
        cx,
        move || market_outpoint.get(),
        move |market: OutPoint| async move { client.get_value().get_market(market, false).await },
    );
    let get_market_result = move || match get_market_resource.read(cx) {
        Some(Ok(Some(m))) => Ok(m),
        Some(Ok(None)) => Err(anyhow!("market does not exist")),
        Some(Err(_)) => Err(anyhow!("issue getting market")),
        None => Err(anyhow!("action has not run")),
    };
    let market = create_memo(cx, move |_| get_market_result().ok());

    let outcome = create_rw_signal(cx, Outcome::from(0));

    let candlestick_interval = create_rw_signal(
        cx,
        general_consensus
            .candlestick_intervals
            .get(0)
            .unwrap()
            .to_owned(),
    );

    view! { cx,
        <Show
            when=move || matches!{market.get(), Some(_)}
            fallback=|_| empty_view()
        >
            <div class="border-[1px] p-2">
                <h1 class="text-2xl">{move || market.get().map(|m| m.information.title)}</h1>

                <div class="border-[1px] p-2">
                    <h2 class="border-b font-bold">"Description"</h2>
                    <p>{move || market.get().map(|m| m.information.description)}</p>
                </div>

                <table class="mt-2 p-2">
                    <tr>
                        <th class="border-[1px] p-2">"Contract price"</th>
                        <td class="border-[1px] p-2">{move || market.get().unwrap().contract_price.to_string()}</td>
                    </tr>
                    <tr>
                        <th class="border-[1px] p-2">"Payout control's fee per contract"</th>
                        <td class="border-[1px] p-2">{move || market.get().unwrap().payout_controls_fee_per_contract.to_string()}</td>
                    </tr>
                    <tr>
                        <th class="border-[1px] p-2">"Cumulative agreeing weight required for payout"</th>
                        <td class="border-[1px] p-2">{move || market.get().unwrap().weight_required_for_payout.to_string()}</td>
                    </tr>
                </table>

                <table class="mt-2 p-2">
                    <thead>
                        <th>"Payout Control Public Key"</th>
                        <th>"Weight"</th>
                    </thead>
                    {move || market.get().map(|m| {
                        m.payout_controls_weights
                            .into_iter()
                            .map(move |(k, v)| view! {
                                cx,
                                <tr>
                                    <td class="border-[1px] p-2">{k.to_string()}</td>
                                    <td class="border-[1px] p-2">{v}</td>
                                </tr>
                            })
                            .collect_view(cx)
                    })}
                </table>

                <p>"Expected payout time: "{move || unix_timestamp_to_js_string(market.get().unwrap().information.expected_payout_timestamp)}</p>

                <div class="flex">
                    {move || market.get().map(|m| {
                        m.information.outcome_titles.into_iter().enumerate().map(|(i, outcome_title)| {
                            view! {
                                cx,
                                <button
                                    class="border-2 border-black p-4"
                                    on:click=move |_| {outcome.set(i as Outcome)}
                                >
                                    {outcome_title}
                                </button>
                            }
                        }).collect_view(cx)
                    })}
                </div>

                <div class="flex">
                    {general_consensus.candlestick_intervals.iter().map(|ci| {
                            let ci = ci.to_owned();

                            view! {
                                cx,
                                <button
                                    class="border-2 border-black p-3"
                                    on:click=move |_| {candlestick_interval.set(ci)}
                                >
                                    {ci}"s"
                                </button>
                            }
                        }).collect_view(cx)}
                </div>

                <CandlestickChart market_outpoint=market_outpoint outcome=outcome candlestick_interval=candlestick_interval/>

                <NewOrder market_outpoint=market_outpoint outcome=outcome />
            </div>
        </Show>
    }
}
