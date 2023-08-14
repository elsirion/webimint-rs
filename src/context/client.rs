use fedimint_core::task::sleep;
use futures::stream::StreamExt;

use crate::client::ClientRpc;
use fedimint_core::Amount;
use leptos::*;

//
// Client Context
//
#[derive(Clone)]
pub(crate) struct ClientContext {
    pub client: StoredValue<ClientRpc>,
    pub balance: ReadSignal<Amount>,
}

pub fn provide_client_context(cx: Scope, client: ClientRpc) {
    let client = store_value(cx, client);

    let (balance, set_balance) = create_signal(cx, Amount::ZERO);

    wasm_bindgen_futures::spawn_local(async move {
        let mut balance_stream = loop {
            let balance_res = client.get_value().subscribe_balance().await;

            match balance_res {
                Ok(balance) => {
                    log!("balance_stream OK");
                    break balance;
                }
                Err(e) => {
                    warn!("client could not get balance: {e:?}");
                    sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        };

        while let Some(amount) = balance_stream.next().await {
            log!("balance stream update: {}", amount.msats);
            set_balance.set(amount);
        }
    });

    let context = ClientContext { client, balance };

    provide_context(cx, context);
}
