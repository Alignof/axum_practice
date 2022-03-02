use yew::{function_component, html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <div>
                { "copyright 2021- XXXXX: " }
                <a href="https://yew.rs">{ "Yew" }</a><br/>
                { "contribute: visit " }
                <a href="https://github.com/Alignof/yew_practice">{ "https://github.com/Alignof/yew_practice" }</a>
            </div>
        </footer>
    }
}

