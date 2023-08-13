use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::*;

use crate::context::ClientContext;

//
// Receive e-cash component
//
#[component]
pub fn Receive(cx: Scope) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    let client = client.clone();
    let submit_action = create_action(cx, move |invoice: &String| {
        let invoice = invoice.clone();
        async move { client.get_value().receive(invoice).await }
    });

    let input_ref: NodeRef<Input> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        // TODO: Validate value
        let value = input_ref.get().expect("<input> to exist").value();

        submit_action.dispatch(value);
    };

    view! { cx,

        <form on:submit=on_submit>
            <input
                type="text"
                placeholder="e-cash notes, i.e. BAQB6ijaAs0mXNoyKYvhI…"
                node_ref=input_ref
            />
            <input
                type="submit"
                value="Redeem e-cash"
            />
            <Show
              when=move || !submit_action.pending().get()
              fallback=move |_| view!{ cx, "..."}
              >
              <p>{move || {
                    match submit_action.value().get() {
                    Some(result) =>
                      match result {
                        Err(error) => format!("✗ Failed to redeem e-cash: {:?}", error),
                        Ok(value) => format!("✓ Redeemed {:?} msat", value.msats)
                      }
                    None => "".to_string()
                  }
                }
              }</p>
            </Show>
        </form>

    }
}
