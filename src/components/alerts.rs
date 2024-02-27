use leptos::*;

#[component]
pub fn NoteBlock(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!("bg-yellow-100 border-l-4 border-yellow-500 text-yellow-700 p-4 w-full {}", class) role="alert">
          <p class="font-bold">Note</p>
          <p>{ children() }</p>
        </div>
    }
}

#[component]
pub fn SuccessBlock(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!("bg-green-100 border-l-4 border-green-500 text-green-700 p-4 w-full {}", class) role="alert">
            <p class="font-bold">Success</p>
            <p>{ children() }</p>
        </div>
    }
}

#[component]
pub fn WarningBlock(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!("bg-orange-100 border-l-4 border-orange-500 text-orange-700 p-4 w-full {}", class) role="alert">
            <p class="font-bold">Warning</p>
            <p>{ children() }</p>
        </div>
    }
}

#[component]
pub fn ErrorBlock(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!("bg-orange-100 border-l-4 border-orange-500 text-orange-700 p-4 w-full {}", class) role="alert">
            <p class="font-bold">Error</p>
            <p>{ children() }</p>
        </div>
    }
}
