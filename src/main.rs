use leptos::{ev::SubmitEvent, *};
mod App;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
        <div>
         <App::Counter />
         <UncontrolledForm />
        </div>
        }
    })
}

#[component]
fn uncontrolled_form(cx: Scope) -> impl IntoView {
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
                prop:value=name.get()
            />
            <p>"name is:  " {name}</p>
        </form>
    }
}
