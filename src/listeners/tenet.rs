//! tenet weapon info fetching

use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

use anyhow::{Context, bail};
use poise::serenity_prelude::{Color, CreateEmbed, CreateEmbedFooter, CreateMessage};
use regex::Regex;
use tokio::time::{Instant, sleep_until};

use crate::state;

/// the types of bonus elements
#[derive(Debug, derive_more::Display, derive_more::FromStr)]
enum BonusElement {
    Impact,
    Heat,
    Cold,
    Electricity,
    Toxin,
    Magnetic,
    Radiation,
}

/// represents a weapon's bonus element and its bonus percentage
#[derive(Debug)]
struct BonusData {
    element: BonusElement,
    bonus: f64,
}

/// represents the state of ergo glast's shop
#[derive(Debug)]
struct TenetState {
    agendus: BonusData,
    exec: BonusData,
    ferrox: BonusData,
    grigori: BonusData,
    livia: BonusData,
}

async fn get_tenet_state() -> Result<TenetState, anyhow::Error> {
    const PKG_NAME: &str = env!("CARGO_PKG_NAME");
    const TENET_WIKI_URL: &str = "https://wiki.warframe.com/api.php?action=query&prop=revisions&rvprop=content&rvslots=main&titles=Tenet_Weapons&format=json";

    // a table row will look something like this:
    // <td>{{Weapon|TenetGrigori}}</td><td>{{D|Electricity}}</td><td>{{ValenceBonusPercentageColor|38.6}}</td>
    let weapon_capture = Regex::new(
        r"\{\{Weapon\|Tenet([[:alpha:]]*)\}\}</td><td>\{\{D\|([[:alpha:]]*)\}\}</td><td>\{\{ValenceBonusPercentageColor\|(\d{2}\.?\d?)\}\}",
    )?;

    // fetch wiki
    let client = reqwest::Client::builder().user_agent(PKG_NAME).build()?;
    let response = client.get(TENET_WIKI_URL).send().await?;
    let status = response.status();
    if !status.is_success() {
        bail!("warframe wiki responded with {status}");
    }

    // capture weapons
    let text = response.text().await?;
    // first remove all whitespace from the text to make regex capturing easier
    // this operates on the entire wiki article, might be worth improving somehow
    let text: String = text.split_whitespace().collect();

    let mut map = weapon_capture
        .captures_iter(&text)
        .map(|c| {
            // TODO: extract can panic, which might be a bad idea
            let (_full_match, [weapon, element, bonus]) = c.extract();
            Ok((
                weapon,
                BonusData {
                    element: element.parse()?,
                    bonus: bonus.parse()?,
                },
            ))
        })
        .collect::<Result<HashMap<_, _>, anyhow::Error>>()?;

    Ok(TenetState {
        agendus: map.remove("Agendus").context("no agendus")?,
        exec: map.remove("Exec").context("no exec")?,
        ferrox: map.remove("Ferrox").context("no ferrox")?,
        grigori: map.remove("Grigori").context("no grigori")?,
        livia: map.remove("Livia").context("no livia")?,
    })
}

pub async fn tenet_embed() -> Result<CreateEmbed, anyhow::Error> {
    let state = get_tenet_state().await?;
    Ok(CreateEmbed::new()
        .color(Color::from_rgb(92, 171, 250))
        .title("Current Tenet Rotation")
        .url("https://wiki.warframe.com/w/Tenet_Weapons")
        .description(format!(
            "\
                Agendus: {}% {}\n\
                Exec: {}% {}\n\
                Ferrox: {}% {}\n\
                Grigori: {}% {}\n\
                Livia: {}% {}\n\
            ",
            state.agendus.bonus,
            state.agendus.element,
            state.exec.bonus,
            state.exec.element,
            state.ferrox.bonus,
            state.ferrox.element,
            state.grigori.bonus,
            state.grigori.element,
            state.livia.bonus,
            state.livia.element,
        ))
        .footer(CreateEmbedFooter::new("These values are player-reported.")))
}

// TODO: improve, this is a rough draft of an idea
pub async fn tenet_listener() -> Result<(), anyhow::Error> {
    // wiki page starts counter at 3/12/2015
    const START_UNIX_TIMESTAMP: u64 = 1_449_100_800;
    // wiki has 4d looptime
    const INTERVAL: Duration = Duration::from_hours(4 * 24);

    let now = SystemTime::now();
    let start = SystemTime::UNIX_EPOCH + Duration::from_secs(START_UNIX_TIMESTAMP);

    // how many times the interval has passed, rounded down
    let passed_intervals = now
        .duration_since(start)
        .context("current time is earlier than december 2015")?
        .as_secs()
        / INTERVAL.as_secs();
    // next time glast rotates shop
    let next_rotation = start + (INTERVAL * u32::try_from(passed_intervals + 1)?);
    // duration until next glast rotation
    let next_rotation = next_rotation
        .duration_since(now)
        .context("next rotation is before now")?;

    // add an initial 2 hours in hopes of the wiki being updated after 2 hours
    let mut instant = Instant::now() + next_rotation + Duration::from_hours(2);
    loop {
        sleep_until(instant).await;
        let http = state::get_http();
        for ch in state::get_channels().await.iter() {
            ch.send_message(http, CreateMessage::new().add_embed(tenet_embed().await?))
                .await?;
        }
        // calculate time until next tenet rotation
        instant += INTERVAL;
    }
}
