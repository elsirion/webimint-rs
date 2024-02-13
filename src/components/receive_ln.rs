use leptos::*;

use crate::components::ln_receive_form::LnReceiveForm;
use crate::components::loader_icon::LoaderIcon;
use crate::components::qrcode::QrCode;
use crate::context::ClientContext;
use crate::utils::empty_view;

//
// Receive LN component
//
#[component]
pub fn ReceiveLn() -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>();

    let client = client.clone();
    let submit_action = create_action(move |(amount_msat, description): &(u64, String)| {
        let description = description.clone();
        let amount_msat = *amount_msat;
        async move {
            client
                .get_value()
                .ln_receive(amount_msat, description)
                .await
        }
    });

    view! {
        <LnReceiveForm
            on_submit=move |amount_msat, description| {
                submit_action.dispatch((amount_msat, description));
            }
        />
        <div class="w-full my-4 p-4 bg-slate-100 flex justify-center">
            <Show
                when=move || !submit_action.pending().get()
                fallback=|| view!{<LoaderIcon />}
            >
            { move || {
                match submit_action.value().get() {
                    Some(Ok((invoice, await_paid))) => {
                        let qr_invoice_upper = format!("lightning:{invoice}").to_ascii_uppercase();

                        let paid_resource = create_resource(|| (), move |()| {
                            let mut await_paid = await_paid.clone();
                            async move {
                                let _ = await_paid.wait_for(|paid| *paid).await;
                            }
                        });

                        view!{
                            <div class="w-full">
                                { move || {
                                    // TODO: fix
                                    // Needed to suppress loading screen
                                    if paid_resource.loading().get() {
                                        return empty_view().into_view();
                                    }

                                    match paid_resource.get() {
                                        Some(()) => view! {
                                            <div class="bg-green-100 border-l-4 border-green-500 text-green-700 p-4 w-full mb-8" role="alert">
                                                <p class="font-bold">Success</p>
                                                <p>The invoice has been paid!</p>
                                            </div>
                                        }.into_view(),
                                        None => empty_view().into_view(),
                                    }
                                }}
                                <span class="break-all" style="font-family: mono">{&invoice}</span>
                                <QrCode
                                    data={Signal::derive(move || qr_invoice_upper.clone())}
                                    class="w-full mt-8"
                                />
                            </div>
                        }.into_view()
                    }
                    Some(Err(e)) => {
                        view!{
                            <div class="bg-orange-100 border-l-4 border-orange-500 text-orange-700 p-4 w-full" role="alert">
                                <p class="font-bold">Error</p>
                                <p>{e.to_string()}</p>
                            </div>
                        }.into_view()
                    }
                    None => {
                        empty_view().into_view()
                    }
                }
            }}
            </Show>
        </div>
    }
}
