use leptos::*;
use leptos::ev::MouseEvent;

#[component]
pub fn Failure<F>(cx: Scope, on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! { cx,
      <div class="flex flex-col gap-6">
        <h1 class="font-heading text-gray-900 text-4xl font-semibold">"Failed"</h1>
        <p class="font-body text-gray-600 text-xl">"Failed to connect to the specified federation"</p>
        <button
          on:click=on_click
          class="py-4 px-6 text-2xl flex items-center justify-center bg-blue-500 hover:enabled:bg-blue-600 text-white font-semibold font-body rounded-lg cursor-pointer enabled:ease disabled:opacity-70 disabled:cursor-not-allowed hover:enabled:shadow-lg"
        >
          "Try again"
        </button>
      </div>
    }
}
