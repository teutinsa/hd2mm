use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window"])]
    fn getCurrentWindow() -> JsWindow;
}

#[wasm_bindgen]
extern "C" {
    type JsWindow;

    #[wasm_bindgen(method)]
    fn minimize(this: &JsWindow);

    #[wasm_bindgen(method)]
    fn toggleMaximize(this: &JsWindow);

    #[wasm_bindgen(method)]
    fn close(this: &JsWindow);
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_help: Callback<MouseEvent>,
}

#[function_component]
pub fn TitleBar(props: &Props) -> Html {
    let has_update = use_state(|| false);

    {
        let has_update = has_update.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                
            });
            || ()
        });
    }
    
    let on_update = {
        let has_update = has_update.clone();
        Callback::from(move |_| {
            if !*has_update {
                return;
            }
        })
    };
    let on_minimize = Callback::from(|_| getCurrentWindow().minimize());
    let on_maximize = Callback::from(|_| getCurrentWindow().toggleMaximize());
    let on_close = Callback::from(|_| getCurrentWindow().close());
    
    html! {
        <div class={classes!("titlebar")} data-tauri-drag-region="true">
            <img src="public/images/logo.png"/>
            <span>{"Mod Manager"}</span>
            if *has_update {
                <button class={classes!("update-button")} onclick={on_update}>
                    <svg width="15" height="15">
                        <path d="M7,1 L7,13 M1,8 L7,13 L13,8" style="fill:none;stroke-width:2"/>
                    </svg>
                </button>
            }
            <button class={classes!("help-button")} onclick={props.on_help.clone()}>
                {"?"}
            </button>
            <button onclick={on_minimize}>
                <svg width="15" height="15">
                    <path d="M1,13 L13,13" style="fill:none;stroke-width:2"/>
                </svg>
            </button>
            <button onclick={on_maximize}>
                <svg width="15" height="15">
                    <path d="M1,1 L13,1 L13,13 L1,13 Z" style="fill:none;stroke-width:2"/>
                </svg>
            </button>
            <button class={classes!("close-button")} onclick={on_close}>
                <svg width="15" height="15">
                    <path d="M0,0 L15,15 M15,0 L0,15" style="fill:none;stroke-width:2"/>
                </svg>
            </button>
        </div>
    }
}