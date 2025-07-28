use yew::prelude::*;
use crate::app::contexts::MessageBoxContext;
use crate::app::contexts::message_box::Message;

#[function_component]
pub fn MessageBox() -> Html {
    let ctx = use_context::<MessageBoxContext>();

    if ctx.is_none() {
        return html!();
    }

    let ctx = ctx.unwrap();

    if let Some(msg) = ctx.get_top() {
        match msg {
            Message::Info(msg) => html! {
                <div class={classes!("message-box-shadow")}>
                    <div class={classes!("message-box")}>
                        
                    </div>
                </div>
            },
            Message::Warning(msg) => html! {
                <div class={classes!("message-box-shadow")}>
                    <div class={classes!("message-box")}>
                        
                    </div>
                </div>
            },
            Message::Error(msg) => html! {
                <div class={classes!("message-box-shadow")}>
                    <div class={classes!("message-box")}>
                        
                    </div>
                </div>
            },
            Message::Confirm {
                question,
                callback
            } => {
                html! {
                    <div class={classes!("message-box-shadow")}>
                        <div class={classes!("message-box")}>
                            
                        </div>
                    </div>
                }
            },
        }
    } else {
        html!()
    }
}