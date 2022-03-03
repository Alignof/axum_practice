use yew::{function_component, html};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="fixed top-0 w-full p-5 bg-black text-white">
            <div class="inset-y-0 left-0 w-50 text-center">
                <div class="text-4xl"> { "HCCC" } </div>
                <div class="text-xl"> { "- Human C Compiler Contest -" } </div>
            </div>
        </header>
    }
}


