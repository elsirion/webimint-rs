use leptos::ev::KeyboardEvent;
use leptos::*;

use crate::components::SubmitButton;

#[component]
pub fn SubmitForm<F>(
    cx: Scope,
    on_submit: F,
    placeholder: String,
    description: String,
    submit_label: String,
    loading: ReadSignal<bool>,
) -> impl IntoView
where
    F: Fn(String) + 'static + Copy,
{
    let (value, set_value) = create_signal(cx, String::new());

    let button_is_disabled = Signal::derive(cx, move || loading.get() || value.get().is_empty());

    view! { cx,
      <form on:submit=|ev| ev.prevent_default()>

      <p class="font-body text-gray-900 text-xl">{description}</p>
      <textarea
        class="my-8 w-full text-xl font-body text-gray-600 border-gray-400 placeholder:text-gray-400 ring-0 focus:border-blue-400 focus:ring-0"
        rows="4"
        required
        placeholder=placeholder
        prop:disabled=move || loading.get()
        prop:value=move || value.get()
        on:keydown=move |ev: KeyboardEvent| {
          if ev.key() == "Enter" {
              ev.prevent_default();
              on_submit(value.get());
          }
        }
        on:input=move |ev| {
          let val = event_target_value(&ev);
          set_value.set(val);
        }
        on:paste=move |ev| {
          let val = event_target_value(&ev);
          set_value.set(val);
        }
      />

      <SubmitButton
          class="w-full"
        loading=loading
        disabled=button_is_disabled
        on_click=move |_| {
          on_submit(value.get());
        }
      >{submit_label}</SubmitButton>
      </form>
    }
}
