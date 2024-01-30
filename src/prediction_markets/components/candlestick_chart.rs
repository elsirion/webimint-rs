use std::collections::BTreeMap;
use std::time::Duration;

use fedimint_core::OutPoint;
use fedimint_prediction_markets_common::{
    Candlestick, ContractOfOutcomeAmount, Outcome, Seconds, UnixTimestamp,
};
use leptos::*;
use serde::Serialize;
use tracing::{info, warn};

use crate::context::ClientContext;
use crate::prediction_markets::js;

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
                chart_msg_stream.set(Some(ChartMsg::Data {
                    interval: this_candlestick_interval,
                    candlesticks: c,
                }));
            }
        });
    });

    let chart_ctx = create_rw_signal(cx, None);
    let chart_div = view! { cx, <div class="h-[500px]" /> }
        .id("prediction_markets_chart")
        .on_mount(move |_| {
            chart_ctx.set(Some(js::create_chart()));
        });

    create_effect::<ChartMsg>(cx, move |prev_msg| {
        let Some(msg) = chart_msg_stream.get() else {
            return ChartMsg::ClearChart;
        };
        let Some(ctx) = chart_ctx.get() else {
            warn!("CandlestickChart: Recieved ChartMsg before chart_ctx was ready");
            return ChartMsg::ClearChart;
        };
        if let ChartMsg::Data {
            interval: _,
            candlesticks: _,
        } = &msg
        {
            let data = serde_wasm_bindgen::to_value(&msg).unwrap();
            match prev_msg {
                Some(ChartMsg::Data {
                    interval: _,
                    candlesticks: _,
                }) => js::update_chart_data(ctx, data),
                _ => js::set_chart_data(ctx, data),
            }
        }

        msg
    });

    chart_div
}

#[derive(Debug, Clone, Serialize)]
enum ChartMsg {
    Data {
        interval: Seconds,
        candlesticks: BTreeMap<UnixTimestamp, Candlestick>,
    },
    ClearChart,
}
