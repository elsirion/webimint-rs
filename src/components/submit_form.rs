use leptos::ev::KeyboardEvent;
use leptos::html::Form;
use leptos::*;
use leptos_qr_scanner::Scan;
use leptos_use::use_element_visibility;

use crate::components::SubmitButton;

#[component]
pub fn SubmitForm<F>(
    on_submit: F,
    placeholder: String,
    description: String,
    submit_label: String,
    loading: ReadSignal<bool>,
    #[prop(default = false)] default_scan: bool,
) -> impl IntoView
where
    F: Fn(String) + 'static + Copy,
{
    let (value, set_value) = create_signal(String::new());

    let button_is_disabled = Signal::derive(move || loading.get() || value.get().is_empty());
    let scan_disabled = Signal::derive(move || loading.get());

    let (scan, set_scan) = create_signal(default_scan);

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

    let form_ref = create_node_ref::<Form>();
    let is_visible = use_element_visibility(form_ref);

    let qr_scanner = view! {
        <Scan
          active=Signal::derive(move || scan.get() && is_visible.get())
          on_scan=move |invite| {
            set_scan.set(false);
            set_value.set(invite.clone());
            on_submit(invite);
          }
          class="my-8 mx-auto flex aspect-square w-4/5"
        />
    };

    view! {
      <form
        on:submit=|ev| ev.prevent_default()
        node_ref=form_ref
      >

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
          "Manual"
        } else {
          "Scan"
        }
      }}</SubmitButton>
      </div>
      </form>
    }
}
