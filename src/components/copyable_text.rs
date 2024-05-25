use leptos::*;
use leptos_use::{use_clipboard, UseClipboardReturn};
use web_sys::HtmlTextAreaElement;

#[component]
pub fn CopyableText(
    #[prop(into)] text: Signal<String>,
    #[prop(default = 4)] rows: usize,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let UseClipboardReturn {
        is_supported,
        text: _,
        copied,
        copy,
    } = use_clipboard();

    view! {
        <div class=format!("flex flex-col gap-2 {}", class)>
            <textarea
                readonly
                prop:rows={rows}
                class="w-full p-3 break-all resize-none text-gray-600 border-gray-400 ring-0 focus:border-blue-400"
                style="font-family: mono"
                on:focus=|ev| {
                    let t = event_target::<HtmlTextAreaElement>(&ev);
                    t.select();
                }
                on:mouseup=|ev| ev.prevent_default()
            >
                { text }
            </textarea>
            <Show when=move || is_supported.get() >
                <button
                    on:click={
                        let copy = copy.clone();
                        move |_| copy(text.get().as_str())
                    }
                    class="w-full py-3 bg-blue-500 hover:enabled:bg-blue-600
                        text-white font-semibold font-body rounded-lg cursor-pointer enabled:ease disabled:opacity-70
                        disabled:cursor-not-allowed hover:enabled:shadow-lg"
                >
                    { move || {
                        if copied.get() {
                            "Copied!"
                        } else {
                            "Copy"
                        }
                    }}
                </button>
            </Show>
        </div>
    }
}
