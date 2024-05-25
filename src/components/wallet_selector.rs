use leptos::*;

use super::{NoteBlock, WarningBlock};
use crate::utils::empty_view;

#[component]
pub fn WalletSelector<F>(available: Vec<String>, on_select: F) -> impl IntoView
where
    F: Fn(String) + 'static + Copy,
{
    let (_, set_loading) = create_signal(false);
    let select = move |name: String| {
        set_loading.set(true);
        on_select(name);
    };

    let available_clone = available.clone();
    let wallets_available = move || available_clone.len() > 0;

    const MAX_NAME_LEN: usize = 20;

    let available_list = available
        .into_iter()
        .map(|name| {
            let select_name = name.clone();
            let abbreviated_name = if name.len() > MAX_NAME_LEN {
                let part_len = MAX_NAME_LEN / 2 - 1;
                format!("{}...{}", &name[..part_len], &name[name.len() - part_len..])
            } else {
                name.clone()
            };
            view! {
                  <button
                    class="px-4 w-4/5 py-2 bg-blue-400 text-white font-bold rounded hover:bg-blue-700 focus:outline-none focus:shadow-outline min-w-[200px]"
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
          <h1 class="font-heading text-gray-900 font-semibold mb-6">"Select a wallet:"</h1>
        </Show>
        <div class="flex flex-col items-center mb-6 gap-y-4">
        { available_list }
        </div>
    }
}
