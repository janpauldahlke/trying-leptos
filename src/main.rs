use leptos::*;
mod App;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
        <div>
         <App::Counter />
         <App::UncontrolledForm />
         <App::ControlledForm />
        </div>
        }
    })
}
