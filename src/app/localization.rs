use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Localization {

}

impl Localization {
    
}

impl Default for Localization {
    fn default() -> Self {
        Self {

        }
    }
}

pub enum LocalizationAction {
    ChangeLanguage(&'static str)
}

impl Reducible for Localization {
    type Action = LocalizationAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            LocalizationAction::ChangeLanguage(code) => {

            }
        }
        self
    }
}