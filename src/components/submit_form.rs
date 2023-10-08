use leptos::ev::KeyboardEvent;
use leptos::*;
use leptos_qr_scanner::Scan;

use crate::components::SubmitButton;

#[component]
pub fn SubmitForm<F>(
    on_submit: F,
    placeholder: String,
    description: String,
    submit_label: String,
    loading: ReadSignal<bool>,
) -> impl IntoView
where
    F: Fn(String) + 'static + Copy,
{
    let (value, set_value) = create_signal(String::new());

    let button_is_disabled = Signal::derive(move || loading.get() || value.get().is_empty());
    let scan_disabled = Signal::derive(move || loading.get());

    let (scan, set_scan) = create_signal(false);

    let textarea = view! {
        <textarea
          class="my-8 w-full text-xl font-body text-gray-600 border-gray-400 placeholder:text-gray-400 ring-0 focus:border-blue-400 focus:ring-0"
          rows="4"
          required
          placeholder=placeholder.clone()
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
    };

    let qr_scanner = view! {
        <Scan
          active=scan
          on_scan=move |invite| {
            set_scan.set(false);
            set_value.set(invite.clone());
            on_submit(invite);
          }
          class="my-8"
        />
    };

    view! {
      <form on:submit=|ev| ev.prevent_default()>

      <p class="font-body text-gray-600 text-xl">{description}</p>
      <Show
        when=move || scan.get()
        fallback=move || textarea.clone()
      >
        {qr_scanner.clone()}
      </Show>

      <div class="flex space-x-4">
      <SubmitButton
        class="w-5/6"
        loading=loading
        disabled=button_is_disabled
        on_click=move |_| {
          on_submit(value.get());
        }
      >{submit_label}</SubmitButton>
      <SubmitButton
        class="w-1/6"
        loading=loading
        disabled=scan_disabled
        on_click=move |_| {
          set_scan.set(!scan.get());
        }
      >{move || {
        if scan.get() {
          "Stop"
        } else {
          "Scan"
        }
      }}</SubmitButton>
      </div>
      </form>
    }
}
