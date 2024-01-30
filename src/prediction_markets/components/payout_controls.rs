use leptos::*;

use crate::context::ClientContext;
use crate::prediction_markets::components::ClientPayoutControl;


#[component]
pub fn PayoutControls(cx: Scope) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    view!{
        cx,
        <ClientPayoutControl />
    }
}