use leptos::*;

#[component]
pub fn Counter(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
      <div>
        <button
            on:click=move |_| {
                set_count.update(|n| { *n += 1
                });
            }
        >"Count up "</button>

        <button
            on:click=move |_| {
                set_count.update(|n| { *n -= 1
                });
            }
        >"Count down "</button>

        <p>{move || count.get()}</p>
        </div>
    }
}
