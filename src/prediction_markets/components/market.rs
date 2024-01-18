use anyhow::anyhow;
use fedimint_core::OutPoint;
use fedimint_prediction_markets_common::Outcome;
use leptos::*;

use crate::context::ClientContext;
use crate::utils::empty_view;
use crate::prediction_markets::components::CandlestickChart;

#[component]
pub fn Market<F>(cx: Scope, outpoint: F) -> impl IntoView
where
    F: Fn() -> OutPoint + Copy + 'static,
{
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    let get_market_resource = create_resource(
        cx,
        move || outpoint(),
        move |market: OutPoint| async move { client.get_value().get_market(market, false).await },
    );
    let get_market_result = move || match get_market_resource.read(cx) {
        Some(Ok(Some(m))) => Ok(m),
        Some(Ok(None)) => Err(anyhow!("market does not exist")),
        Some(Err(_)) => Err(anyhow!("issue getting market")),
        None => Err(anyhow!("action has not run")),
    };
    let market = move || get_market_result().ok();

    let title = move || {
        view! {
            cx,
            <h1 class="text-2xl">{market().map(|m| m.information.title)}</h1>
        }
    };

    let payout_controls = move || {
        view! {
            cx,
            <table class="p-2 bor">
                <thead>
                    <th>"Payout Control Public Key"</th>
                    <th>"Weight"</th>
                </thead>
                {market().map(|m| {
                    m.payout_controls_weights
                        .into_iter()
                        .map(move |(k, v)| view! {
                            cx,
                            <tr class="border-b">
                                <td>{k.to_string()}</td>
                                <td>{v}</td>
                            </tr>
                        })
                        .collect_view(cx)
                })}
            </table>
        }
    };

    let selected_outcome = create_rw_signal(cx, Outcome::from(0));

    let outcomes = move || {
        view! {
            cx,
            <div class="flex">
                {market().map(|m| {
                    m.information.outcome_titles.into_iter().enumerate().map(|(outcome, outcome_title)| {
                        view! {
                            cx,
                            <div on:click=move |_| {selected_outcome.set(outcome as Outcome)} class="p-4">{outcome_title}</div>
                        }
                    }).collect_view(cx)
                })}
            </div>
        }
    };

    view! { cx,
        <Show
            when=move || matches!{market(), Some(_)}
            fallback=|_| empty_view()
        >
            {title}
            {payout_controls}
            {outcomes}
            <CandlestickChart outpoint=outpoint outcome=selected_outcome/>
        </Show>
    }
}
