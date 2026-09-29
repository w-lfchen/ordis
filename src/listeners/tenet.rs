//! tenet weapon info fetching

use anyhow::bail;
use regex::Regex;

/// represents a tenet weapon that can be bought from ergo glast
/// TODO: adjust field types
#[derive(Debug)]
pub struct TenetWeapon {
    weapon: String,
    element: String,
    bonus: f64,
}

pub async fn get_tenet_weapons() -> Result<Vec<TenetWeapon>, anyhow::Error> {
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
    let text = text
        .split_whitespace()
        .fold(String::with_capacity(text.len()), |x, y| x + y);

    weapon_capture
        .captures_iter(&text)
        .map(|c| {
            // TODO: extract can panic, which might be a bad idea
            let (_full_match, [weapon, element, bonus]) = c.extract();
            Ok(TenetWeapon {
                weapon: weapon.into(),
                element: element.into(),
                bonus: bonus.parse()?,
            })
        })
        .collect()
}
