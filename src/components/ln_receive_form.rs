use leptos::*;

use super::submit_button::SubmitButton;
use crate::utils::empty_view;

#[component]
pub fn LnReceiveForm<F>(on_submit: F) -> impl IntoView
where
    F: Fn(u64, String) + 'static + Copy,
{
    let (amount, set_amount) = create_signal("".to_string());
    let (description, set_description) = create_signal("".to_string());
    let (error, set_error) = create_signal(None);

    let on_submit = move || {
        let amount_msat = match amount.get().parse::<u64>() {
            Ok(amt) => {
                set_error.set(None);
                amt
            }
            Err(e) => {
                set_error.set(Some(format!("Invalid amount: {e}")));
                return;
            }
        };

        on_submit(amount_msat, description.get());
    };

    view! {
        <form
            on:submit=move |ev| {
                ev.prevent_default();
                on_submit()
            }
        >
            <input
                type="number"
                placeholder="Amount msat"
                class="my-4 w-full text-xl font-body text-gray-600 border-gray-400 placeholder:text-gray-400 ring-0 focus:border-blue-400 focus:ring-0"
                on:input=move |ev| {
                    set_amount.set(event_target_value(&ev));
                }
                prop:value=move || amount.get()
            />
            {move || {
                if let Some(error) = error.get() {
                    view!{
                        <div class="bg-orange-100 border-l-4 border-orange-500 text-orange-700 p-4" role="alert">
                          <p class="font-bold">Error</p>
                          <p>{error}</p>
                        </div>
                    }.into_view()
                } else {
                    empty_view().into_view()
                }
            }}
            <input
                type="text"
                placeholder="Description"
                class="my-4 w-full text-xl font-body text-gray-600 border-gray-400 placeholder:text-gray-400 ring-0 focus:border-blue-400 focus:ring-0"
                on:input=move |ev| {
                    set_description.set(event_target_value(&ev));
                }
                prop:value=move || description.get()
            />
            <SubmitButton
                class="my-4 w-full"
                loading=create_signal(false).0
                disabled=Signal::derive(move || false)
                on_click=move |_| {}
            >
                "Generate Invoice"
            </SubmitButton>
        </form>
    }
}
