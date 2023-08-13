use crate::components::Joined;
use leptos::ev::SubmitEvent;

use crate::client::ClientRpc;
use crate::context::provide_client_context;
use crate::utils::empty_view;
use leptos::html::Input;
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

    let invite_code_element: NodeRef<Input> = create_node_ref(cx);

    let on_submit_join = move |ev: SubmitEvent| {
        ev.prevent_default();

        let invite = invite_code_element.get().expect("<input> to exist").value();
        join_action.dispatch(invite);
    };

    let joined = move || join_action.value().get().is_some();

    view! { cx,

      <Show
      when=move || !joined()
        fallback=|_| empty_view()
        >
        <p>"Join a federation"</p>
        <form on:submit=on_submit_join>
            // TODO: Validate invite code. Listen to `on:change`
            <input
                type="text"
                node_ref=invite_code_element
                placeholder="Invite Code, i.e. fed11jpr3lgm8t…"
                prop:disabled=joined()
            />
            <input
                type="submit"
                value="Join Federation"
                prop:disabled=joined()
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

    }
}
