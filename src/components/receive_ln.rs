use crate::components::ln_receive_form::LnReceiveForm;
use crate::components::loader_icon::LoaderIcon;
use crate::components::qrcode::QrCode;
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
        <div class="w-full my-4 p-4 bg-slate-100 flex justify-center">
            <Show
                when=move || !submit_action.pending().get()
                fallback=|cx| view!{cx, <LoaderIcon />}
            >
            { move || {
                match submit_action.value().get() {
                    Some(Ok((invoice, await_paid))) => {
                        let qr_invoice_upper = format!("lightning:{invoice}").to_ascii_uppercase();

                        let paid_resource = create_resource(cx, || (), move |()| {
                            let mut await_paid = await_paid.clone();
                            async move {
                                let _ = await_paid.wait_for(|paid| *paid).await;
                            }
                        });

                        view!{ cx,
                            <div class="w-full">
                                { move || {
                                    // TODO: fix
                                    // Needed to suppress loading screen
                                    if paid_resource.loading().get() {
                                        return empty_view().into_view(cx);
                                    }

                                    match paid_resource.read(cx) {
                                        Some(()) => view! {cx,
                                            <div class="bg-green-100 border-l-4 border-green-500 text-green-700 p-4 w-full mb-8" role="alert">
                                                <p class="font-bold">Success</p>
                                                <p>The invoice has been paid!</p>
                                            </div>
                                        }.into_view(cx),
                                        None => empty_view().into_view(cx),
                                    }
                                }}
                                <span class="break-all" style="font-family: mono">{&invoice}</span>
                                <QrCode
                                    data={Signal::derive(cx, move || qr_invoice_upper.clone())}
                                    class="w-full mt-8"
                                />
                            </div>
                        }.into_view(cx)
                    }
                    Some(Err(e)) => {
                        view!{ cx,
                            <div class="bg-orange-100 border-l-4 border-orange-500 text-orange-700 p-4 w-full" role="alert">
                                <p class="font-bold">Error</p>
                                <p>{e.to_string()}</p>
                            </div>
                        }.into_view(cx)
                    }
                    None => {
                        empty_view().into_view(cx)
                    }
                }
            }}
            </Show>
        </div>
    }
}
