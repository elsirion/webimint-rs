use leptos::*;

use crate::components::SubmitForm;
use crate::utils::empty_view;

#[component]
pub fn CreateWallet<F>(on_select: F) -> impl IntoView
where
    F: Fn(String) + 'static + Copy,
{
    let (loading, set_loading) = create_signal(false);
    let (show_create_wallet_form, set_show_create_wallet_form) = create_signal(false);
    let select = move |name: String| {
        set_loading.set(true);
        on_select(name);
    };

    view! {
    <div class="flex justify-center">
      <Show when=move || !show_create_wallet_form.get() fallback=|| empty_view() >
        <button
          class="mt-4 px-4 py-2 bg-blue-500 text-white font-bold rounded hover:bg-blue-700 focus:outline-none focus:shadow-outline min-w-[200px]"
          on:click=move |_| {
            set_show_create_wallet_form.set(true);
          }
        >
          "Create a new wallet"
        </button>
      </Show>
      <Show when=move || show_create_wallet_form.get() fallback=|| empty_view() >
        <SubmitForm
          description="Enter a name for the new wallet".into()
          on_submit=move |name| {
            set_show_create_wallet_form.set(false);
            select(name);
          }
          placeholder="Wallet Name".into()
          submit_label="Create".into()
          loading=loading
        />
      </Show>
    </div>
        }
}
