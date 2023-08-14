use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::*;

use crate::context::ClientContext;

//
// Receive LN component
//
#[component]
pub fn Send(cx: Scope) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    let client = client.clone();
    let submit_action = create_action(cx, move |invoice: &String| {
        let invoice = invoice.clone();
        async move { client.get_value().ln_send(invoice).await }
    });

    let input_ref: NodeRef<Input> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_ref.get().expect("<input> to exist").value();
        // TODO: Validate value
        submit_action.dispatch(value);
    };

    view! { cx,
      <form on:submit=on_submit>
            <input
                type="text"
                placeholder="LN invoice, i.e. lnbcrt1p0…"
                node_ref=input_ref
            />
            <input
                type="submit"
                value="Pay LN invoice"
            />
            <Show
            when=move || !submit_action.pending().get()
            fallback=move |_| view!{ cx, "..."}
            >
            <p>{move || {
                  match submit_action.value().get() {
                  Some(result) =>
                    match result {
                      Err(error) => format!("✗ Failed to send invoice {:?}", error),
                      Ok(_) => "✓ Invoice successfully sent".to_string()
                    }
                  None => "".to_string()
                }
              }
            }</p>
          </Show>
        </form>

    }
}
