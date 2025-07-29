use yew::prelude::*;
use crate::app::contexts::MessageBoxContext;
use crate::app::contexts::message_box::{Message, MessageBoxAction};

#[function_component]
pub fn MessageBox() -> Html {
    let ctx = use_context::<MessageBoxContext>();

    if ctx.is_none() {
        return html!();
    }

    let ctx = ctx.unwrap();

    let pop_callback = {
        let ctx = ctx.clone();
        Callback::from(move |_| {
            println!("callback!");
            ctx.dispatch(MessageBoxAction::Pop)
        })
    };

    if let Some(msg) = ctx.get_top() {
        match msg {
            Message::Info(msg) => html! {
                <div class={classes!("message-box-shadow")}>
                    <div class={classes!("message-box", "notification-box", "info-box")}>
                        <h1>{"Info"}</h1>
                        <span>{msg}</span>
                        <button onclick={pop_callback}>{"OK"}</button>
                    </div>
                </div>
            },
            Message::Warning(msg) => html! {
                <div class={classes!("message-box-shadow")}>
                    <div class={classes!("message-box", "notification-box", "warning-box")}>
                        <h1>{"Warning"}</h1>
                        <span>{msg}</span>
                        <button onclick={pop_callback}>{"OK"}</button>
                    </div>
                </div>
            },
            Message::Error(msg) => html! {
                <div class={classes!("message-box-shadow")}>
                    <div class={classes!("message-box", "notification-box", "error-box")}>
                        <h1>{"Error"}</h1>
                        <span>{msg}</span>
                        <button onclick={pop_callback}>{"OK"}</button>
                    </div>
                </div>
            },
            Message::Confirm {
                question,
                callback
            } => {
                let yes_callback = {
                    let ctx = ctx.clone();
                    Callback::from(move |_| {
                        callback.emit(());
                        ctx.dispatch(MessageBoxAction::Pop);
                    })
                };

                html! {
                    <div class={classes!("message-box-shadow")}>
                        <div class={classes!("message-box", "confirm-box")}>
                            <span>{question}</span>
                            <div>
                                <button onclick={yes_callback}>{"Yes"}</button>
                                <button onclick={pop_callback}>{"No"}</button>
                            </div>
                        </div>
                    </div>
                }
            },
        }
    } else {
        html!()
    }
}