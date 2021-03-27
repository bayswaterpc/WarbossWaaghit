use crate::factions::Wh2Factions;
use std::path::PathBuf;

#[cfg_attr(
    feature = "persistence",
    derive(serde::Deserialize, serde::Serialize, Clone)
)]
pub struct ArmyBuild {
    pub file: PathBuf,
    pub file_stem: String,
    pub faction: Wh2Factions,
    pub vs_faction: Wh2Factions,
}

impl Default for ArmyBuild {
    fn default() -> Self {
        Self {
            file: PathBuf::new(),
            file_stem: String::new(),
            faction: Wh2Factions::UNKNOWN,
            vs_faction: Wh2Factions::UNKNOWN,
        }
    }
}
