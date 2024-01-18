use leptos::*;

use crate::context::ClientContext;
use crate::prediction_markets::js;

#[component]
pub fn PayoutControlPublicKey(cx: Scope) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    let pk_resource = create_resource(
        cx,
        || (),
        move |_| async move { client.get_value().get_payout_control_public_key().await },
    );
    let pk = move || {
        let Some(pk_result) = pk_resource.read(cx) else {
            return None
        };
        pk_result.ok().map(|pk| pk.to_string())
    };

    view! { cx,
        <p>client payout control</p>
        <div class="flex gap-3">
            <textarea readonly rows="1" class="flex-1 resize-none rounded p-3 bg-gray-100 border-gray-500">
                {move || pk().unwrap_or("Loading...".into())}
            </textarea>
            <button 
                class="rounded border-spacing-5 p-3 bg-gray-100 border border-gray-500"
                on:click=move |_| {pk().map(|pk| js::copy_text_to_clipboard(&pk));}
            >
                Copy
            </button>
        </div>
    }
}