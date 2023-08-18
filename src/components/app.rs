use crate::components::{Footer, Joined, Logo, SubmitForm};

use crate::client::ClientRpc;
use crate::context::provide_client_context;
use crate::utils::empty_view;
use leptos::*;
use leptos_meta::Title;

//
// App component
//
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let join_action = create_action(cx, |invoice: &String| {
        let invoice = invoice.clone();
        async move {
            let client = ClientRpc::new();
            let result = client.join(invoice).await;
            result.ok().map(|_| client)
        }
    });

    let joined = move || join_action.value().get().is_some();

    view! { cx,
      <Title text="Fedimint Web Client" />
      <div class="h-[100dvh]">
        <div class="mx-auto w-full h-full flex flex-col max-w-[600px] p-6">
          <header class="flex justify-center mb-20">
            <Logo />
          </header>
          <main class="w-full pb-24 flex-grow ">
            <Show
              when=move || !joined()
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
