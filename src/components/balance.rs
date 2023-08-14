use leptos::*;

use crate::context::ClientContext;

//
// Balance component
//
#[component]
pub fn Balance(cx: Scope) -> impl IntoView {
    let ClientContext { balance, .. } = expect_context::<ClientContext>(cx);

    let balance_text = move || format! {"{:?} msat", balance.get().msats};

    view! { cx,
      <p>"Balance: " {balance_text}</p>
    }
}
