use leptos::*;

use crate::context::ClientContext;
use crate::prediction_markets::js;

#[component]
pub fn ClientPayoutControl(cx: Scope) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    let pk_resource = create_resource(
        cx,
        || (),
        move |_| async move { client.get_value().get_client_payout_control().await },
    );
    let pk = create_memo(cx, move |_| match pk_resource.read(cx) {
        Some(Ok(pk)) => Some(pk),
        _ => None,
    });

    view! { cx,
        <div class="border-[1px] p-2">
            <p class="border-b text-lg">Your Payout Control</p>
            <div class="flex gap-3 p-2">
                <textarea readonly rows="1" class="flex-1 resize-none rounded p-3 bg-gray-100 border-gray-500">
                    {move || pk.get().map(|pk| pk.to_string()).unwrap_or("Loading...".into())}
                </textarea>
                <button
                    class="rounded border-spacing-5 p-3 bg-gray-100 border border-gray-500"
                    on:click=move |_| {pk.get().map(|pk| js::copy_text_to_clipboard(&pk.to_string()));}
                >
                    Copy
                </button>
            </div>
        </div>
    }
}
