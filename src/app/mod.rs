mod titlebar;
mod contexts;
mod pages;
mod components;

use std::panic::catch_unwind;
use std::panic::AssertUnwindSafe;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use titlebar::TitleBar;
use contexts::MessageBoxContext;
use contexts::message_box::MessageBoxState;
use contexts::SettingsContext;
use contexts::LocalizationContext;
use contexts::localization::LocalizationState;
use contexts::ErrorContext;
use pages::NotFoundPage;
use pages::ErrorPage;
use pages::LoadingPage;
use pages::DashboardPage;
use components::MessageBox;

#[derive(Routable, PartialEq, Eq, Clone)]
pub enum Route {
    #[at("/")]
    Dashboard,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Dashboard => html!(<DashboardPage/>),
        Route::NotFound => html!(<NotFoundPage/>),
    }
}

#[function_component]
pub fn App() -> Html {
    let error = use_state_eq(|| ErrorContext::default());
    let loading = use_state(|| true);
    let settings = use_state_eq(|| SettingsContext::default());
    let localization = use_reducer_eq(|| LocalizationState::default());
    let message_box = use_reducer_eq(|| MessageBoxState::default());
    
    {
        let error = error.clone();
        let loading = loading.clone();
        let settings = settings.clone();
        let localization = localization.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                
                loading.set(false);
            });
            || ()
        });
    }

    let on_help = {
        let message_box = message_box.clone();
        Callback::from(move |_| {
            use contexts::message_box::*;
            message_box.dispatch(MessageBoxAction::Push(Message::Info("Not implemented.".to_owned())));
        })
    };

    let result = catch_unwind(AssertUnwindSafe(|| {
        html! {
            <ContextProvider<ErrorContext> context={(*error).clone()}>
                <TitleBar on_help={on_help}/>
                if (*error).has_error() {
                    <ErrorPage/>
                } else {
                    if *loading {
                        <LoadingPage/>
                    } else {
                        <ContextProvider<SettingsContext> context={(*settings).clone()}>
                            <ContextProvider<LocalizationContext> context={localization}>
                                <ContextProvider<MessageBoxContext> context={message_box}>
                                    <BrowserRouter>
                                        <Switch<Route> render={switch}/>
                                    </BrowserRouter>
                                    <MessageBox/>
                                </ContextProvider<MessageBoxContext>>
                            </ContextProvider<LocalizationContext>>
                        </ContextProvider<SettingsContext>>
                    }
                }
            </ContextProvider<ErrorContext>>
        }
    }));
    
    match result {
        Ok(html) => html,
        Err(err) => html! {
            <p class={classes!("critical", "scroll-v")}>
                {format!("Critical failure!\n{:?}", err)}
            </p>
        }
    }
}
