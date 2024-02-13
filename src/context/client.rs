use leptos::*;

use crate::client::ClientRpc;

//
// Client Context
//
#[derive(Clone)]
pub(crate) struct ClientContext {
    pub client: StoredValue<ClientRpc>,
}

pub fn provide_client_context(client: ClientRpc) {
    let client = store_value(client);

    let context = ClientContext { client };

    provide_context(context);
}
