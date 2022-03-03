mod components;

use crate::components::{
    header::Header,
    footer::Footer,
    home::Home,
};

use yew::prelude::*;
use yew_router::prelude::*;

pub struct Model;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html!{ <Home /> },
        Route::About => html!{ <h1>{ "about" }</h1> },
        Route::NotFound => html!{ <h1>{ "404" }</h1> },
    }
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Header />

                <main class="text-orange-300 mt-32 font-sans-serif text-center text-2xl">
                        <Switch<Route> render={Switch::render(switch)} />
                </main>

                <Footer />
            </BrowserRouter>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
