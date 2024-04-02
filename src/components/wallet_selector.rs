use leptos::*;

use super::{NoteBlock, SubmitForm, WarningBlock};
use crate::utils::empty_view;

#[component]
pub fn WalletSelector<F>(available: Vec<String>, on_select: F) -> impl IntoView
where
    F: Fn(String) + 'static + Copy,
{
    let (loading, set_loading) = create_signal(false);
    let (show_submit_form, set_show_submit_form) = create_signal(false);
    let select = move |name: String| {
        set_loading.set(true);
        on_select(name);
    };

    let available_clone = available.clone();
    let wallets_available = move || available_clone.len() > 0;

    let available_list = available
        .into_iter()
        .map(|name| {
            let select_name = name.clone();
            let abbreviated_name = if name.len() > 10 {
                let part_len = 10 / 2 - 1;
                format!("{}...{}", &name[..part_len], &name[name.len() - part_len..])
            } else {
                name.clone()
            };
            view! {
                  <button
                    class="px-4 py-2 bg-blue-400 text-white font-bold text-xl rounded hover:bg-blue-700 focus:outline-none focus:shadow-outline min-w-[200px]"
                    on:click=move |ev| {
                      ev.prevent_default();
                      select(select_name.clone());
                    }
                  >
                    {abbreviated_name}
                  </button>
            }
        })
        .collect::<Vec<_>>();

    view! {
      <NoteBlock class="mb-8">
          "To switch wallets after selecting one just reload the web app."
        </NoteBlock>
        <WarningBlock class="mb-8">
          "Webimint is a highly experimental Fedimint wallet, use at your own risk.
          It's currently compatible with the 0.2 release of Fedimint."
        </WarningBlock>

        <Show when=wallets_available fallback=|| empty_view() >
          <h1 class="font-heading text-gray-900 text-xl md:text-2xl font-semibold mb-6">"Select a wallet:"</h1>
        </Show>
        <div class="flex flex-col items-center mb-6 gap-y-4">
        { available_list }
        <Show when=move || !show_submit_form.get() fallback=|| { view! { <div></div> } } >

          <button
            class="mt-4 px-4 py-2 bg-blue-500 text-white font-bold text-xl rounded hover:bg-blue-700 focus:outline-none focus:shadow-outline min-w-[200px]"
            on:click=move |_| {
              set_show_submit_form.set(true);
            }
          >
            "Create a new wallet"
          </button>
        </Show>
        <Show when=move || show_submit_form.get() fallback=|| empty_view() >
          <SubmitForm
            description="Enter a name for the new wallet".into()
            on_submit=move |name| {
              set_show_submit_form.set(false);
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
