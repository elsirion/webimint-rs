use itertools::Itertools;
use leptos::*;

#[component]
pub fn SegmentedButton<F>(
    #[prop(default = 0)] active_idx: usize,
    segments: Vec<String>,
    on_change: F,
) -> impl IntoView
where
    F: Fn(usize) + 'static + Copy,
{
    const ALL_CLASS: &str = "px-4 py-2 focus:outline-none flex-1 text-center";
    const FIRST_CLASS: &str = "rounded-l-full";
    const LAST_CLASS: &str = "rounded-r-full";
    const ACTIVE_CLASS: &str = "text-white bg-blue-500";
    const INACTIVE_CLASS: &str = "text-blue-500 bg-transparent";

    let (active_idx, set_active_idx) = create_signal(active_idx);
    let num_segments = segments.len();

    let buttons = segments
        .into_iter()
        .enumerate()
        .map(|(idx, name)| {
            let class = move || {
                let is_first = idx == 0;
                let is_last = idx == num_segments - 1;
                let is_active = idx == active_idx.get();

                std::iter::once(ALL_CLASS)
                    .chain(is_first.then(|| FIRST_CLASS))
                    .chain(is_last.then(|| LAST_CLASS))
                    .chain(is_active.then(|| ACTIVE_CLASS))
                    .chain((!is_active).then(|| INACTIVE_CLASS))
                    .join(" ")
            };
            view! {
              <button
                class=class
                on:click=move |_| {
                    set_active_idx.set(idx);
                    on_change(idx);
                }
              >
                {name}
              </button>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <div class="bg-white p-1 rounded-full shadow flex w-full mb-8">
            {buttons}
        </div>
    }
}
