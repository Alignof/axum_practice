use yew::{function_component, html};
use crate::Msg;

#[function_component(Home)]
pub fn home() -> Html {
    // style for tailwind
    let btn_sty = "bg-white text-blue-800 m-3 rounded py-2 px-4";

    html! {
        <div class="m-5">
            <button class={btn_sty} onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
            <button class={btn_sty} onclick={link.callback(|_| Msg::MinusOne)}>{ "-1" }</button>
            <p class="text-3xl">{ self.value }</p>
        </div>
    }
}



