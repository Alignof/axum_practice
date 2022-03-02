use yew::{function_component, html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="fixed bottom-0 border-t-2 border-solid border-orange-500 text-white text-2xl pb-4">
            <div>
                { "copyright 2022- n.takana: " }
                <a href="https://yew.rs">{ "Yew" }</a><br/>
                { "contribute: visit " }
                <a href="https://github.com/Alignof/yew_practice">{ "https://github.com/Alignof/yew_practice" }</a>
            </div>
        </footer>
    }
}

