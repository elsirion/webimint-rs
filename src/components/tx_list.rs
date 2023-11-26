use crate::client::Transaction;
use leptos::*;
use leptos_icons::*;

use crate::components::LoaderIcon;
use crate::context::ClientContext;

//
// Receive e-cash component
//
#[component]
pub fn TxList<F>(cx: Scope, update_signal: F) -> impl IntoView
where
    F: Fn() + Copy + 'static,
{
    let ClientContext { client, .. } = expect_context::<ClientContext>(cx);

    let tx_list_resource = create_resource(cx, update_signal, move |()| async move {
        let client = client.get_value();
        client.list_transactions().await.expect("list tx failed")
    });

    view! { cx,
        <div>
            <Suspense
                fallback=move || {view! {cx, <LoaderIcon />}}
            >
                <table class="border-y border-slate-400 border-collapse table-auto w-full text-sm">
                    <thead class="bg-slate-50">
                        <tr class="border-y border-slate-300">
                            <th class="p-4">Type</th>
                            <th class="p-4">Description</th>
                            <th class="p-4">Amount</th>
                        </tr>
                    </thead>
                    <tbody>
                    {move || {
                        tx_list_resource.read(cx).map(|transactions| {
                            transactions.into_iter()
                                .map(|tx| {
                                    view! {cx, <TxListRow transaction=tx />}
                                })
                                .collect::<Vec<_>>()
                        })
                    }}
                    </tbody>
                </table>
            </Suspense>
        </div>
    }
}

#[component]
pub fn TxListRow(cx: Scope, transaction: Transaction) -> impl IntoView {
    view! { cx,
        <tr class="border-y border-slate-300">
            <td class="text-center p-4">
                {
                    match transaction.operation_kind.as_ref() {
                        "ln" => view! {cx, <Icon icon=icon!(BsLightningCharge) width="2em" height="2em"/>}.into_view(cx),
                        "mint" => view! {cx, <Icon icon=icon!(FaCoinsSolid) width="2em" height="2em"/>}.into_view(cx),
                        other => {
                            let kind = other.to_owned();
                            view! {cx, <span>{kind}</span>}.into_view(cx)
                        }
                    }
                }
            </td>
            <td class="p-4">
                {transaction.description}
            </td>
            <td class="p-4">
                <p
                    class={if transaction.amount_msat > 0 { "text-emerald-600 text-right" } else { "text-red-600 text-right" } }
                >
                    {transaction.amount_msat} " msat"
                </p>
            </td>
        </tr>
    }
}
