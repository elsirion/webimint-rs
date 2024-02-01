use std::collections::BTreeMap;
use std::str::FromStr;

use fedimint_core::Amount;
use fedimint_prediction_markets_common::{
    MarketInformation, Outcome, UnixTimestamp, Weight, WeightRequiredForPayout,
};
use leptos::*;
use secp256k1::PublicKey;
use tracing::info;

use crate::{context::ClientContext, prediction_markets::helpers::js_string_to_unix_timestamp};

#[component]
pub fn NewMarket(cx: Scope) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    let form_contract_price = create_rw_signal(cx, "1000".to_owned());
    let form_outcomes = create_rw_signal(cx, "2".to_owned());
    let form_payout_control_weights: RwSignal<Vec<(RwSignal<String>, RwSignal<String>)>> =
        create_rw_signal(cx, vec![]);
    let form_weight_required_for_payout = create_rw_signal(cx, "1".to_owned());
    let form_payout_controls_fee_per_contract = create_rw_signal(cx, "0".to_owned());

    let form_title = create_rw_signal(cx, "".to_owned());
    let form_description = create_rw_signal(cx, "".to_owned());
    let form_outcome_titles: RwSignal<Vec<RwSignal<String>>> = create_rw_signal(cx, vec![]);
    let form_expected_payout_timestamp = create_rw_signal(cx, "".to_owned());

    let new_market_action = create_action(cx, move |_| async move {
        let contract_price = form_contract_price
            .get_untracked()
            .parse::<Amount>()
            .map_err(|e| format!("error parsing contract price: {}", e))?;
        let outcomes = form_outcomes
            .get_untracked()
            .parse::<Outcome>()
            .map_err(|e| format!("error parsing number of outcomes: {}", e))?;
        let payout_control_weights = {
            let mut b = BTreeMap::new();
            for (payout_control, weight) in form_payout_control_weights.get_untracked().into_iter() {
                let payout_control = match client
                    .get_value()
                    .get_name_to_payout_control(payout_control.get_untracked())
                    .await
                { 
                    Ok(Some(pk)) => pk,
                    Ok(None) => PublicKey::from_str(&payout_control.get())
                        .map_err(|e| format!("error parsing payout control: {}", e))?,
                    Err(e) => return Err(e.to_string()),
                };
                let weight = weight
                    .get_untracked()
                    .parse::<Weight>()
                    .map_err(|e| format!("{}", e))?;

                if let Some(_) = b.insert(payout_control, weight) {
                    return Err("duplicate payout control keys".to_owned());
                }
            }
            b
        };
        let weight_required_for_payout = form_weight_required_for_payout
            .get_untracked()
            .parse::<WeightRequiredForPayout>()
            .map_err(|e| format!("error parsing weight required for payout: {}", e))?;
        let payout_controls_fee_per_contract = form_payout_controls_fee_per_contract
            .get_untracked()
            .parse::<Amount>()
            .map_err(|e| format!("error parsing payout controls fee per contract: {}", e))?;

        let market_information = MarketInformation {
            title: form_title.get_untracked(),
            description: form_description.get_untracked(),
            outcome_titles: form_outcome_titles
                .get_untracked()
                .into_iter()
                .map(|t| t.get_untracked())
                .collect::<Vec<_>>(),
            expected_payout_timestamp: js_string_to_unix_timestamp(form_expected_payout_timestamp.get_untracked()),
        };

        let r = Ok(client
            .get_value()
            .new_market(
                contract_price,
                outcomes,
                payout_control_weights,
                weight_required_for_payout,
                payout_controls_fee_per_contract,
                market_information,
            )
            .await
            .map_err(|e| format!("Issue creating market: {:?}", e))?);

        if let Ok(market_outpoint) = r.clone() {
            client
                .get_value()
                .save_market(market_outpoint)
                .await
                .map_err(|e| format!("Issue saving new market: {:?}", e))?;

            // to cache market
            _ = client.get_value().get_market(market_outpoint, false).await;
        }

        r
    });

    create_effect(cx, move |_| {
        let Ok(outcomes) = form_outcomes.get().parse::<Outcome>() else {
            return;
        };

        form_outcome_titles.update(|v| {
            v.clear();
            for _ in 0..outcomes {
                v.push(create_rw_signal(cx, "".to_owned()));
            }
        })
    });

    view! {cx,
        <div class="block">
            <label>"Title"</label>
            <br />
            <input type="text"
                on:input=move |ev| form_title.set(event_target_value(&ev))
                prop:value=move || form_title.get()
            />
            <br />

            <label>"Description"</label>
            <br />
            <input type="text"
                on:input=move |ev| form_description.set(event_target_value(&ev))
                prop:value=move || form_description.get()
            />
            <br />

            <label>"Number of outcomes"</label>
            <br />
            <input type="number"
                on:input=move |ev| form_outcomes.set(event_target_value(&ev))
                prop:value=move || form_outcomes.get()
            />
            <br />

            <label>"Outcome titles"</label>
            <br />
            {move || {
                form_outcome_titles
                    .get()
                    .into_iter()
                    .map(|outcome_title| {
                        view! {
                            cx,
                            <input type="text"
                                    on:input=move |ev| outcome_title.set(event_target_value(&ev))
                                    prop:value=move || outcome_title.get()
                            />
                        }
                    })
                    .collect_view(cx)
            }}
            <br />

            <label>"Contract price"</label>
            <br />
            <input type="number"
                on:input=move |ev| form_contract_price.set(event_target_value(&ev))
                prop:value=move || form_contract_price.get()
            />
            <br />

            <label>"Payout control and their weights"</label>
            <br />
            {move || {
                form_payout_control_weights
                    .get()
                    .into_iter()
                    .enumerate()
                    .map(|(i,(payout_control, weight))| {
                        view! {
                            cx,
                            <div class="flex">
                                <button on:click=move |_| {
                                    form_payout_control_weights.update(|v| {
                                        v.remove(i);
                                    })
                                }>
                                    "X"
                                </button>
                                <input type="text"
                                    on:input=move |ev| payout_control.set(event_target_value(&ev))
                                    prop:value=move || payout_control.get()
                                />
                                <input type="text"
                                    on:input=move |ev| weight.set(event_target_value(&ev))
                                    prop:value=move || weight.get()
                                />
                            </div>
                        }
                    })
                    .collect_view(cx)
                }
            }
            <br />
            <button
                class="font-bold text-lg"
                on:click=move |_| {
                    form_payout_control_weights.update(|v| {
                        v.push(
                            (create_rw_signal(cx, "".to_owned()), create_rw_signal(cx, "".to_owned()))
                        );
                    });
                }
            >
                +
            </button>
            <br />

            <label>"Cumulative agreeing weight required for payout"</label>
            <br />
            <input type="number"
                on:input=move |ev| form_weight_required_for_payout.set(event_target_value(&ev))
                prop:value=move || form_weight_required_for_payout.get()
            />
            <br />

            <label>"Fee per contract payed to payout controls on payout"</label>
            <br />
            <input type="number"
                on:input=move |ev| form_payout_controls_fee_per_contract.set(event_target_value(&ev))
                prop:value=move || form_payout_controls_fee_per_contract.get()
            />
            <br />

            <label>"Expected payout timestamp"</label>
            <br />
            <input type="datetime-local"
                on:input=move |ev| form_expected_payout_timestamp.set(event_target_value(&ev))                
                prop:value=move || form_expected_payout_timestamp.get()
            />
            <br />

            <button on:click=move |_| new_market_action.dispatch(())>"Create market"</button>

            <p>
                {move || new_market_action
                    .value()
                    .get()
                    .map(|r| {
                        match r {
                            Ok(outpoint) => format!("Market created successfully: {}. Market has been saved.", outpoint.txid),
                            Err(e) => e,
                        }
                    })
                }
            </p>
        </div>
    }
}
