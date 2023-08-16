use leptos::*;

use crate::context::ClientContext;

//
// Balance component
//
#[component]
pub fn Balance(cx: Scope, #[prop(optional, into)] class: String) -> impl IntoView {
    let ClientContext { balance, .. } = expect_context::<ClientContext>(cx);

    let balance_text = move || format! {"{:?} msat", balance.get().msats};

    view! { cx,
      <div class=class>
        <h2 class="text-xl leading-tight w-full pb-4 mb-4 text-gray-700 border-b-2 border-gray-700">"Balance"</h2>
        <h3 class="text-4xl">{balance_text}</h3>
      </div>
    }
}
