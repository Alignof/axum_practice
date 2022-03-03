use yew::{function_component, html, Callback};
use yew_router::prelude::{History, use_history};
use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    let history = use_history().unwrap();
    let home_click = {
        let history = history.clone();
        let onclick = Callback::once(move |_| history.push(Route::Home));
        html! {
            <button class="m-2" {onclick}>{ "home" }</button>
        }
    };
    let about_click = {
        let history = history.clone();
        let onclick = Callback::once(move |_| history.push(Route::About));
        html! {
            <button class="m-2" {onclick}>{ "about" }</button>
        }
    };

    html! {
        <header class="fixed flex top-0 w-full p-2 items-center bg-black text-white">
            <div class="w-50 text-center">
                <div class="text-4xl"> { "HCCC" } </div>
                <div class="text-xl"> { "- Human C Compiler Contest -" } </div>
            </div>

            <div class="text-2xl ml-auto align-middle">
                {home_click}
                {about_click}
                <a class="m-2" href="https://github.com/Alignof/HCCC_Infra">{ "repo" }</a>
            </div>
        </header>
    }
}

