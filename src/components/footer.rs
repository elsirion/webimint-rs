use leptos::*;

use crate::components::LogoFedimint;

#[component]
pub fn Footer(#[prop(optional, into)] class: String) -> impl IntoView {
    view! {
      <div class={format!("flex justify-center items-center text-body text-sm text-gray-500 {class}")}>
        "Powered by " <a class="opacity-50 hover:opacity-100 ease" href="https://fedimint.org/"><LogoFedimint class="ml-2 w-[100px] h-[23px]" /></a>
      </div>
    }
}
