use leptos::*;
use leptos_router::*;

#[allow(non_snake_case)]
mod App;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
        <div>
            <Routable />
        </div>
        }
    })
}

#[component]
fn Routable(cx: Scope) -> impl IntoView {
    view! { cx,
     <Router>
        <header style="height:10vh;width:100%;display:flex;align-items:center;justify-content:center;background:black;">
        <a href="/" style="margin:0 10px;color:white;">Home</a>
        <a href="/counter" style="margin:0 10px;color:white;">Counter</a>
        <a href="/uncontrolled" style="margin:0 10px;color:white;">Uncontrolled Form</a>
        <a href="/controlled" style="margin:0 10px;color:white;">Controlled Form</a>
    </header>

        <main>
        <Routes>
            <Route
                path="/counter"
                view=move |cx| view! {cx, <App::Counter /> }
                ></Route>
            <Route
                path="/uncontrolled"
                view=move |cx| view! {cx, <App::UncontrolledForm /> }
                ></Route>
            <Route
                path="/controlled"
                view=move |cx| view! {cx, <App::ControlledForm /> }
                ></Route>
            <Route
                path="/"
                view=move |cx| view! {cx, <div>"hello leptos"</div> }
                ></Route>

        </Routes>
        </main>
    </Router>
    }
}
