use leptos::*;

use super::submit_form::SubmitForm;

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
        <div class="bg-yellow-100 border-l-4 border-yellow-500 text-yellow-700 p-4 mb-8" role="alert">
          <p class="font-bold">Note</p>
          <p> "To switch wallets after selecting one just reload the web app." </p>
        </div>
        <div class="bg-orange-100 border-l-4 border-orange-500 text-orange-700 p-4 mb-8" role="alert">
          <p class="font-bold">Warning</p>
          <p>
            "Webimint is a highly experimental Fedimint wallet, use at your own risk. It's currently compatible with the 0.2 release of Fedimint."
          </p>
        </div>
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
