pub struct WithWarning<S, W> {
    value: S,
    warning: Option<W>,
}

impl<S, W> WithWarning<S, W> {
    pub fn new(value: S) -> Self {
        Self {
            value,
            warning: None,
        }
    }

    pub fn with_warning(value: S, warning: W) -> Self {
        Self {
            value,
            warning: Some(warning),
        }
    }

    pub fn unwrap(self) -> S {
        self.value
    }

    pub fn has_warning(&self) -> bool {
        self.warning.is_some()
    }

    pub fn unwrap_warning(self) -> W {
        self.warning.unwrap()
    }

    pub fn as_ref(&self) -> WithWarning<&S, &W> {
        WithWarning {
            value: &self.value,
            warning: self.warning.as_ref(),
        }
    }

    pub fn map<T, F: FnOnce(S) -> T>(self, f: F) -> WithWarning<T, W> {
        WithWarning {
            value: f(self.value),
            warning: self.warning,
        }
    }

    pub fn map_warning<U, F: FnOnce(W) -> U>(self, f: F) -> WithWarning<S, U> {
        WithWarning {
            value: self.value,
            warning: self.warning.map(f), 
        }
    }
}