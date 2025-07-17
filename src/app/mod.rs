mod titlebar;
mod localization;

use yew::prelude::*;
use yew_router::prelude::*;
use titlebar::TitleBar;
use localization::Localization;

#[derive(Routable, PartialEq, Eq, Clone)]
pub enum Route {
    #[at("/")]
    Loading,
    #[not_found]
    #[at("/404")]
    NotFound
}

fn switch(route: Route) -> Html {
    html! {

    }
}

#[function_component]
pub fn App() -> Html {
    let on_help = Callback::from(|_| {

    });

    let localization = use_state_eq(|| Localization::default());

    html! {
        <ContextProvider<Localization> context={(*localization).clone()}>
            <TitleBar on_help={on_help}/>
            <BrowserRouter>
                <Switch<Route> render={switch}/>
            </BrowserRouter>
        </ContextProvider<Localization>>
    }
}
