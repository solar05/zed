use crate::{
    themes::{one_dark, rose_pine, rose_pine_dawn, rose_pine_moon, sandcastle},
    Theme, ThemeMetadata, ThemeSettings,
};
use anyhow::{anyhow, Result};
use gpui2::{AppContext, SharedString};
use parking_lot::RwLock;
use settings2::SettingsStore;
use std::{collections::HashMap, sync::Arc};

pub struct ThemeRegistry(Arc<RwLock<ThemeRegistryState>>);

struct ThemeRegistryState {
    themes: HashMap<SharedString, Arc<Theme>>,
    active_theme: Arc<Theme>,
}

impl ThemeRegistry {
    pub fn new(settings: &mut SettingsStore) -> Self {
        let state = Arc::new(RwLock::new(ThemeRegistryState {
            themes: HashMap::default(),
            active_theme: Arc::new(one_dark()),
        }));


        settings.observe_global({
            let state = state.clone();
            |new_settings: &ThemeSettings, _old_settings| {

            }
        });

        let mut this = Self(state);
        this.insert_themes([
            one_dark(),
            rose_pine(),
            rose_pine_dawn(),
            rose_pine_moon(),
            sandcastle(),
        ]);

        this
    }

    fn insert_themes(&mut self, themes: impl IntoIterator<Item = Theme>) {
        for theme in themes.into_iter() {
            self.0
                .write()
                .themes
                .insert(theme.metadata.name.clone(), Arc::new(theme));
        }
    }

    pub fn list_names(&self, _staff: bool) -> Vec<SharedString> {
        self.0.read().themes.keys().cloned().collect()
    }

    pub fn list(&self, _staff: bool) -> Vec<Arc<Theme>> {
        self.0
            .read()
            .themes
            .values().cloned()
            .collect()
    }

    pub fn get(&self, name: &str) -> Result<Arc<Theme>> {
        self.0
            .read()
            .themes
            .get(name)
            .ok_or_else(|| anyhow!("theme not found: {}", name))
            .cloned()
    }
}
