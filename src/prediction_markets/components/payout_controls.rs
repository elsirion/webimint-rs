use std::str::FromStr;

use leptos::*;
use secp256k1::PublicKey;
use tracing::warn;

use super::PredictionMarketsStaticDataContext;
use crate::context::ClientContext;
use crate::prediction_markets::js;
use crate::utils::empty_view;

#[component]
pub fn PayoutControls() -> impl IntoView {
    let reload_name_to_payout_control_table = create_rw_signal(());

    view! {
        <div class="flex flex-col gap-2">
            <ClientPayoutControl />
            <SetNameToPayoutControl reload_name_to_payout_control_table=reload_name_to_payout_control_table />
            <NameToPayoutControlTable reload_name_to_payout_control_table=reload_name_to_payout_control_table />
        </div>
    }
}

#[component]
pub fn ClientPayoutControl() -> impl IntoView {
    let PredictionMarketsStaticDataContext {
        client_payout_control,
        general_consensus: _,
    } = expect_context::<PredictionMarketsStaticDataContext>();

    view! {
        <div class="border-[1px] p-2">
            <p class="border-b text-lg">"Your Payout Control"</p>
            <div class="flex gap-3 p-2">
                <textarea readonly rows="1" class="flex-1 resize-none rounded p-3 bg-gray-100">
                    {client_payout_control.to_string()}
                </textarea>
                <button
                    class="rounded border-spacing-5 p-3 bg-gray-100 border border-gray-500"
                    on:click=move |_| js::copy_text_to_clipboard(&client_payout_control.to_string())
                >
                    "Copy"
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn SetNameToPayoutControl(reload_name_to_payout_control_table: RwSignal<()>) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>();

    let form_name = create_rw_signal("".to_owned());
    let form_payout_control = create_rw_signal("".to_owned());

    let set_name_to_payout_control_action = create_action(move |()| async move {
        let name = form_name.get_untracked();
        if name.len() == 0 {
            return Err("Name must have non-zero length".to_owned());
        }
        let payout_control = PublicKey::from_str(form_payout_control.get_untracked().as_ref())
            .map_err(|e| format!("Error parsing public key: {e}"))?;

        form_name.set("".to_owned());
        form_payout_control.set("".to_owned());
        reload_name_to_payout_control_table.set(());

        Ok(client
            .get_value()
            .set_name_to_payout_control(name, Some(payout_control))
            .await
            .map_err(|e| format!("Error setting name to public key: {e}"))?)
    });

    view! {
        <div class="flex flex-col gap-1 border-[1px] p-2">
            <h2 class="border-b text-lg">"Assign Name to Payout Control"</h2>

            <label>"Name"</label>
            <input
                on:input=move |ev| form_name.set(event_target_value(&ev))
                prop:value=move || form_name.get()
            />

            <label>"Payout Control"</label>
            <input
                on:input=move |ev| form_payout_control.set(event_target_value(&ev))
                prop:value=move || form_payout_control.get()
            />

            <button
                class="border-[1px] p-2 hover:bg-slate-200"
                on:click=move |_| set_name_to_payout_control_action.dispatch(())
            >
                "Save"
            </button>
        </div>
    }
}

#[component]
pub fn NameToPayoutControlTable(
    reload_name_to_payout_control_table: RwSignal<()>,
) -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>();

    let name_to_payout_control_map_resource = create_resource(
        move || (),
        move |()| async move { client.get_value().get_name_to_payout_control_map().await },
    );
    create_effect(move |_| {
        _ = reload_name_to_payout_control_table.get();
        name_to_payout_control_map_resource.refetch();
    });

    let set_name_to_none_action = create_action(move |name: &String| {
        let name = name.to_owned();
        async move {
            _ = client
                .get_value()
                .set_name_to_payout_control(name, None)
                .await
                .map_err(|e| warn!("Error setting name to no payout control: {e}"));

            reload_name_to_payout_control_table.set(());
        }
    });

    let sorted_by_name = create_memo(move |_| {
        let mut v = vec![];
        let Some(Ok(m)) = name_to_payout_control_map_resource.get() else {
            return v;
        };
        for kv in m {
            v.push(kv)
        }
        v.sort_by(|(name1, _), (name2, _)| name1.cmp(name2));

        v
    });

    view! {
        <Show
            when=move || matches!{name_to_payout_control_map_resource.get(), Some(Ok(_))}
            fallback=|| empty_view()
        >
            <table>
                <thead>
                    <th>""</th>
                    <th class="border-[1px] p-2">"Name"</th>
                    <th class="border-[1px] p-2">"Payout Control"</th>
                </thead>
                {move || {
                    sorted_by_name
                        .get()
                        .into_iter()
                        .map(|(name, public_key)| {
                            let action_name = name.to_owned();
                            view! {
                                <tr>
                                    <td
                                        class="border-[1px] p-2 text-center cursor-pointer hover:bg-red-500"
                                        on:click=move |_| set_name_to_none_action.dispatch(action_name.to_owned())
                                    >
                                        "X"
                                    </td>
                                    <td class="border-[1px] p-2">{name}</td>
                                    <td class="border-[1px] p-2">{public_key.to_string()}</td>
                                </tr>
                            }
                        })
                        .collect_view()
                }}
            </table>
        </Show>
    }
}
