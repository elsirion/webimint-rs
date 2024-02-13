use leptos::ev::MouseEvent;
use leptos::*;

use crate::components::LoaderIcon;
use crate::utils::empty_view;

#[component]
pub fn SubmitButton<F>(
    loading: ReadSignal<bool>,
    disabled: Signal<bool>,
    on_click: F,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! {

      <button
        on:click=on_click
        class={move || format!("py-4 px-6 text-2xl flex items-center justify-center bg-blue-500 hover:enabled:bg-blue-600 text-white font-semibold font-body rounded-lg cursor-pointer enabled:ease disabled:opacity-70 disabled:cursor-not-allowed hover:enabled:shadow-lg {class}")}
        disabled=move || disabled.get()
      >
        {children()}
        <Show
          when=move || loading.get()
          fallback=move || empty_view()
        >
          <LoaderIcon class="ml-2" />
        </Show>
      </button>

    }
}
