use leptos::*;

use crate::components::{Balance, ReceiveEcash, ReceiveLn, SendEcash, SendLn, TxList};
use crate::context::ClientContext;

//
// Joined component
// First view whenever an user joined a Federation
//
#[component]
pub fn Joined() -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>();

    // get name of the federation
    let name_resource = create_resource(
        || (),
        move |_| async move { client.get_value().get_name().await },
    );

    let federation_label = move || {
        name_resource
            .get()
            .map(|value| match value {
                Err(error) => format!("Failed to get federation name {error:?}"),
                Ok(value) => value,
            })
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };

    let tab_change_signal = create_rw_signal(());

    let menu_items = vec![
        MenuItem {
            title: "Transactions".into(),
            view: view! { <TxList update_signal=move || tab_change_signal.get() /> },
        },
        MenuItem {
            title: "Spend".into(),
            view: view! { <SendEcash /> },
        },
        MenuItem {
            title: "Redeem".into(),
            view: view! { <ReceiveEcash /> },
        },
        MenuItem {
            title: "LN Send".into(),
            view: view! { <SendLn /> },
        },
        MenuItem {
            title: "LN Receive".into(),
            view: view! { <ReceiveLn /> },
        },
    ];

    view! {
        <h1 class="font-heading text-gray-900 text-4xl font-semibold">{federation_label}</h1>
        <Balance class="my-12" />
        <Menu
            items=menu_items
            on_tab_change=move || tab_change_signal.set(())
            initial_item=1
        />
    }
}

struct MenuItem {
    title: String,
    view: View,
}

#[component]
fn Menu<F>(
    items: Vec<MenuItem>,
    on_tab_change: F,
    #[prop(default = 0)] initial_item: usize,
) -> impl IntoView
where
    F: Fn() + Copy + 'static,
{
    let (tab, set_tab) = create_signal(initial_item);

    view! {
        <ul class="my-12 w-full flex flex-row">
            {
                items.iter().enumerate().map(|(i, item)| {
                    view! {
                        <li class="flex-1">
                            <button
                                on:click=move |_| {
                                    set_tab.set(i);
                                    on_tab_change();
                                }
                                class={move || format!("my-2 block w-full text-center
                                    border-b-2 py-4 ease font-body font-semibold
                                    text-xl leading-tight hover:text-blue-500 {active}",
                                    active = if tab.get() == i {"text-blue-400 border-blue-400"}
                                        else {"text-gray-400 border-gray-200 hover:border-gray-700"})}
                            >
                                { item.title.to_owned() }
                            </button>
                        </li>
                    }
                }).collect_view()
            }
        </ul>
        {
            items.iter().enumerate().map(|(i, item)| {
                let view = item.view.to_owned();

                view! {
                    <Show when=move || tab.get() == i>
                        { view.to_owned() }
                    </Show>
                }
            }).collect_view()
        }
    }
}
