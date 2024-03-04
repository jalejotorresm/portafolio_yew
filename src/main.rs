mod app;
mod components;

use crate::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
