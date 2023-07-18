use leptos::*;

#[allow(non_snake_case)]
mod App;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
        <div>
        <div>
            <header>
                <nav>
                    <a href="/counter">"Counter"</a>
                    <a href="/uncontrolled_form">"Uncontrolled Form"</a>
                    <a href="/controlled_form">"Controlled Form"</a>
                </nav>
            </header>
        </div>
         <App::Counter />
         <App::UncontrolledForm />
         <App::ControlledForm />

        </div>
        }
    })
}
