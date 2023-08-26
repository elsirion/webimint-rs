use crate::components::ln_receive_form::LnReceiveForm;
use crate::context::ClientContext;
use crate::utils::empty_view;
use leptos::*;

//
// Receive LN component
//
#[component]
pub fn ReceiveLn(cx: Scope) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    let client = client.clone();
    let submit_action = create_action(cx, move |(amount_msat, description): &(u64, String)| {
        let description = description.clone();
        let amount_msat = *amount_msat;
        async move {
            client
                .get_value()
                .ln_receive(amount_msat, description)
                .await
        }
    });

    view! { cx,
        <LnReceiveForm
            on_submit=move |amount_msat, description| {
                submit_action.dispatch((amount_msat, description));
            }
        />
        { move || {
            if let Some(invoice) = submit_action.value().get() {
                view!(cx,
                    <div class="w-full my-4 p-4 bg-slate-100">
                        <span class="break-all" style="font-family: mono">{invoice}</span>
                    </div>
                ).into_view(cx)
            } else {
                empty_view().into_view(cx)
            }
        }}
    }
}
