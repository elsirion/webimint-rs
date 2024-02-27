use fedimint_core::Amount;
use fedimint_mint_client::OOBNotes;
use leptos::*;

use super::{CopyableText, ErrorBlock, QrCode, SubmitButton, SuccessBlock, WarningBlock};
use crate::client::RpcError;
use crate::context::ClientContext;

//
// Send Ecash component
//
#[component]
pub fn SendEcash() -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>();

    let (amount, set_amount) = create_signal("".to_owned());
    let (error, set_error) = create_signal(None);

    #[derive(Clone)]
    struct ActionReturn {
        requested_amount: Amount,
        result: anyhow::Result<OOBNotes, RpcError>,
    }
    let client = client.clone();
    let submit_action = create_action(move |amount: &Amount| {
        let amount = amount.to_owned();
        async move {
            ActionReturn {
                requested_amount: amount,
                result: client.get_value().ecash_send(amount).await,
            }
        }
    });

    let parse_and_submit = move || {
        let amount = match amount.get().parse::<Amount>() {
            Ok(a) => a,
            Err(e) => {
                set_error.set(Some(format!("Invalid amount: {e}")));
                return;
            }
        };

        set_error.set(None);

        submit_action.dispatch(amount);
    };

    view! {
        <div class="flex flex-col gap-4">
            <input
                type="number"
                placeholder="Amount msat"
                class="w-full text-xl font-body text-gray-600 border-gray-400 placeholder:text-gray-400 ring-0 focus:border-blue-400 focus:ring-0"
                on:input=move |ev| {
                    set_amount.set(event_target_value(&ev));
                }
                prop:value=move || amount.get()
            />

            <SubmitButton
                loading=submit_action.pending()
                disabled=submit_action.pending().into()
                on_click=move |_| parse_and_submit()
                class="w-full"
            >
                Spend
            </SubmitButton>

            {move || {
                error.get().map(|e| {
                    view! {
                        <ErrorBlock>
                            {e}
                        </ErrorBlock>
                    }
                })
            }}

            {move || {
                submit_action.value().get().map(|r| {
                    r.result.err().map(|err| {
                        view! {
                            <ErrorBlock>
                                {format!("{:?}", err)}
                            </ErrorBlock>
                        }
                    })
                })
            }}

            {move || {
                submit_action.value().get().map(|r| {
                    r.result.ok().map(move |notes| {
                        let total = notes.total_amount();
                        let notes_string_signal = Signal::derive(move || notes.to_string());
                        view! {
                            <SuccessBlock>
                                {format!("Notes representing {} shown below.", total)}
                            </SuccessBlock>
                            <Show when=move || total != r.requested_amount>
                                <WarningBlock>
                                    {format!("The notes represent {} more than the amount you requested.", total-r.requested_amount)}
                                </WarningBlock>
                            </Show>
                            <CopyableText
                                text=notes_string_signal
                                rows=10
                            />
                            <QrCode
                                data=notes_string_signal
                            />
                        }
                    })
                })
            }}
        </div>
    }
}
