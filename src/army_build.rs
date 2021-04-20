use crate::ca_game::CaGame;
use crate::factions::Wh2Factions;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

#[cfg_attr(
    feature = "persistence",
    derive(serde::Deserialize, serde::Serialize, Clone)
)]
#[derive()]
pub struct ArmyBuild {
    pub file: PathBuf,
    pub file_stem: String,
    pub faction: Wh2Factions,
    pub vs_faction: Wh2Factions,
    pub original_file: PathBuf,
    pub ca_game: CaGame,
}

impl PartialEq for ArmyBuild {
    fn eq(&self, other: &Self) -> bool {
        self.file_stem == other.file_stem
    }
}

impl Eq for ArmyBuild {}

impl Hash for ArmyBuild {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file_stem.hash(state);
    }
}

impl Default for ArmyBuild {
    fn default() -> Self {
        Self {
            file: PathBuf::new(),
            file_stem: String::new(),
            faction: Wh2Factions::UNKNOWN,
            vs_faction: Wh2Factions::UNKNOWN,
            original_file: PathBuf::new(),
            ca_game: CaGame::Warhammer2,
        }
    }
}
