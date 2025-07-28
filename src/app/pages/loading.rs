use yew::prelude::*;

#[function_component]
pub fn LoadingPage() -> Html {

    html! {
        <div class={classes!("load-spinner")}></div>
    }
}