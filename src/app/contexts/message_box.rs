use std::{cell::RefCell, rc::Rc};
use yew::{Callback, Reducible, UseReducerHandle};

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Info(String),
    Warning(String),
    Error(String),
    Confirm {
        question: String,
        callback: Callback<()>,
    }
}

pub type MessageBoxContext = UseReducerHandle<MessageBoxState>;

#[derive(Clone, PartialEq)]
pub struct MessageBoxState {
    boxes: Rc<RefCell<Vec<Message>>>
}

impl MessageBoxState {
    pub fn has_messages(&self) -> bool {
        !self.boxes.borrow().is_empty()
    }

    pub fn get_top(&self) -> Option<Message> {
        self.boxes.borrow().last().cloned()
    }
}

impl Default for MessageBoxState {
    fn default() -> Self {
        Self {
            boxes: Rc::new(RefCell::new(Vec::<Message>::new())),
        }
    }
}

pub enum MessageBoxAction {
    Push(Message),
    Pop,
}

impl Reducible for MessageBoxState {
    type Action = MessageBoxAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            MessageBoxAction::Push(message) => self.boxes.borrow_mut().push(message),
            MessageBoxAction::Pop => {
                _ = self.boxes.borrow_mut().pop()
            }
        }
        self
    }
}