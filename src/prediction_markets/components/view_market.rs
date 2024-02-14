use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;
use std::time::Duration;
use std::vec;

use anyhow::anyhow;
use fedimint_core::{Amount, OutPoint, TransactionId};
use fedimint_prediction_markets_common::{
    Candlestick, ContractOfOutcomeAmount, Order, OrderIdClientSide, Outcome, Seconds, Side,
    SignedAmount, UnixTimestamp,
};
use leptos::*;
use secp256k1::PublicKey;
use serde::Serialize;
use tracing::warn;

use super::PredictionMarketsStaticDataContext;
use crate::context::ClientContext;
use crate::prediction_markets::helpers::unix_timestamp_to_js_string;
use crate::prediction_markets::js;
use crate::utils::empty_view;

#[component]
pub fn ViewMarket() -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>();

    let market_outpoint: RwSignal<Option<OutPoint>> = create_rw_signal(None);

    let get_saved_markets_resource = create_resource(
        || (()),
        move |()| async move { client.get_value().get_saved_markets().await },
    );
    let saved_markets_set_memo = create_memo(move |_| {
        let Some(Ok(saved_markets)) = get_saved_markets_resource.get() else {
            return None;
        };
        let set = saved_markets
            .into_iter()
            .map(|(k, _)| (k, ()))
            .collect::<HashMap<OutPoint, ()>>();

        Some(set)
    });
    let market_saved = create_memo(move |_| {
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

    let save_market_button_action = create_action(move |()| {
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
        <Show
            when=move || matches!{get_saved_markets_resource.get(), Some(_)}
            fallback=|| empty_view()
        >

            <Show
                when=move || matches!{market_outpoint.get(), None}
                fallback=|| empty_view()
            >
                <SelectMarket
                    market_outpoint=market_outpoint
                    saved_markets=create_memo(move |_| {
                        get_saved_markets_resource
                            .get()
                            .map(|r| r.map_err(|e| warn!("Error getting saved markets: {e:?}"))
                            .unwrap()
                        )
                    })
                />
            </Show>
            <Show
                when=move || matches!{market_outpoint.get(), Some(_)}
                fallback=|| empty_view()
            >
                <div class="flex flex-col gap-2">
                    <div class="flex justify-end gap-2">
                        <button
                            class="border-[1px] p-2 cursor-pointer text-lg"
                            on:click=move |_| save_market_button_action.dispatch(())
                        >
                            {save_market_button_text}
                        </button>
                        <button
                            class="border-[1px] p-2 cursor-pointer text-lg"
                            on:click=move |_| market_outpoint.set(None)
                        >
                            "X"
                        </button>
                    </div>
                    <Market market_outpoint=create_memo(move |_| market_outpoint.get().unwrap()) />
                </div>
            </Show>

        </Show>
    }
}

#[component]
pub fn SelectMarket(
    market_outpoint: RwSignal<Option<OutPoint>>,
    saved_markets: Memo<Option<Vec<(OutPoint, UnixTimestamp)>>>,
) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>();

    #[derive(Debug, Clone, PartialEq)]
    enum Tabs {
        SavedMarkets,
        ClientPayoutControlMarkets,
    }
    let current_tab = create_rw_signal(Tabs::SavedMarkets);

    let form_market_txid = move |txid: String| {
        let Ok(txid) = TransactionId::from_str(txid.as_ref()) else {
            return;
        };
        market_outpoint.set(Some(OutPoint { txid, out_idx: 0 }));
    };

    let saved_markets_sorted_by_saved_timestamp = create_memo(move |_| {
        saved_markets.get().map(|mut v| {
            v.sort_by(|(_, a), (_, b)| b.cmp(a));

            v
        })
    });

    let client_payout_control_markets_resource = create_resource(
        || (),
        move |()| async move {
            client
                .get_value()
                .get_client_payout_control_markets(false, UnixTimestamp::ZERO)
                .await
        },
    );
    let client_payout_control_markets_sorted_by_creation_timestamp = create_memo(move |_| {
        client_payout_control_markets_resource.with(move |r| {
            let mut v = vec![];
            let Some(Ok(btree)) = r else { return v };

            for (creation, market_set) in btree.iter().rev() {
                for market in market_set {
                    v.push((market.to_owned(), creation.to_owned()))
                }
            }

            v
        })
    });

    view! {
        <div class="flex flex-col gap-2">
            <div class="flex items-center gap-2 border-[1px] p-2">
                <label>"Go to market by txid:"</label>
                <input type="text" class="flex-grow" on:input=move |ev| form_market_txid(event_target_value(&ev)) />
            </div>

            <div class="flex">
                <button
                    class={move || format!("flex-grow border-b-2 p-3 font-body font-semibold leading-tight hover:text-blue-500 {}",
                        if current_tab.get() == Tabs::SavedMarkets {"text-blue-400 border-blue-400"} else {"text-gray-400 border-gray-200 hover:border-gray-700"})}
                    on:click=move |_| current_tab.set(Tabs::SavedMarkets)
                >
                    "Saved Markets"
                </button>
                <button
                    class={move || format!("flex-grow border-b-2 p-3 font-body font-semibold leading-tight hover:text-blue-500 {}",
                        if current_tab.get() == Tabs::ClientPayoutControlMarkets {"text-blue-400 border-blue-400"} else {"text-gray-400 border-gray-200 hover:border-gray-700"})}
                    on:click=move |_| current_tab.set(Tabs::ClientPayoutControlMarkets)
                >
                    "Client Payout Control Markets"
                </button>
            </div>

            <Show
                when=move || matches!{current_tab.get(), Tabs::SavedMarkets} && matches!{saved_markets.get(), Some(_)}
                fallback=|| empty_view()
            >
                <table>
                    <thead>
                        <th class="border-[1px] p-2">"Market"</th>
                        <th class="border-[1px] p-2">"Saved Timestamp"</th>
                    </thead>
                    <For
                        each=move || saved_markets_sorted_by_saved_timestamp.get().unwrap()
                        key=|(market_outpoint, _)| market_outpoint.to_owned()
                        children=move |(saved_market_outpoint, saved_market_saved_timestamp)| {
                            let market_resource = create_resource(
                                || (),
                                move |()| async move {
                                    client
                                        .get_value()
                                        .get_market(saved_market_outpoint, true)
                                        .await
                                },
                            );
                            let get_market_name = move || {
                                market_resource.get().map(|market| {
                                    Some(market.ok()??.information.title)
                                })
                                .flatten()
                                .unwrap_or(saved_market_outpoint.to_string())

                            };

                            view!{
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
            </Show>

            <Show
                when=move || matches!{current_tab.get(), Tabs::ClientPayoutControlMarkets}
                    && matches!{client_payout_control_markets_resource.get(), Some(_)}
                fallback=|| empty_view()
            >
                <table>
                    <thead>
                        <th class="border-[1px] p-2">"Market"</th>
                        <th class="border-[1px] p-2">"Creation Timestamp"</th>
                    </thead>
                    <For
                        each=move || client_payout_control_markets_sorted_by_creation_timestamp.get()
                        key=|(market_outpoint, _)| market_outpoint.to_owned()
                        children=move |(client_payout_control_market_outpoint, client_payout_control_market_creation_timestamp)| {
                            let market_resource = create_resource(
                                || (),
                                move |()| async move {
                                    client
                                        .get_value()
                                        .get_market(client_payout_control_market_outpoint, true)
                                        .await
                                },
                            );
                            let get_market_name = move || {
                                market_resource.get().map(|market| {
                                    Some(market.ok()??.information.title)
                                })
                                .flatten()
                                .unwrap_or(client_payout_control_market_outpoint.to_string())

                            };

                            view!{
                                <tr
                                    class="cursor-pointer"
                                    on:click=move |_| market_outpoint.set(Some(client_payout_control_market_outpoint))
                                >
                                    <td class="border-[1px] p-2">{get_market_name}</td>
                                    <td class="border-[1px] p-2">{unix_timestamp_to_js_string(client_payout_control_market_creation_timestamp)}</td>
                                </tr>
                            }
                        }
                    />
                </table>
            </Show>
        </div>
    }
}

#[component]
pub fn Market(market_outpoint: Memo<OutPoint>) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>();
    let PredictionMarketsStaticDataContext {
        client_payout_control,
        general_consensus: _,
    } = expect_context::<PredictionMarketsStaticDataContext>();

    let run_sync_orders = create_rw_signal(());
    let outcome_stats: RwSignal<Vec<(ContractOfOutcomeAmount, SignedAmount)>> =
        create_rw_signal(vec![]);

    let get_market_resource = create_resource(
        move || market_outpoint.get(),
        move |market: OutPoint| async move { client.get_value().get_market(market, false).await },
    );
    let get_market_result = move || match get_market_resource.get() {
        Some(Ok(Some(m))) => Ok(m),
        Some(Ok(None)) => Err(anyhow!("market does not exist")),
        Some(Err(_)) => Err(anyhow!("issue getting market")),
        None => Err(anyhow!("action has not run")),
    };
    let market = create_memo(move |_| get_market_result().ok());

    let outcome = create_rw_signal(Outcome::from(0));

    let name_to_payout_control_map_resource = create_resource(
        || (),
        move |()| async move { client.get_value().get_name_to_payout_control_map().await },
    );
    let payout_control_to_name_memo = create_memo(move |_| {
        let Some(Ok(m)) = name_to_payout_control_map_resource.get() else {
            return None;
        };

        let reversed_m = m
            .into_iter()
            .map(|(k, v)| (v, k))
            .collect::<HashMap<_, _>>();

        Some(reversed_m)
    });
    let payout_control_string = move |payout_control: PublicKey| {
        payout_control_to_name_memo
            .with(move |m_opt| {
                let Some(m) = m_opt else {
                    return None;
                };

                m.get(&payout_control).map(|name| name.to_owned())
            })
            .unwrap_or(payout_control.to_string())
    };

    let market_payout_control_proposals_resource = create_resource(
        move || market_outpoint.get(),
        move |market_outpoint| async move {
            client
                .get_value()
                .get_market_payout_control_proposals(market_outpoint, false)
                .await
        },
    );
    let payout_control_proposal_string = move |payout_control: PublicKey| {
        market_payout_control_proposals_resource.with(move |r| {
            let Some(Ok(m)) = r else {
                return "loading...".to_owned();
            };
            match m.get(&payout_control) {
                Some(v) => {
                    let mut s = String::new();
                    for a in v {
                        s.push_str(format!("{a}, ").as_ref())
                    }
                    s.pop();
                    s.pop();

                    s
                }
                None => "No vote".to_owned(),
            }
        })
    };

    let form_outcome_payouts = create_memo(move |_| {
        market.get().map(move |m| {
            let mut v = vec![];
            for _ in 0..m.outcomes {
                v.push(create_rw_signal("".to_owned()))
            }
            v
        })
    });
    let propose_payout_action = create_action(move |()| async move {
        let market_outpoint = market_outpoint.get();
        let Some(outcome_payouts_strings) = form_outcome_payouts.get() else {
            return Err("failed getting outcome payouts".to_owned());
        };

        let mut outcome_payouts = vec![];
        for s in outcome_payouts_strings {
            let Ok(a) = Amount::from_str(s.get().as_ref()) else {
                return Err("failed to pare outcome payout amount".to_owned());
            };

            outcome_payouts.push(a);
        }

        let r = client
            .get_value()
            .propose_payout(market_outpoint, outcome_payouts)
            .await
            .map_err(|e| format!("{e:?}"));

        if matches!(r, Ok(_)) {
            get_market_resource.refetch();
            market_payout_control_proposals_resource.refetch();
        }

        r
    });

    view! {
        <Show
            when=move || matches!{market.get(), Some(_)}
            fallback=|| empty_view()
        >
            <div class="flex flex-col content-center gap-2 border p-2">
                <h1 class="text-2xl">{move || market.get().map(|m| m.information.title)}</h1>

                <div class="border-[1px] p-2">
                    <h2 class="border-b font-bold">"Description"</h2>
                    <p>{move || market.get().map(|m| m.information.description)}</p>
                </div>

                <table>
                    <tr>
                        <th class="border-[1px] p-2">"ID"</th>
                        <td class="border-[1px] p-2 text-xs">{move || market_outpoint.get().txid.to_string()}</td>
                    </tr>
                    <tr>
                        <th class="border-[1px] p-2">"Contract price"</th>
                        <td class="border-[1px] p-2">{move || market.get().unwrap().contract_price.to_string()}</td>
                    </tr>
                    <tr>
                        <th class="border-[1px] p-2">"Payout control's fee per contract"</th>
                        <td class="border-[1px] p-2">{move || market.get().unwrap().payout_controls_fee_per_contract.to_string()}</td>
                    </tr>
                    <tr>
                        <th class="border-[1px] p-2">"Current open contracts"</th>
                        <td class="border-[1px] p-2">{move || market.get().unwrap().open_contracts.0.to_string()}</td>
                    </tr>
                    <tr>
                        <th class="border-[1px] p-2">"Cumulative agreeing weight required for payout"</th>
                        <td class="border-[1px] p-2">{move || market.get().unwrap().weight_required_for_payout.to_string()}</td>
                    </tr>
                </table>

                <table class="overflow-auto">
                    <thead>
                        <th class="border p-1 text-sm">"Payout Control Public Key"</th>
                        <th class="border p-1 text-sm">"Weight"</th>
                        <th class="border p-1 text-sm">"Current Payout Proposal"</th>
                    </thead>
                    {move || market.get().map(|m| {
                        m.payout_controls_weights
                            .into_iter()
                            .map(move |(public_key, weight)| view! {
                                <tr>
                                    <td class="border p-1 text-sm">{payout_control_string(public_key)}</td>
                                    <td class="border p-1 text-sm">{weight}</td>
                                    <td class="border p-1 text-sm">{payout_control_proposal_string(public_key)}</td>
                                </tr>
                            })
                            .collect_view()
                    })}
                </table>

                <div class="flex flex-col gap-1 border p-2">
                    <Show
                        when=move || matches!{market.get().unwrap().payout, None}
                        fallback=|| empty_view()
                    >
                        <h2 class="font-bold">"A payout has not yet occured."</h2>
                        <p>"The market is expected to payout at "{move || unix_timestamp_to_js_string(market.get().unwrap().information.expected_payout_timestamp)}"."</p>

                        <Show
                            when=move || market.get().unwrap().payout_controls_weights.contains_key(&client_payout_control)
                            fallback=|| empty_view()
                        >
                            <h2 class="pt-3 font-bold">"Propose a payout"</h2>
                            <table class="overflow-auto">
                                <thead>
                                    {move || market.get().unwrap().information.outcome_titles.into_iter()
                                        .map(
                                            move |t| {
                                                view! {
                                                    <th class="border p-1">{t}</th>
                                                }
                                            }
                                        ).collect_view()
                                    }
                                </thead>
                                <tr>
                                    {move || form_outcome_payouts.get()
                                        .map(|outcome_payouts| outcome_payouts.into_iter()
                                            .map(move |outcome_payout| {
                                                    view! {
                                                        <td class="border p-1">
                                                            <input
                                                                type="number"
                                                                class="p-1 w-[100%]"
                                                                on:input=move |ev| outcome_payout.set(event_target_value(&ev))
                                                                prop:value=move || outcome_payout.get()
                                                            />
                                                        </td>
                                                    }
                                                }
                                            ).collect_view()
                                        )
                                    }
                                </tr>
                            </table>
                            <button
                                class="border-[1px] p-2 hover:bg-slate-200"
                                on:click=move |_| {
                                    propose_payout_action.value().set(None);
                                    propose_payout_action.dispatch(());
                                }
                            >
                                "Propose Payout"
                            </button>
                            <p>
                                {move || propose_payout_action.value().get().map(|v| {
                                    match v {
                                        Ok(_) => format!("Successfully proposed payout"),
                                        Err(e) => format!("Error proposing payout: {e}")
                                    }
                                })}
                            </p>
                        </Show>
                    </Show>

                    <Show
                        when=move || matches!{market.get().unwrap().payout, Some(_)}
                        fallback=|| empty_view()
                    >
                        <h2 class="font-bold">"The market payout has occured."</h2>
                        <p>"The payout occured at "{unix_timestamp_to_js_string(market.get().unwrap().payout.unwrap().occurred_consensus_timestamp)}"."</p>
                        <table class="overflow-auto">
                            <thead>
                                {move || market.get().unwrap().information.outcome_titles.into_iter()
                                    .map(move |t| {
                                        view! {
                                            <th class="border p-1">{t}</th>
                                        }
                                    }).collect_view()
                                }
                            </thead>
                            <tr>
                                {move || market.get().unwrap().payout.unwrap().outcome_payouts.into_iter()
                                    .map(
                                        move |a| {
                                            view! {
                                                <td class="border p-1">{format!("{a}")}</td>
                                            }
                                        }
                                    ).collect_view()
                                }
                            </tr>
                        </table>
                    </Show>
                </div>

                <div class="flex">
                    {move || market.get().map(|m| {
                        m.information.outcome_titles.into_iter().enumerate().map(|(i, outcome_title)| {
                            view! {
                                <button
                                    class={format!("border-2 p-4 border-black {}", if outcome.get() == i as u8 {"bg-slate-100"} else {"bg-slate-400"})}
                                    on:click=move |_| {outcome.set(i as Outcome)}
                                >
                                    {outcome_title}
                                </button>
                            }
                        }).collect_view()
                    })}
                </div>

                <CandlestickChart market_outpoint=market_outpoint outcome=outcome />
                <NewOrder market_outpoint=market_outpoint outcome=outcome run_sync_orders=run_sync_orders market=market />
                <AccountStats market=market outcome_stats=outcome_stats />
                <MarketOrdersTable market_outpoint=market_outpoint outcome=outcome run_sync_orders=run_sync_orders market=market outcome_stats=outcome_stats />
            </div>
        </Show>
    }
}

#[component]
pub fn CandlestickChart(
    market_outpoint: Memo<OutPoint>,
    outcome: RwSignal<Outcome>,
) -> impl IntoView {
    const DELAY_BETWEEN_CANDLESTICK_REQUESTS: Duration = Duration::from_millis(500);

    let ClientContext { client, .. } = expect_context::<ClientContext>();
    let PredictionMarketsStaticDataContext {
        client_payout_control: _,
        general_consensus,
    } = expect_context::<PredictionMarketsStaticDataContext>();

    #[derive(Debug, Clone, Serialize)]
    enum ChartMsg {
        Data {
            interval: Seconds,
            candlesticks: BTreeMap<UnixTimestamp, Candlestick>,
        },
        ClearChart,
    }

    let candlestick_interval = create_rw_signal(
        general_consensus
            .candlestick_intervals
            .get(0)
            .unwrap()
            .to_owned(),
    );

    let chart_msg_stream: RwSignal<Option<ChartMsg>> = create_rw_signal(None);
    let params_incrementer = create_rw_signal(0u32);

    create_effect(move |prev| {
        let this_market_outpoint = market_outpoint.get();
        let this_outcome = outcome.get();
        let this_candlestick_interval = candlestick_interval.get();

        params_incrementer.update_untracked(|p| *p += 1);
        let params_id = params_incrementer.get_untracked();

        if prev.is_some() {
            chart_msg_stream.set(Some(ChartMsg::ClearChart));
        }

        let candlestick_timestamp = create_rw_signal(UnixTimestamp::ZERO);
        let candlestick_volume = create_rw_signal(ContractOfOutcomeAmount::ZERO);

        let candlestick_resource = create_resource(
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

        create_effect(move |_| {
            if candlestick_resource.loading().get() == false
                && params_id == params_incrementer.get_untracked()
            {
                set_timeout(
                    move || candlestick_resource.refetch(),
                    DELAY_BETWEEN_CANDLESTICK_REQUESTS,
                );
            }
        });

        create_effect(move |_| {
            let Some(Ok(c)) = candlestick_resource.get() else {
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

    let chart_ctx = create_rw_signal(None);
    let chart_div = view! {
        <div class="relative">
            <div
                class="h-[500px]"
                prop:id="prediction_markets_chart"
            />
            <div class="absolute left-3 top-3 text-sm text-white z-10">
                <span prop:id="prediction_markets_chart_candlestick_series_info"/>
                <br />
                <span prop:id="prediction_markets_chart_volume_series_info"/>
            </div>
        </div>
    }
    .on_mount(move |_| {
        chart_ctx.set(Some(js::create_chart()));
    });

    create_effect::<ChartMsg>(move |prev_msg| {
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

    view! {
        <div class="border p-1">
            <div class="flex">
                {general_consensus.candlestick_intervals.iter().map(|ci| {
                    let ci = ci.to_owned();

                    view! {
                        <button
                            class={move || format!("border-2 p-3 border-black {}", if candlestick_interval.get() == ci {"bg-slate-100"} else {"bg-slate-400"})}
                            on:click=move |_| {candlestick_interval.set(ci)}
                        >
                            {ci}"s"
                        </button>
                    }
                }).collect_view()}
            </div>
            {chart_div}
        </div>
    }
}

#[component]
pub fn NewOrder(
    market_outpoint: Memo<OutPoint>,
    outcome: RwSignal<Outcome>,
    run_sync_orders: RwSignal<()>,
    market: Memo<Option<fedimint_prediction_markets_common::Market>>,
) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>();

    let form_side = create_rw_signal(Side::Buy);
    let form_price = create_rw_signal("".to_owned());
    let form_quantity = create_rw_signal("".to_owned());

    let new_order_action: Action<(), Result<OrderIdClientSide, String>> =
        create_action(move |()| async move {
            let market = market_outpoint.get();
            let outcome = outcome.get();
            let side: Side = form_side.get_untracked();
            let price = Amount::from_msats(
                form_price
                    .get_untracked()
                    .parse::<u64>()
                    .map_err(|e| format!("failed to parse price: {}", e))?,
            );
            let quantity = ContractOfOutcomeAmount(
                form_quantity
                    .get_untracked()
                    .parse()
                    .map_err(|e| format!("failed to parse quantity: {}", e))?,
            );

            form_quantity.set("".to_owned());

            let id = client
                .get_value()
                .new_order(market, outcome, side, price, quantity)
                .await
                .map_err(|e| format!("{e:?}"))?;

            run_sync_orders.set(());

            Ok(id)
        });

    view! {
        <div class="flex flex-col gap-2 border p-1">
            <h2 class="border-b font-bold">"Create New Order"</h2>

            <p>
                "Selected outcome: "
                {
                    move || {
                        market.get().map(
                            move |m| m.information.outcome_titles.get(outcome.get() as usize).unwrap().to_owned()
                        )
                        .unwrap_or(outcome.get().to_string())
                    }
                }
            </p>

            <div>
                <button
                    class={move || format!("p-2 border-2 border-black {}", if form_side.get() == Side::Buy {"bg-green-500"} else {"bg-gray-400"})}
                    on:click=move |_| form_side.set(Side::Buy)
                >
                    "Buy"
                </button>
                <button
                    class={move || format!("p-2 border-2 border-black {}", if form_side.get() == Side::Sell {"bg-red-500"} else {"bg-gray-400"})}
                    on:click=move |_| form_side.set(Side::Sell)
                >
                    "Sell"
                </button>
            </div>

            <div class="flex gap-2">
                <label>"Price"</label>
                <input
                    type="number"
                    class="flex-grow"
                    on:input=move |ev| form_price.set(event_target_value(&ev))
                    prop:value=move || form_price.get()
                />
            </div>

            <div class="flex gap-2">
                <label>"Quantity"</label>
                <input
                    type="number"
                    class="flex-grow"
                    on:input=move |ev| form_quantity.set(event_target_value(&ev))
                    prop:value=move || form_quantity.get()
                />
            </div>

            <button
                class="border-[1px] p-2 hover:bg-slate-200"
                on:click=move |_| {
                    new_order_action.value().set(None);
                    new_order_action.dispatch(());
                }
            >
                "Create Order"
            </button>

            <span>
                {move || new_order_action.value().get().map(|r| {
                    match r {
                        Ok(id) => format!("New order created. ID: {}", id.0),
                        Err(e) => format!("Error creating new order: {e}")
                    }
                })}
            </span>
        </div>
    }
}

#[component]
pub fn AccountStats(
    market: Memo<Option<fedimint_prediction_markets_common::Market>>,
    outcome_stats: RwSignal<Vec<(ContractOfOutcomeAmount, SignedAmount)>>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2 border p-1">
            <h2 class="border-b font-bold">"Your Market Holdings and Stats"</h2>
            <table class="overflow-auto">
                <thead>
                    <th></th>
                    {move || market.get().map(
                        move |m| m.information.outcome_titles.iter().map(
                            move |outcome_title| {
                                view! {
                                    <th class="border p-1 text-sm">{outcome_title}</th>
                                }
                            }
                        )
                        .collect_view()
                    )}
                </thead>
                <tr>
                    <th class="border p-1 text-sm">"Owned Contracts per Outcome"</th>
                    {move || outcome_stats.get().into_iter()
                        .map(move |(owned_contracts, _)| {
                            view! {
                                <td class="border p-1 text-sm">{owned_contracts.0}</td>
                            }
                        })
                        .collect_view()
                    }
                </tr>
                <tr>
                    <th class="border p-1 text-sm">"Profit per Outcome"</th>
                    {move || outcome_stats.get().into_iter()
                        .map(move |(_, profit)| {
                            view! {
                                <td class="border p-1 text-sm">{profit.to_string()}</td>
                            }
                        })
                        .collect_view()
                    }
                </tr>
                <tr>
                    <th class="border p-1 text-sm">"Total Profit"</th>
                    {move || {
                        let mut total_profit = SignedAmount::ZERO;
                        for (_, profit) in outcome_stats.get() {
                            total_profit = total_profit + profit;
                        }
                        view! {
                            <td class="border p-1 text-sm">{total_profit.to_string()}</td>
                        }
                    }}
                </tr>
            </table>
        </div>
    }
}

#[component]
pub fn MarketOrdersTable(
    market_outpoint: Memo<OutPoint>,
    outcome: RwSignal<Outcome>,
    run_sync_orders: RwSignal<()>,
    market: Memo<Option<fedimint_prediction_markets_common::Market>>,
    outcome_stats: RwSignal<Vec<(ContractOfOutcomeAmount, SignedAmount)>>,
) -> impl IntoView {
    const DELAY_BETWEEN_SYNC_ORDER_REQUESTS: Duration = Duration::from_secs(15);

    let ClientContext { client, .. } = expect_context::<ClientContext>();

    let order_cache = create_rw_signal(BTreeMap::<OrderIdClientSide, RwSignal<Order>>::new());
    let form_only_active_orders = create_rw_signal(true);
    let form_only_selected_outcome_orders = create_rw_signal(false);

    create_resource(
        move || market_outpoint.get(),
        move |market_outpoint| async move {
            match client
                .get_value()
                .get_orders_from_db(Some(market_outpoint), None)
                .await
            {
                Ok(market_orders) => {
                    order_cache.set(
                        market_orders
                            .into_iter()
                            .map(|(k, v)| (k, create_rw_signal(v)))
                            .collect(),
                    );
                }
                Err(e) => warn!("Error getting orders from db: {e}"),
            }
        },
    );

    let sync_orders_action = create_action(move |market_outpoint: &OutPoint| {
        let market_outpoint = market_outpoint.to_owned();
        async move {
            match client
                .get_value()
                .sync_orders(false, Some(market_outpoint), None)
                .await
            {
                Ok(synced_orders) => order_cache.update(move |order_cache| {
                    for (id, order) in synced_orders.into_iter() {
                        match order_cache.get(&id) {
                            None => {
                                order_cache.insert(id, create_rw_signal(order));
                            }
                            Some(s) => {
                                s.set(order);
                            }
                        }
                    }
                }),
                Err(e) => warn!("Error syncing market orders: {e}"),
            }
        }
    });

    let sync_orders_interval_handle = set_interval_with_handle(
        move || sync_orders_action.dispatch(market_outpoint.get_untracked()),
        DELAY_BETWEEN_SYNC_ORDER_REQUESTS,
    );
    on_cleanup(move || {
        let Ok(h) = sync_orders_interval_handle else {
            return;
        };
        h.clear()
    });

    create_effect(move |_| {
        _ = run_sync_orders.get();

        sync_orders_action.dispatch(market_outpoint.get_untracked());
    });

    let filtered_orders = create_memo(move |_| {
        let market_outpoint = market_outpoint.get();
        let outcome = outcome.get();
        let only_active_orders = form_only_active_orders.get();
        let only_selected_outcome_orders = form_only_selected_outcome_orders.get();

        order_cache.with(move |order_cache| {
            order_cache
                .iter()
                .filter(move |(_, order_signal)| {
                    let order = order_signal.get_untracked();

                    order.market == market_outpoint
                        && (!only_active_orders
                            || order.quantity_waiting_for_match != ContractOfOutcomeAmount::ZERO)
                        && (!only_selected_outcome_orders || order.outcome == outcome)
                })
                .map(|(id, order_signal)| (id.to_owned(), order_signal.to_owned()))
                .collect::<BTreeMap<OrderIdClientSide, RwSignal<Order>>>()
        })
    });

    let cancel_order_action = create_action(move |id: &OrderIdClientSide| {
        let id = id.to_owned();

        async move {
            _ = client
                .get_value()
                .cancel_order(id)
                .await
                .map_err(|e| warn!("Error canceling order {id:?}: {e}"));

            run_sync_orders.set(());
        }
    });

    create_effect(move |_| {
        market.get().map(move |market| {
            let market_outpoint = market_outpoint.get();

            order_cache.with(move |order_cache| {
                let mut v = vec![
                    (ContractOfOutcomeAmount::ZERO, SignedAmount::ZERO);
                    market.outcomes as usize
                ];

                for (_, order) in order_cache {
                    let order = order.get_untracked();

                    if order.market != market_outpoint {
                        continue;
                    }

                    let outcome_stat = v.get_mut(order.outcome as usize).unwrap();
                    outcome_stat.0 = outcome_stat.0 + order.contract_of_outcome_balance;
                    if order.side == Side::Sell {
                        outcome_stat.0 = outcome_stat.0 + order.quantity_waiting_for_match
                    }

                    outcome_stat.1 = outcome_stat.1
                        + order.bitcoin_acquired_from_order_matches
                        + SignedAmount::from(order.bitcoin_acquired_from_payout);
                }

                outcome_stats.set(v);
            })
        })
    });

    let average_price_per_fulfilled_string = |order_signal: RwSignal<Order>| {
        let order = order_signal.get();

        let mut all_price = order.bitcoin_acquired_from_order_matches.amount.msats as f64;
        if order.bitcoin_acquired_from_order_matches.is_negative() {
            all_price = -all_price;
        }
        if order.side == Side::Buy {
            all_price = -all_price;
        }

        let quantity = order.quantity_fulfilled.0 as f64;

        if quantity == 0f64 {
            "".to_owned()
        } else {
            format!("{:.2} msats", all_price / quantity)
        }
    };

    view! {
        <div class="flex flex-col border p-1">
            <div class="flex gap-6 p-1">
                <h2 class="font-bold">"Your Market Orders"</h2>
                <div>
                    <label class="pr-1 text-xs">"Only Active Orders"</label>
                    <input
                        type="checkbox"
                        on:input=move |ev| form_only_active_orders.set(event_target_checked(&ev))
                        prop:checked=move || form_only_active_orders.get()
                    />
                </div>
                <div>
                    <label class="pr-1 text-xs">"Only Selected Outcome Orders"</label>
                    <input
                        type="checkbox"
                        on:input=move |ev| form_only_selected_outcome_orders.set(event_target_checked(&ev))
                        prop:checked=move || form_only_selected_outcome_orders.get()
                    />
                </div>
            </div>
            <table class="overflow-auto">
                <thead>
                    <th></th>
                    <th class="border p-1 text-sm">"ID"</th>
                    <th class="border p-1 text-sm">"Created"</th>
                    <th class="border p-1 text-sm">"Outcome"</th>
                    <th class="border p-1 text-sm">"Side"</th>
                    <th class="border p-1 text-sm">"Price"</th>
                    <th class="border p-1 text-sm">"Remaining"</th>
                    <th class="border p-1 text-sm">"Fulfilled"</th>
                    <th class="border p-1 text-sm">"Avg. Price per Fulfilled"</th>
                </thead>
                <For
                    each=move || filtered_orders.get().into_iter().rev()
                    key=move |(id, _)| id.to_owned()
                    children=move |(id, order_signal)| {
                        let order = order_signal.get_untracked();

                        view! {
                            <tr>
                                <Show
                                    when=move || {order_signal.get().quantity_waiting_for_match > ContractOfOutcomeAmount::ZERO}
                                    fallback=move || view! {<td></td>}
                                >
                                    <td
                                        class="border p-1 text-sm hover:bg-red-500 cursor-pointer text-center"
                                        on:click=move |_| cancel_order_action.dispatch(id)
                                    >
                                        "X"
                                    </td>
                                </Show>
                                <td class="border p-1 text-sm">{id.0}</td>
                                <td class="border p-1 text-sm">{unix_timestamp_to_js_string(order.created_consensus_timestamp)}</td>
                                <td class="border p-1 text-sm">
                                    {
                                        move || {
                                            market.get().map(
                                                move |m| m.information.outcome_titles.get(order.outcome as usize).unwrap().to_owned()
                                            )
                                            .unwrap_or(order.outcome.to_string())
                                        }
                                    }
                                </td>
                                <td class="border p-1 text-sm">{format!("{:?}", order.side)}</td>
                                <td class="border p-1 text-sm">{order.price.msats}</td>
                                <td class="border p-1 text-sm">{move || order_signal.get().quantity_waiting_for_match.0}</td>
                                <td class="border p-1 text-sm">{move || order_signal.get().quantity_fulfilled.0}</td>
                                <td class="border p-1 text-sm">{move || average_price_per_fulfilled_string(order_signal)}</td>
                            </tr>
                        }
                    }
                />
            </table>
        </div>
    }
}
