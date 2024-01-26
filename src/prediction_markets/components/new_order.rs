use fedimint_core::{Amount, OutPoint};
use fedimint_prediction_markets_common::{ContractOfOutcomeAmount, OrderIdClientSide, Outcome, Side};
use leptos::*;

use crate::context::ClientContext;

#[component]
pub fn NewOrder(
    cx: Scope,
    market_outpoint: Memo<OutPoint>,
    outcome: RwSignal<Outcome>,
) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    let form_side = create_rw_signal(cx, "".to_owned());
    let form_price = create_rw_signal(cx, "".to_owned());
    let form_quantity = create_rw_signal(cx, "".to_owned());

    let new_order_action: Action<(), Result<OrderIdClientSide, String>> = create_action(cx, move |()| async move {
        let market = market_outpoint.get();
        let outcome = outcome.get();
        let side: Side = form_side
            .get()
            .as_str()
            .try_into()
            .map_err(|e| format!("Error getting side: {}", e))?;
        let price = Amount::from_msats(
            form_price
                .get()
                .parse::<u64>()
                .map_err(|e| format!("Error parsing price: {}", e))?,
        );
        let quantity = ContractOfOutcomeAmount(
            form_quantity
                .get()
                .parse()
                .map_err(|e| format!("Error parsing quantity: {}", e))?,
        );

        Ok(client
            .get_value()
            .new_order(market, outcome, side, price, quantity)
            .await
            .map_err(|e| format!("Error creating new order: {e:?}"))?)
    });

    view! {
        cx,
        <div>
            <h3>"Create new order"</h3>
            <br />

            <label>"Side"</label>
            <br />
            <button 
                class={move || format!("p-2 border-2 {}", if form_side.get() == "buy" {"bg-slate-200"} else {""})} 
                on:click=move |_| form_side.set("buy".into())
            >
                "Buy"
            </button>
            <button 
                class={move || format!("p-2 border-2 {}", if form_side.get() == "sell" {"bg-slate-200"} else {""})} 
                on:click=move |_| form_side.set("sell".into())
            >
                "Sell"
            </button>
            <br />

            <label>"Price"</label>
            <br />
            <input type="number"
                on:input=move |ev| form_price.set(event_target_value(&ev)) 
                prop:value=move || form_price.get()    
            />
            <br />

            <label>"Quantity"</label>
            <br />
            <input type="number"
                on:input=move |ev| form_quantity.set(event_target_value(&ev)) 
                prop:value=move || form_quantity.get()    
            />
            <br />

            <button on:click=move |_| new_order_action.dispatch(())>"Create Order"</button>
            <br />

            <span>
                {move || new_order_action.value().get().map(|r| {
                    match r {
                        Ok(id) => format!("New order created with id: {id:?}"),
                        Err(e) => format!("Error creating new order: {e}")
                    }
                })}
            </span>
        </div>
    }
}
