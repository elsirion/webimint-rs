use leptos::*;

use crate::context::ClientContext;

use crate::components::{Balance, Receive, Send};

//
// Joined component
// First view whenever an user joined a Federation
//
#[component]
pub fn Joined(cx: Scope) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    // get name of the federation
    let name_resource = create_resource(
        cx,
        || (),
        move |_| async move { client.get_value().get_name().await },
    );

    let federation_label = move || {
        name_resource
            .read(cx)
            .map(|value| match value {
                Err(error) => format!("Failed to get federation name {error:?}"),
                Ok(value) => format!("Joined {value:?}"),
            })
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };

    view! { cx,
      <p>{federation_label}</p>
      <Balance />
      <Receive />
      <Send />
    }
}
