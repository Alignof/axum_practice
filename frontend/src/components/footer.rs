use yew::{function_component, html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="fixed bottom-0 w-full bg-blue-900 text-white text-2xl">
            <div class="m-2">
                { "copyright 2021-2022 n.takana" }<br/>
                { "contribute: visit " }
                <a href="https://github.com/Alignof/HCCC_Infra">{ "https://github.com/Alignof/HCCC_Infra" }</a>
            </div>
        </footer>
    }
}

