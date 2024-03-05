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
                    <p class="text-[#adb7be] text-lg mb-6 lg:text-xl">
                        { "Lorem ipsum dolor sit amet consectetur adipisicing elit. Error ullam dignissimos dicta amet, doloribus nisi molestiae maiores numquam fuga eos consequuntur saepe eum, alias reprehenderit deleniti! Harum aliquam molestias non." }
                    </p>
                    <div>
                        <button></button>
                        <button></button>
                    </div>
                </div>
                <div class="col-span-5 place-self-center mt-4 lg:mt-0">
                    <div class="rounded-full bg-[#181818] w-[250px] h-[250px] lg:w-[400px] lg:h-[400px] relative">
                        <img src="static/headshot.png" alt="imagen de perfil" class="absolute transform -translate-x-1/2 -translate-y-1/2 top-1/2 left-1/2" width="325" height="325"/>
                    </div>
                </div>
            </div>
        </section>
    }
}
