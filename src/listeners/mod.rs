mod tenet;

use std::sync::Arc;

use warframe::worldstate::{Change, Client, queryable::Fissure};

use crate::state;

#[derive(Debug, Clone)]
struct WfClientState;

pub async fn register_listeners() -> Result<(), anyhow::Error> {
    let _ = tokio::spawn(tenet::tenet_listener());
    let client = Arc::new(Client::default());
    // TOOD: why is this future not send when using the method without state?
    let _ = tokio::spawn(async move {
        client
            .clone()
            .call_on_nested_update_with_state(on_fissure_update, WfClientState)
            .await
    });
    Ok(())
}

async fn on_fissure_update(_state: WfClientState, fissure: &Fissure, change: Change) {
    let text = format!("Fissure {change:?}: {fissure:?}");
    let http = state::get_http();
    for ch in state::get_channels().await.iter() {
        // TODO: add error handling within the constraint that we can't return result
        let _ = ch.say(http, &text).await;
    }
}
