use std::rc::Rc;
use yew::Reducible;

#[derive(Default, Clone)]
pub struct ErrorContext(Option<Rc<anyhow::Error>>);

impl ErrorContext {
    pub fn with_error(error: anyhow::Error) -> Self {
        Self(Some(Rc::new(error)))
    }

    pub fn has_error(&self) -> bool {
        self.0.is_some()
    }

    pub fn error(&self) -> Option<&anyhow::Error> {
        self.0.as_ref().map(|v| &**v)
    }

    pub fn unwrap(self) -> Rc<anyhow::Error> {
        self.0.unwrap()
    }
}

impl PartialEq for ErrorContext {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_some() == other.0.is_some()
    }

    fn ne(&self, other: &Self) -> bool {
        self.0.is_some() != other.0.is_some()
    }
}

pub enum ErrorContextAction {
    SetError(anyhow::Error),
    ClearError,
}

impl Reducible for ErrorContext {
    type Action = ErrorContextAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Rc::new(match action {
            ErrorContextAction::SetError(err) => ErrorContext(Some(Rc::new(err))),
            ErrorContextAction::ClearError => ErrorContext::default()
        })
    }
}