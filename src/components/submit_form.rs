use crate::components::SubmitButton;
use crate::utils::{empty_view, local_storage};
use leptos::ev::KeyboardEvent;
use leptos::*;
use leptos_qr_scanner::Scan;

#[derive(Clone)]
struct SavedFederation {
    name: String,
    code: String,
}

#[component]
pub fn SubmitForm<F>(
    cx: Scope,
    on_submit: F,
    placeholder: String,
    description: String,
    submit_label: String,
    loading: ReadSignal<bool>,
    #[prop(optional, into)] intro_screen: Option<bool>,
) -> impl IntoView
where
    F: Fn(String) + 'static + Copy,
{
    let (value, set_value) = create_signal(cx, String::new());

    let button_is_disabled = Signal::derive(cx, move || loading.get() || value.get().is_empty());
    let scan_disabled = Signal::derive(cx, move || loading.get());

    let (scan, set_scan) = create_signal(cx, false);

    // The federation name and invite code saved to local storage
    let (saved_federation, set_saved_federation) =
        create_signal::<Option<SavedFederation>>(cx, None);

    let textarea = view! {cx,
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

    let qr_scanner = view! { cx,
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

    // Load the saved federation name and invite code from local storage
    create_effect(cx, move |_| {
        let federation_name = local_storage().get_item("federation_name").unwrap();
        let invite_code = local_storage().get_item("invite_code").unwrap();

        if let Some(federation_name) = federation_name {
            if let Some(invite_code) = invite_code {
                set_saved_federation.set(Some(SavedFederation {
                    name: federation_name,
                    code: invite_code,
                }));
            }
        }
    });

    view! { cx,
      <form on:submit=|ev| ev.prevent_default()>

      <p class="font-body text-gray-600 text-xl">{description}</p>
      <Show
        when=move || scan.get()
        fallback=move |_| textarea.clone()
      >
        {qr_scanner.clone()}
      </Show>

      // Display the saved federation retrieved from local storage
      <Show
        when=move || saved_federation.get().is_some() && intro_screen.is_some()
        fallback=move |_| empty_view()
        >
        <div class="flex flex-col gap-2">
          <span class="font-bold text-gray-500">"Or join an existing federation"</span>
          <div
            class="mb-8 flex flex-row gap-2 border border-gray-200 rounded p-2 items-center cursor-pointer hover:border-gray-400 hover:bg-gray-50 transition-colors"
            on:click=move |_| {
              set_value.set(saved_federation.get().unwrap().code.clone());
            }
          >
            <span class="font-bold text-gray-700 text-2xl">{saved_federation.get().unwrap().name}</span>
            <span>" - "</span>
            <p class="text-gray-400 text-xl grow truncate">{saved_federation.get().unwrap().code}</p>
          </div>
        </div>
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
