use std::{cell::RefCell, rc::Rc};
use yew::Callback;

#[derive(Clone, PartialEq)]
pub enum Message {
    Info(String),
    Warning(String),
    Error(String),
    Confirm {
        question: String,
        callback: Callback<()>,
    }
}

#[derive(Clone, PartialEq)]
pub struct MessageBoxContext {
    boxes: Rc<RefCell<Vec<Message>>>
}

impl MessageBoxContext {
    pub fn has_messages(&self) -> bool {
        !self.boxes.borrow().is_empty()
    }

    pub fn get_top(&self) -> Option<Message> {
        self.boxes.borrow().last().cloned()
    }

    pub fn push(&mut self, message: Message) {
        self.boxes.borrow_mut().push(message);
    }

    pub fn pop(&mut self) {
        _ = self.boxes.borrow_mut().pop();
    }
}

impl Default for MessageBoxContext {
    fn default() -> Self {
        Self {
            boxes: Rc::new(RefCell::new(Vec::<Message>::new())),
        }
    }
}