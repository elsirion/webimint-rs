use crate::client::ClientRpc;
use leptos::*;

//
// Client Context
//
#[derive(Clone)]
pub(crate) struct ClientContext {
    pub client: StoredValue<ClientRpc>,
}

pub fn provide_client_context(cx: Scope, client: ClientRpc) {
    let client = store_value(cx, client);

    let context = ClientContext { client };

    provide_context(cx, context);
}
