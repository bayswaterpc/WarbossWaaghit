use crate::factions::Faction;
use std::path::PathBuf;

#[cfg_attr(
    feature = "persistence",
    derive(serde::Deserialize, serde::Serialize, Clone)
)]
pub struct ArmyBuild {
    pub file: PathBuf,
    pub file_stem: String,
    pub faction: Faction,
    pub vs_faction: Faction,
}

impl Default for ArmyBuild {
    fn default() -> Self {
        Self {
            file: PathBuf::new(),
            file_stem: String::new(),
            faction: Faction::UNKNOWN,
            vs_faction: Faction::UNKNOWN,
        }
    }
}
