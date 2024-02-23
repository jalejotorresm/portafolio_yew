use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <main class="flex min-h-screen flex-col bg-black">
            <h1 class="text-white">{"Hola a todos"}</h1>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
