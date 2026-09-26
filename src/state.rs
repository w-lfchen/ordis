//! abstraction over dynamic bot state

use std::{
    collections::HashSet,
    sync::{Arc, LazyLock, Mutex, OnceLock},
};

use poise::serenity_prelude::{ChannelId, Http};

static HTTP: OnceLock<Arc<Http>> = OnceLock::new();
static CHANNELS: LazyLock<Mutex<HashSet<ChannelId>>> = LazyLock::new(Mutex::default);

#[derive(Debug)]
pub struct BotData;

/// sets the http
pub fn set_http(http: Arc<Http>) -> Result<(), Arc<Http>> {
    HTTP.set(http)
}

/// gets the http
pub fn get_http() -> &'static Http {
    HTTP.get().unwrap().as_ref()
}

/// toggles a given channel, returning whether it is now enabed or not
pub fn toggle_channel(id: ChannelId) -> bool {
    let mut channels = CHANNELS.lock().unwrap();
    let contains = channels.contains(&id);
    if contains {
        channels.remove(&id);
    } else {
        channels.insert(id);
    }
    !contains
}
