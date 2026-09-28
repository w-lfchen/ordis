//! abstraction over dynamic bot state

use std::{
    collections::HashSet,
    sync::{Arc, LazyLock, OnceLock},
};

use poise::serenity_prelude::{ChannelId, Http};
use tokio::sync::{RwLock, RwLockReadGuard};

static HTTP: OnceLock<Arc<Http>> = OnceLock::new();
static CHANNELS: LazyLock<RwLock<HashSet<ChannelId>>> = LazyLock::new(RwLock::default);

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
pub async fn toggle_channel(id: ChannelId) -> bool {
    let mut channels = CHANNELS.write().await;
    let contains = channels.contains(&id);
    if contains {
        channels.remove(&id);
    } else {
        channels.insert(id);
    }
    !contains
}

pub async fn get_channels<'a>() -> RwLockReadGuard<'a, HashSet<ChannelId>> {
    CHANNELS.read().await
}
