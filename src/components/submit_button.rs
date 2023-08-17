use crate::components::LoaderIcon;
use crate::utils::empty_view;
use leptos::ev::MouseEvent;
use leptos::*;

#[component]
pub fn SubmitButton<F>(
    cx: Scope,
    loading: ReadSignal<bool>,
    disabled: Signal<bool>,
    on_click: F,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! { cx,

      <button
        on:click=on_click
        class={move || format!("py-4 px-6 text-2xl flex items-center justify-center bg-blue-500 hover:enabled:bg-blue-600 text-white font-semibold font-body rounded-lg cursor-pointer enabled:ease disabled:opacity-70 disabled:cursor-not-allowed hover:enabled:shadow-lg {class}")}
        disabled=move || disabled.get()
      >
        {children(cx)}
        <Show
          when=move || loading.get()
          fallback=move |_| empty_view()
        >
          <LoaderIcon class="ml-2" />
        </Show>
      </button>

    }
}
