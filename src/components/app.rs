use crate::components::{Footer, Joined, Logo, SubmitForm, WalletSelector};

use crate::client::ClientRpc;
use crate::context::provide_client_context;
use crate::utils::empty_view;
use anyhow::anyhow;
use leptos::*;
use leptos_meta::Title;

//
// App component
//
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let client = ClientRpc::new();
    provide_client_context(cx, client.clone());

    let res_client = client.clone();
    let wallets_resource = create_resource(
        cx,
        || (),
        move |()| {
            let client = res_client.clone();
            async move { client.list_wallets().await.ok() }
        },
    );

    let action_client = client.clone();
    let select_wallet_action = create_action(cx, move |wallet_name: &String| {
        let wallet_name = wallet_name.clone();
        let client = action_client.clone();
        async move { client.select_wallet(wallet_name).await.ok() }
    });

    let join_action = create_action(cx, move |invite: &String| {
        let invite = invite.clone();
        let client = client.clone();
        async move { client.join(invite).await }
    });

    let show_select_wallet = move || select_wallet_action.value().get().is_none();
    let show_join = move || {
        (select_wallet_action.value().get() == Some(Some(false)))
            && join_action
                .value()
                .with(|r| matches!(r, None | Some(Err(_))))
    };
    let show_join_error = move || join_action.value().with(|r| matches!(r, Some(Err(_))));
    let show_wallet = move || {
        let select_wallet = select_wallet_action.value().get();
        select_wallet.is_some()
            && (join_action
                .value()
                .with(|r| if let Some(Ok(_)) = r { true } else { false })
                || select_wallet == Some(Some(true)))
    };

    view! { cx,
      <Title text="Fedimint Web Client" />
      <div class="h-[100dvh]">
        <div class="mx-auto w-full h-full flex flex-col max-w-[600px] p-6">
          <header class="flex justify-center mb-20">
            <Logo class="bg-red border-1 border-blue"/>
          </header>
          <main class="w-full pb-24 flex-grow ">
            <Show
              when=show_select_wallet
                fallback=|_| empty_view()
              >
              {
                  move || {
                    if let Some(Some(wallets)) = wallets_resource.read(cx) {
                      view! { cx,
                        <WalletSelector
                          available=wallets
                          on_select=move |wallet_name| select_wallet_action.dispatch(wallet_name)
                        />
                      }.into_view(cx)
                    } else {
                      empty_view().into_view(cx)
                    }
                  }
              }

            </Show>
            <Show
              when=show_join
                fallback=|_| empty_view()
              >
              <h1 class="font-heading text-gray-900 text-4xl font-semibold mb-6">"Join a Federation"</h1>
              <SubmitForm
                description="Enter invite code (i.e. fed11jpr3lgm8t…) to join a Federation".into()
                on_submit=move |value| join_action.dispatch(value)
                placeholder="invite code".into()
                submit_label="Join".into()
                loading=join_action.pending()
              />
            </Show>

            <Show
              when=show_join_error
              fallback=|_| empty_view()
            >
              {move || view!{ cx, <div class="text-body text-md mt-4"><span class="text-red-500">{
                format!("✗ Failed to join federation: {:?}", join_action.value().with(|r| {
                  match r {
                    Some(Err(e)) =>anyhow!("{:?}", e),
                    _ => anyhow!("")
                  }
                }))
              }</span></div>}}
            </Show>

            <Show
              when=show_wallet
              fallback=|_| empty_view()
            >
              <Joined />
            </Show>
          </main>
          <Footer class="w-full py-2" />
        </div>
      </div>
    }
}
