use yew::prelude::*;

use crate::components::hero_section::HeroSection;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="flex min-h-screen flex-col bg-[#121212] container mx-auto px-12 py-4">
            <HeroSection />
        </main>
    }
}
