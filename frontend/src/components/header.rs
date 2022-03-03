use yew::{function_component, html};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="fixed top-0 w-full p-5 bg-black text-white text-2xl">
            <div>
                <p class="text-3xl"> { "HCCC" } </p>
            </div>
        </header>
    }
}


