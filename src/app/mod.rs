mod titlebar;

use yew::prelude::*;
use titlebar::TitleBar;

#[function_component]
pub fn App() -> Html {
    let on_help = Callback::from(|_| {

    });

    html! {
        <>
            <TitleBar on_help={on_help}/>
        </>
    }
}
