use leptos::*;

use crate::components::SubmitForm;
use crate::context::ClientContext;

//
// Receive e-cash component
//
#[component]
pub fn Receive() -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>();

    let client = client.clone();
    let submit_action = create_action(move |invoice: &String| {
        let invoice = invoice.clone();
        async move { client.get_value().receive(invoice).await }
    });

    view! {

      <SubmitForm
        description="Enter e-cash notes (i.e. BAQB6ijaAs0mXNoyKYvhI…) to redeem".into()
        on_submit=move |v| submit_action.dispatch(v)
        placeholder="e-cash notes".into()
        submit_label="Redeem".into()
        loading=submit_action.pending()
      />

      {move ||
        if let Some(result) = submit_action.value().get() {
          view!(
            <div class="text-body text-md mt-4">{
              match result {
                Err(error) => view!(<span class="text-red-500">{format!("✗ Failed to redeem e-cash: {error}")}</span>),
                Ok(value) => view!(<span class="text-green-600">{format!("✓ Redeemed {:?} msat", value.msats)}</span>)
              }
            }
            </div>)
        } else  {
          view!(<div></div>)
        }
      }

    }
}
