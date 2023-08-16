use leptos::*;

use crate::components::SubmitForm;
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

    view! { cx,

      <SubmitForm
        description="Enter e-cash notes (i.e. BAQB6ijaAs0mXNoyKYvhI…) to redeem".into()
        on_submit=move |v| submit_action.dispatch(v)
        placeholder="e-cash notes".into()
        submit_label="Redeem".into()
        loading=submit_action.pending()
      />

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
    }
}
