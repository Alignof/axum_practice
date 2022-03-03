use yew::{html, Component, Context, Html};

pub enum Msg {
    AddOne,
    MinusOne,
}

pub struct Home {
    value: i64,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            },
            Msg::MinusOne => {
                self.value -= 1;
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

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
}

