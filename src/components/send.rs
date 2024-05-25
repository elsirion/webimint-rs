use leptos::*;

use crate::components::{Protocol, ProtocolSelector, SendEcash, SendLn};

#[component]
pub fn Send() -> impl IntoView {
    const DEFAULT_PROTOCOL: Protocol = Protocol::Lightning;
    let (active_protocol, set_active_protocol) = create_signal(DEFAULT_PROTOCOL);

    let active_protocol_view = move || match active_protocol.get() {
        Protocol::OnChain => view! {
            "TODO"
        }
        .into_view(),
        Protocol::Lightning => view! {
            <SendLn />
        }
        .into_view(),
        Protocol::ECash => view! {
            <SendEcash />
        }
        .into_view(),
    };

    view! {
        <ProtocolSelector
          active_protocol=DEFAULT_PROTOCOL
          on_change=move |protocol| set_active_protocol.set(protocol)
        />
        {active_protocol_view}
    }
}
