use yew::prelude::*;

use crate::app::contexts::ErrorContext;

#[function_component]
pub fn ErrorPage() -> Html {
    let error = use_context::<ErrorContext>();
    
    let message = match error {
        Some(ctx) => match ctx.error() {
            Some(err) => format!("{}", err),
            None => format!("No error set!\nWhy are we here?\n{} ({},{})", file!(), line!(), column!()),
        },
        None => format!("No error context found!\n{} ({},{})", file!(), line!(), column!()),
    };
    
    html! {
        <div class={classes!("error-page")}>
            <h1>{"Error"}</h1>
            <hr/>
            <div class={classes!("scroll-v")}>
                <span>{message}</span>
            </div>
        </div>
    }
}