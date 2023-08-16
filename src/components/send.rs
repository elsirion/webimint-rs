use leptos::*;

use crate::components::SubmitForm;
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

    view! { cx,

      <SubmitForm
        description="Enter LN invoice (i.e. lnbcrt1p0…) to send a payment".into()
        on_submit=move |v| submit_action.dispatch(v)
        placeholder="LN invoice".into()
        submit_label="Send".into()
        loading=submit_action.pending()
      />

      <p>{move || {
            match submit_action.value().get() {
            Some(result) =>
              match result {
                Err(error) => format!("✗ Failed to send invoice {:?}", error),
                Ok(_) => "✓ Invoice successfully sent".into()
              }
            None => "".into()
          }
        }
      }</p>

    }
}
