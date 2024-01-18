use fedimint_prediction_markets_common::Outcome;
use leptos::*;
use crate::context::ClientContext;

use fedimint_core::OutPoint;


#[component]
pub fn CandlestickChart<F>(cx: Scope, outpoint: F, outcome: RwSignal<Outcome>) -> impl IntoView
where
    F: Fn() -> OutPoint + Copy + 'static,
{
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    

    view! {
        cx,
        {outpoint().to_string()}
        <br />
        {outcome}
    }
}
