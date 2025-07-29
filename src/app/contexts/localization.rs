use std::{collections::HashMap, rc::Rc};
use yew::{prelude::Reducible, UseReducerHandle};

pub type LocalizationContext = UseReducerHandle<LocalizationState>;

#[derive(PartialEq)]
pub struct LocalizationState {
    code: &'static str,
    localizations: HashMap<&'static str, Localization>,
}

impl LocalizationState {
    
}

impl Default for LocalizationState {
    fn default() -> Self {
        Self {
            code: "en",
            localizations: HashMap::from([
                ("en", Localization::parse(&include_str!("../../../public/text/en.json")).expect(""))
            ]),
        }
    }
}

pub enum LocalizationAction {
    ChangeLanguage(&'static str)
}

impl Reducible for LocalizationState {
    type Action = LocalizationAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            LocalizationAction::ChangeLanguage(code) => {

            }
        }
        self
    }
}

#[derive(PartialEq)]
enum LocalizationValue {
    Text(String),
    Format(String)
}

#[derive(PartialEq)]
struct Localization(HashMap<String, LocalizationValue>);

impl Localization {
    pub fn parse(src: &str) -> Result<Self, serde_json::Error> {
        fn walk_node<'a>(map: &mut HashMap<String, LocalizationValue>, trace: &mut Vec<&'a str>, node: &'a serde_json::Map<String, serde_json::Value>) {
            for (k, v) in node {
                trace.push(k);
                match v {
                    serde_json::Value::String(str) => {
                        let key = trace.join(".");
                        _ = map.insert(key, LocalizationValue::Text(str.to_owned()));
                    },
                    serde_json::Value::Object(value) => walk_node(map, trace, value),
                    _ => {}
                }
                trace.pop();
            }
        }

        let value = serde_json::from_str::<serde_json::Value>(src)?;
        let mut map = HashMap::new();
        let mut trace = Vec::<&str>::new();
        if let Some(value) = value.as_object() {
            walk_node(&mut map, &mut trace, value);
        }
        Ok(Self(map))
    }
}