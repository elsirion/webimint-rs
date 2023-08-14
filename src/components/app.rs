use crate::components::{Joined, Logo};

use crate::client::ClientRpc;
use crate::context::provide_client_context;
use crate::utils::empty_view;
use leptos::html::Textarea;
use leptos::*;

//
// App component
//
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let join_action = create_action(cx, |invoice: &String| {
        let invoice = invoice.clone();
        async move {
            let client = ClientRpc::new();
            _ = client.join(invoice).await;
            client
        }
    });

    let invite_code_element: NodeRef<Textarea> = create_node_ref(cx);

    // TODO: Validate invite code (create_effect ...)
    let (invite_code, set_invite_code) = create_signal::<String>(cx, "".to_string());

    let on_submit_join = move || {
        let invite = invite_code.get();
        join_action.dispatch(invite);
    };

    let joined = move || join_action.value().get().is_some();
    let disabled = move || invite_code.get().chars().count() == 0;

    view! { cx,
      <div class="h-[100dvh] w-full max-w-[600px] mx-auto p-6">

        <header class="w-full mb-20">
          <Logo />
        </header>
        <main class="w-full flex flex-col">
          <Show
            when=move || !joined()
              fallback=|_| empty_view()
            >
            <h1 class="font-heading text-gray-900 text-4xl font-semibold">"Join a Federation"</h1>
            <p class="font-body text-gray-900 text-xl">"Enter invite code (i.e. fed11jpr3lgm8t…) to join a Federation"</p>
            <form
              class="flex flex-col gap-2"
              on:submit=move |ev| {
                ev.prevent_default();
                on_submit_join();
              }
              >
              <textarea
                class="my-8 w-full text-xl font-body"
                rows="4"
                node_ref=invite_code_element
                placeholder="Invite Code"
                on:input=move |ev| {
                  set_invite_code.set(event_target_value(&ev));
                }
              />
              <input
                type="submit"
                value="Join"
                class="fm-btn-primary w-fit text-2xl"
                disabled=move || disabled()
              />
            </form>

          </Show>


          <Show when=move || join_action.pending().get()
            fallback=|_| empty_view()
            >
            <p>"Joining ..."</p>
          </Show>

          <Suspense
            fallback=move || view!{ cx, "Loading..."}
          >
          <ErrorBoundary fallback=|cx, error| view!{ cx, <p>{format!("Failed to create client: {:?}", error.get())}</p>}>
          { move || {
            join_action.value().get().map(|c| {
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
      </div>
    }
}
