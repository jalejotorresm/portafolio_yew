use yew::prelude::*;

#[function_component(HeroSection)]
pub fn hero_section() -> Html {
    html! {
        <section>
            <div class="grid grid-cols-1 lg:grid-cols-12">
                <div class="col-span-7 place-self-center">
                    <h1 class="text-white mb-4 text-4xl lg:text-6xl font-extrabold">
                        { "Hola a todos, soy Alejandro" }
                    </h1>
                    <p class="text-[#adb7be] text-lg lg:text-xl">
                        { "Lorem ipsum dolor sit amet consectetur adipisicing elit. Error ullam dignissimos dicta amet, doloribus nisi molestiae maiores numquam fuga eos consequuntur saepe eum, alias reprehenderit deleniti! Harum aliquam molestias non." }
                    </p>
                </div>
                <div class="col-span-5"></div>
            </div>
        </section>
    }
}
