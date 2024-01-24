use std::collections::BTreeMap;
use std::time::Duration;

use fedimint_core::OutPoint;
use fedimint_prediction_markets_common::{
    Candlestick, ContractOfOutcomeAmount, Outcome, Seconds, UnixTimestamp,
};
use leptos::*;
use tracing::info;

use crate::context::ClientContext;

const DELAY_BETWEEN_CANDLESTICK_REQUESTS: Duration = Duration::from_millis(500);

#[component]
pub fn CandlestickChart(
    cx: Scope,
    market_outpoint: Memo<OutPoint>,
    outcome: RwSignal<Outcome>,
    candlestick_interval: RwSignal<Seconds>,
) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    let chart_msg_stream: RwSignal<Option<ChartMsg>> = create_rw_signal(cx, None);
    let params_incrementer = create_rw_signal(cx, 0u32);

    create_effect(cx, move |prev| {
        let this_market_outpoint = market_outpoint.get();
        let this_outcome = outcome.get();
        let this_candlestick_interval = candlestick_interval.get();

        params_incrementer.set_untracked(params_incrementer.get_untracked() + 1);
        let params_id = params_incrementer.get_untracked();

        if prev.is_some() {
            chart_msg_stream.set(Some(ChartMsg::ClearChart));
        }

        let candlestick_timestamp = create_rw_signal(cx, UnixTimestamp::ZERO);
        let candlestick_volume = create_rw_signal(cx, ContractOfOutcomeAmount::ZERO);

        let candlestick_resource = create_resource(
            cx,
            move || (),
            move |_| async move {
                let r = client
                    .get_value()
                    .wait_candlesticks(
                        this_market_outpoint,
                        this_outcome,
                        this_candlestick_interval,
                        candlestick_timestamp.get_untracked(),
                        candlestick_volume.get_untracked(),
                    )
                    .await;

                if let Ok(candlesticks) = &r {
                    if let Some(last) = candlesticks.last_key_value() {
                        candlestick_timestamp.set_untracked(last.0.to_owned());
                        candlestick_volume.set_untracked(last.1.volume.to_owned());
                    }
                }

                r
            },
        );

        create_effect(cx, move |_| {
            if candlestick_resource.loading().get() == false
                && params_id == params_incrementer.get_untracked()
            {
                set_timeout(
                    move || candlestick_resource.refetch(),
                    DELAY_BETWEEN_CANDLESTICK_REQUESTS,
                );
            }
        });

        create_effect(cx, move |_| {
            let Some(Ok(c)) = candlestick_resource.read(cx) else {
                return;
            };

            if params_id == params_incrementer.get_untracked() {
                chart_msg_stream.set(Some(ChartMsg::Candelsticks(c)))
            }
        });
    });

    create_effect(cx, move |_| {
        if let Some(msg) = chart_msg_stream.get() {
            info!("Chart message recieved: {:?}", msg)
        }
    });

    let mut chart_div = view! { cx, <div />};
    chart_div = chart_div.id("chart");

    // js::create_chart();

    view! {
        cx,
        {chart_div}
    }
}

#[derive(Debug, Clone)]
enum ChartMsg {
    Candelsticks(BTreeMap<UnixTimestamp, Candlestick>),
    ClearChart,
}
