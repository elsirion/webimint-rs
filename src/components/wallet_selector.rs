use leptos::*;

use super::{NoteBlock, SubmitForm, WarningBlock};

#[component]
pub fn WalletSelector<F>(available: Vec<String>, on_select: F) -> impl IntoView
where
    F: Fn(String) + 'static + Copy,
{
    let (loading, set_loading) = create_signal(false);
    let select = move |name: String| {
        set_loading.set(true);
        on_select(name);
    };

    let available_list = available
        .into_iter()
        .map(|name| {
            let select_name = name.clone();
            view! {
                <li>
                  <a
                    class="text-xl underline"
                    href="#"
                    on:click=move |ev| {
                      ev.prevent_default();
                      select(select_name.clone());
                    }
                  >
                    {name}
                  </a>
                </li>
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

        <h1 class="font-heading text-gray-900 text-4xl font-semibold mb-6">"Select a wallet:"</h1>
        <ul class="mb-6 list-disc">
          { available_list }
        </ul>

      <SubmitForm
        description="… or create a new one".into()
        on_submit=select
        placeholder="Wallet Name".into()
        submit_label="Create".into()
        loading=loading
      />
    }
}
