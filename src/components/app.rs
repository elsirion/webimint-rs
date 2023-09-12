use crate::components::{Failure, Footer, Joined, Logo, SubmitForm};

use crate::client::ClientRpc;
use crate::context::provide_client_context;
use crate::utils::{empty_view, local_storage};
use leptos::*;
use leptos_meta::Title;

//
// App component
//
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Indicates whether the user has successfully joined a federation
    let (success, set_success) = create_signal(cx, false);

    let join_action = create_action(cx, move |invite_code: &String| {
        let invite_code_cloned = invite_code.clone();
        async move {
            let client = ClientRpc::new();
            let result = client.join(invite_code_cloned.clone()).await;
            let ok_result = result.ok();

            set_success.set(ok_result.is_some());

            // If the client joins the federation, save the invite code to local storage
            if let Some(_) = &ok_result {
                local_storage()
                    .set_item("invite_code", invite_code_cloned.as_str())
                    .ok()?;
            }

            ok_result.map(|_| client)
        }
    });

    // Whether the user has attempted to join a federation.
    // Does not indicate whether the user has _successfully_ joined a federation.
    let attempted = move || join_action.value().get().is_some();

    view! { cx,
      <Title text="Fedimint Web Client" />
      <div class="h-[100dvh]">
        <div class="mx-auto w-full h-full flex flex-col max-w-[600px] p-6">
          <header class="flex justify-center mb-20">
            <Logo />
          </header>
          <main class="w-full pb-24 flex-grow ">
            <Show
              when=move || !attempted()
              fallback=move |_| empty_view()
            >
              <h1 class="font-heading text-gray-900 text-4xl font-semibold mb-6">"Join a Federation"</h1>
              <div class="bg-orange-100 border-l-4 border-orange-500 text-orange-700 p-4 mb-8" role="alert">
                <p class="font-bold">Warning</p>
                <p> "This demo lacks persistent storage, reloading the bowser tab will reset all state and burn all deposited funds." </p>
                <p
                  class="mt-2"
                >
                  <a
                    href="https://github.com/elsirion/webimint-rs/issues/31"
                    class="underline text-orange-600 hover:text-orange-800"
                    target="_blank"
                  >
                    "Want to fix this? Take a look at issue #31 😃"
                  </a>
                </p>
              </div>
              <SubmitForm
                description="Enter invite code (i.e. fed11jpr3lgm8t…) to join a Federation".into()
                on_submit=move |value| join_action.dispatch(value)
                placeholder="invite code".into()
                submit_label="Join".into()
                loading=join_action.pending()
                intro_screen=true
              />
            </Show>

            <Show
              when=move || !success.get() && attempted()
              fallback=move |_cx| empty_view()
            >
              <Failure
                on_click=|_| {
                  window().location().reload().ok();
                }
              />
            </Show>

            <Suspense
              fallback=move || view!{ cx, "Loading..."}
            >
            <ErrorBoundary fallback=|cx, error| view!{ cx, <p>{format!("Failed to create client: {:?}", error.get())}</p>}>
              { move || { 
                join_action.value().get().flatten().map(|c| {
                  // Create app context to provide ClientRpc
                  // as soon as it's available
                  provide_client_context(cx, c);

                  view! { cx,
                    <Joined />
                  }
                })
              }}
            </ErrorBoundary>
            </Suspense>
          </main>
          <Footer class="w-full py-2" />
        </div>
      </div>
    }
}
