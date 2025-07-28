use std::rc::Rc;
use yew::prelude::Reducible;

#[derive(Clone, PartialEq)]
pub struct LocalizationContext {
    initialized: bool,
}

impl LocalizationContext {
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub async fn init(&mut self) {
        
    }
}

impl Default for LocalizationContext {
    fn default() -> Self {
        Self {
            initialized: false,
        }
    }
}

pub enum LocalizationContextAction {
    ChangeLanguage(&'static str)
}

impl Reducible for LocalizationContext {
    type Action = LocalizationContextAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            LocalizationContextAction::ChangeLanguage(code) => {

            }
        }
        self
    }
}