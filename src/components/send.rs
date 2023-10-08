use leptos::*;

use crate::components::SubmitForm;
use crate::context::ClientContext;

//
// Receive LN component
//
#[component]
pub fn Send() -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>();

    let client = client.clone();
    let submit_action = create_action(move |invoice: &String| {
        let invoice = invoice.clone();
        async move { client.get_value().ln_send(invoice).await }
    });

    view! {

      <SubmitForm
        description="Enter LN invoice (i.e. lnbcrt1p0…) to send a payment".into()
        on_submit=move |v| submit_action.dispatch(v)
        placeholder="LN invoice".into()
        submit_label="Send".into()
        loading=submit_action.pending()
      />


      {move ||
        if let Some(result) = submit_action.value().get() {
          view!(
            <div class="text-body text-md mt-4">{
              match result {
                Err(error) => view!(<span class="text-red-500">{format!("✗ Failed to send invoice {error}")}</span>),
                Ok(_) => view!(<span class="text-green-600">"✓ Invoice successfully sent"</span>)
              }
            }
            </div>)
        } else  {
          view!(<div></div>)
        }
      }

    }
}
