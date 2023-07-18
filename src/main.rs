use leptos::*;

#[allow(non_snake_case)]
mod App;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
        <div>
            <header style="height:10vh;width:100%;display:flex;align-items:center;justify-content:center;background:black;">
                <a href="/counter" style="margin:0 10px;color:white;">Counter</a>
                <a href="/uncontrolled_form" style="margin:0 10px;color:white;">Uncontrolled Form</a>
                <a href="/controlled_form" style="margin:0 10px;color:white;">Controlled Form</a>
            </header>

            <App::Counter />
            <App::UncontrolledForm />
            <App::ControlledForm />
            </div>
            }
    })
}
