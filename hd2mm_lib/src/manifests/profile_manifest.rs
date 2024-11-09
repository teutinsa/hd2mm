use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use trace_fn::trace_fn;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModState {
    enabled: bool,
    options: Vec<bool>,
    sub_options: Vec<usize>
}

impl ModState {
    #[trace_fn]
    pub fn new(enabled: bool, options: Vec<bool>, sub_options: Vec<usize>) -> Self {
        Self {
            enabled,
            options,
            sub_options
        }
    }

    pub fn options_mut(&mut self) -> &mut [bool] {
        &mut self.options
    }

    pub fn options(&self) -> &[bool] {
        &self.options
    }

    pub fn sub_options_mut(&mut self) -> &mut [usize] {
        &mut self.sub_options
    }

    pub fn sub_options(&self) -> &[usize] {
        &self.sub_options
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProfileManifest {
    name: String,
    mods: HashMap<Uuid, ModState>
}

impl ProfileManifest {
    #[trace_fn]
    pub fn new(name: String, mods: HashMap<Uuid, ModState>) -> Self {
        Self {
            name,
            mods
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn mods(&self) -> std::collections::hash_map::Keys<'_, uuid::Uuid, ModState> {
        self.mods.keys()
    }

    #[trace_fn]
    pub fn add_mod(&mut self, guid: &Uuid, state: ModState) -> bool {
        if self.mods.contains_key(guid) {
            false
        } else {
            self.mods.insert(guid.clone(), state);
            true
        }
    }

    #[trace_fn]
    pub fn remove_mod(&mut self, guid: &Uuid) {
        self.mods.remove(guid);
    }

    pub fn get_state(&self, guid: &Uuid) -> Option<&ModState> {
        self.mods.get(guid)
    }
}
