use std::rc::Rc;
use yew::Reducible;

#[derive(Clone, PartialEq)]
pub struct SettingsContext {
    
}

impl SettingsContext {
    pub async fn init(&mut self) {
        
    }
}

impl Default for SettingsContext {
    fn default() -> Self {
        Self {

        }
    }
}

pub enum SettingsContextAction {
    Reset
}

impl Reducible for SettingsContext {
    type Action = SettingsContextAction;
    
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            SettingsContextAction::Reset => {

            }
        }
        self
    }
}