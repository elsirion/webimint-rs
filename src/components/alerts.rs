use leptos::*;

#[component]
pub fn NoteBlock(children: Children) -> impl IntoView {
    view! {
        <div class="bg-yellow-100 border-l-4 border-yellow-500 text-yellow-700 p-4 mb-8 w-full" role="alert">
          <p class="font-bold">Note</p>
          <p>{ children() }</p>
        </div>
    }
}

#[component]
pub fn SuccessBlock(children: Children) -> impl IntoView {
    view! {
        <div class="bg-green-100 border-l-4 border-green-500 text-green-700 p-4 mb-8 w-full" role="alert">
            <p class="font-bold">Success</p>
            <p>{ children() }</p>
        </div>
    }
}

#[component]
pub fn WarningBlock(children: Children) -> impl IntoView {
    view! {
        <div class="bg-orange-100 border-l-4 border-orange-500 text-orange-700 p-4 mb-8 w-full" role="alert">
            <p class="font-bold">Warning</p>
            <p>{ children() }</p>
        </div>
    }
}

#[component]
pub fn ErrorBlock(children: Children) -> impl IntoView {
    view! {
        <div class="bg-orange-100 border-l-4 border-orange-500 text-orange-700 p-4 mb-8 w-full" role="alert">
            <p class="font-bold">Error</p>
            <p>{ children() }</p>
        </div>
    }
}
