use leptos::{ev::SubmitEvent, *};

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

#[component]
pub fn uncontrolled_form(cx: Scope) -> impl IntoView {
    // import the type for <input>
    use leptos::html::Input;

    let (name, set_name) = create_signal(cx, "Uncontrolled".to_string());
    let input_element: NodeRef<Input> = create_node_ref(cx);
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element.get().expect("<input> to exist").value();
        set_name.set(value);
    };

    view! {
        cx,
        <form on:submit=on_submit>
            <input
                type="text"
                ref=input_element
                prop:value=move || name.get()
            />
            <p>"name is:  " {name}</p>
        </form>
    }
}

#[component]
pub fn controlled_form(cx: Scope) -> impl IntoView {
    println!("controlled_form {:?}", cx);
    let (name, set_name) = create_signal(cx, "Controlled".to_string());
    view! { cx,
      <div>
        <input
          type="text"
          on:input=move |ev| {
            set_name.set(event_target_value(&ev));
          }
          prop:value=move || name.get()

        />
        <p>"name is:  " {name}</p>
      </div>
    }
}
