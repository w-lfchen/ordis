//! tenet weapon info fetching

use std::collections::HashMap;

use anyhow::{Context, bail};
use regex::Regex;

/// the types of bonus elements
#[derive(Debug, derive_more::Display, derive_more::FromStr)]
pub enum BonusElement {
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
pub struct BonusData {
    element: BonusElement,
    bonus: f64,
}

/// represents the state of ergo glast's shop
#[derive(Debug)]
pub struct TenetState {
    agendus: BonusData,
    exec: BonusData,
    ferrox: BonusData,
    grigori: BonusData,
    livia: BonusData,
}

pub async fn get_tenet_state() -> Result<TenetState, anyhow::Error> {
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
