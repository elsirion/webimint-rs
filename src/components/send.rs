use leptos::*;

use crate::components::SubmitForm;
use crate::context::ClientContext;

//
// Receive LN component
//
#[component]
pub fn Send(cx: Scope) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    let submit_action = create_action(cx, move |invoice: &String| {
        let invoice = invoice.clone();
        async move { client.get_value().ln_send(invoice).await }
    });

    view! { cx,

      <SubmitForm
        description="Enter LN invoice (i.e. lnbcrt1p0…) to send a payment".into()
        on_submit=move |v| submit_action.dispatch(v)
        placeholder="LN invoice".into()
        submit_label="Send".into()
        loading=submit_action.pending()
      />


      {move ||
        if let Some(result) = submit_action.value().get() {
          view!(cx,
            <div class="text-body text-md mt-4">{
              match result {
                Err(error) => view!(cx, <span class="text-red-500">{format!("✗ Failed to send invoice {error}")}</span>),
                Ok(_) => view!(cx, <span class="text-green-600">"✓ Invoice successfully sent"</span>)
              }
            }
            </div>)
        } else  {
          view!(cx, <div></div>)
        }
      }

    }
}
