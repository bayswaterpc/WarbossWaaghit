use crate::ca_game::CaGame;
use crate::factions::Wh2Factions;
use crate::ymd_hms_dash_format;
use crate::ymd_hms_dash_format::YMD_HMS_FORMAT;
use chrono::Utc;
use std::path::PathBuf;

#[cfg_attr(
    feature = "persistence",
    derive(serde::Deserialize, serde::Serialize, Clone)
)]
#[derive()]
pub struct AdvancedSearch {
    pub name: String, //also acts as display name

    #[serde(with = "ymd_hms_dash_format")]
    pub created_on_search: chrono::DateTime<Utc>,
    pub check_after_created: bool,

    pub faction: Wh2Factions,
    pub vs_faction: Wh2Factions,

    pub ca_game: CaGame,
    pub game_mod: String,
    pub created_by: String,

    pub faction_str: String,    // for display, nonwarhammer2 games, & mods
    pub vs_faction_str: String, // for display, nonwarhammer2 games, & mods

    pub funds: u32,
}

impl Default for AdvancedSearch {
    fn default() -> Self {
        Self {
            name: String::new(),
            faction: Wh2Factions::UNKNOWN,
            vs_faction: Wh2Factions::UNKNOWN,
            created_on_search: std::time::SystemTime::now().into(),
            check_after_created: true,

            ca_game: CaGame::Warhammer2,
            game_mod: String::new(),
            created_by: String::new(),
            faction_str: String::new(),
            vs_faction_str: String::new(),

            funds: 12400,
        }
    }
}
