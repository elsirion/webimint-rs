use leptos::*;

use crate::components::LogoFedimint;

#[component]
pub fn Footer(version: &'static str, #[prop(optional, into)] class: String) -> impl IntoView {
    let version_prefix: &'static str = &version[..7];
    view! {
      <div class={format!("flex justify-center items-center text-body text-sm text-gray-500 {class}")}>
        "Webimint version "
        <a
          class="text-gray-950 font-mono mx-1"
          href={ format!("https://github.com/elsirion/webimint-rs/commit/{version}") }
        >
          { version_prefix }
        </a>
        " powered by " <a class="opacity-50 hover:opacity-100 ease" href="https://fedimint.org/"><LogoFedimint class="ml-2 w-[100px] h-[23px]" /></a>
      </div>
    }
}
