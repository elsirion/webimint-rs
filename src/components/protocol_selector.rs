use leptos::*;

use crate::components::SegmentedButton;

pub enum Protocol {
    OnChain,
    Lightning,
    ECash,
}

impl Protocol {
    fn from_idx(idx: usize) -> Protocol {
        match idx {
            0 => Protocol::OnChain,
            1 => Protocol::Lightning,
            2 => Protocol::ECash,
            _ => panic!("Out of bounds"),
        }
    }

    fn into_idx(self) -> usize {
        match self {
            Protocol::OnChain => 0,
            Protocol::Lightning => 1,
            Protocol::ECash => 2,
        }
    }
}
#[component]
pub fn ProtocolSelector<F>(
    #[prop(default = Protocol::Lightning)] active_protocol: Protocol,
    on_change: F,
) -> impl IntoView
where
    F: Fn(Protocol) + 'static + Copy,
{
    view! {
        <SegmentedButton
          active_idx=active_protocol.into_idx()
          segments=vec!["On-Chain".into(), "Lightning".into(), "E-Cash".into()]
          on_change=move |idx| on_change(Protocol::from_idx(idx))
        />
    }
}
