use leptos::*;

use crate::context::ClientContext;

use crate::components::{Balance, Receive, Send};
use crate::utils::empty_view;

#[derive(Clone, PartialEq)]
enum Tab {
    Send,
    Receive,
}

//
// Joined component
// First view whenever an user joined a Federation
//
#[component]
pub fn Joined(cx: Scope) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    // get name of the federation
    let name_resource = create_resource(
        cx,
        || (),
        move |_| async move { client.get_value().get_name().await },
    );

    let federation_label = move || {
        name_resource
            .read(cx)
            .map(|value| match value {
                Err(error) => format!("Failed to get federation name {error:?}"),
                Ok(value) => value,
            })
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };

    let (tab, set_tab) = create_signal(cx, Tab::Send);

    view! { cx,
      <h1 class="font-heading text-gray-900 text-4xl font-semibold mb-6">{federation_label}</h1>
      <Balance class="my-6" />
      <ul
        class="my-12 w-full flex flex-row"
        >
        <li class="w-1/2">
          <button
            on:click=move |_| {
              set_tab.set(Tab::Send);
            }
            class={move || format!("my-2 block w-full text-center
            border-b-2 
            py-4
            ease
            font-body font-semibold  
            text-xl leading-tight hover:text-gray-700 {active}", 
            active = if tab.get() == Tab::Send {"text-gray-700 border-gray-700"} else {"text-gray-400 border-gray-200 hover:border-gray-700"} )}
            >Send
          </button>
        </li>
        <li class="w-1/2">
          <button
            on:click=move |_| {
              set_tab.set(Tab::Receive);
            }
            class={move || format!("my-2 block w-full text-center
            border-b-2 
            py-4
            ease
            font-body font-semibold  
            text-xl leading-tight hover:text-gray-700 {active}", 
            active = if tab.get() == Tab::Receive {"text-gray-700 border-gray-700"} else {"text-gray-400 border-gray-200 hover:border-gray-700"} )}

            >Redeem
          </button>
        </li>
      </ul>

      <Show
          when=move || tab.get() == Tab::Send
          fallback=|_| empty_view()
          >
          <Send />
      </Show>
      <Show
          when=move || tab.get() == Tab::Receive
          fallback=|_| empty_view()
          >
          <Receive />
      </Show>



    }
}