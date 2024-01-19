use std::time::Duration;

use fedimint_core::OutPoint;
use fedimint_prediction_markets_common::{
    ContractOfOutcomeAmount, Outcome, Seconds, UnixTimestamp,
};
use leptos::*;
use tracing::info;

use crate::context::ClientContext;
use crate::prediction_markets::js;

#[component]
pub fn CandlestickChart(
    cx: Scope,
    market_outpoint: Memo<OutPoint>,
    outcome: RwSignal<Outcome>,
    candlestick_interval: RwSignal<Seconds>,
) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    let candlesticks_recieved = create_rw_signal(cx, Vec::new());

    let newest_candlestick_timestamp = create_rw_signal(cx, UnixTimestamp::ZERO);
    let newest_candlestick_volume = create_rw_signal(cx, ContractOfOutcomeAmount::ZERO);

    let candlestick_resource = create_resource(
        cx,
        move || (),
        move |()| async move {
            info!(
                "{:?} {:?}",
                newest_candlestick_timestamp.get_untracked(),
                newest_candlestick_volume.get_untracked()
            );

            client
                .get_value()
                .wait_candlesticks(
                    market_outpoint.get_untracked(),
                    outcome.get_untracked(),
                    candlestick_interval.get_untracked(),
                    newest_candlestick_timestamp.get_untracked(),
                    newest_candlestick_volume.get_untracked(),
                )
                .await
        },
    );

    create_effect(cx, move |_| {
        _ = market_outpoint.get();
        _ = outcome.get();
        _ = candlestick_interval.get();

        newest_candlestick_timestamp.set(UnixTimestamp::ZERO);
        newest_candlestick_volume.set(ContractOfOutcomeAmount::ZERO);

        candlestick_resource.refetch();
    });

    create_effect(cx, move |_| match candlestick_resource.read(cx) {
        Some(Ok(candlesticks)) => {
            if candlesticks.len() != 0 {
                for c in candlesticks.iter() {
                    candlesticks_recieved.update(|cr| cr.push((c.0.to_owned(), c.1.to_owned())))
                }

                let newest_candlestick = candlesticks.last_key_value().unwrap();
                newest_candlestick_timestamp.set(newest_candlestick.0.to_owned());
                newest_candlestick_volume.set(newest_candlestick.1.volume.to_owned());
            }

            set_timeout(
                move || candlestick_resource.refetch(),
                Duration::from_millis(600),
            );

            info!("{:?}", candlesticks)
        }
        _ => (),
    });

    let mut chart_div = view! { cx, <div />};
    chart_div = chart_div.id("chart");

    // js::create_chart();

    view! {
        cx,
        {market_outpoint.get_untracked().to_string()}
        <br />
        {outcome}
        <br />
        {move || candlesticks_recieved.get().into_iter().map(|c| format!("timestamp: {:?}\n candlestick: {:?}\n\n", c.0, c.1)).collect_view(cx)}
        {chart_div}
    }
}
