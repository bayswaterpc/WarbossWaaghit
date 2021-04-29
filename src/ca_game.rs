use crate::army_setups_manager::ArmySetupsManager;
use crate::central_panel_state::AppState;
use eframe::egui;
use eframe::egui::{Color32, Ui};
use enum_iterator::IntoEnumIterator;
use std::collections::HashMap;
use std::path::PathBuf;

#[cfg_attr(
    feature = "persistence",
    derive(
        serde::Deserialize,
        serde::Serialize,
        Debug,
        Clone,
        IntoEnumIterator,
        PartialEq,
        Eq,
        Hash
    )
)]
pub enum CaGame {
    Attila,
    Empire,
    Medieval2,
    Napoleon,
    Rome,
    Rome2,
    RomeRemastered,
    Shogun2,
    ThreeKingdoms,
    ThronesOfBritannia,
    Warhammer,
    Warhammer2,
}

pub fn is_ca_game_folder(folder_str: &str, ca_game: &CaGame) -> bool {
    let p = PathBuf::from(folder_str);
    let game_subdir = get_ca_game_subfolder(&ca_game);
    for d in p.iter() {
        let subdir_str = d.to_string_lossy().to_string();
        if subdir_str == game_subdir {
            return true;
        }
    }
    return false;
}

fn get_subdir_str_ca_game_map() -> HashMap<String, CaGame> {
    let mut m = HashMap::new();
    for e in CaGame::into_enum_iter() {
        let subdir_str = get_ca_game_subfolder(&e);
        m.insert(subdir_str, e);
    }
    m
}

pub fn get_ca_game_from_folder_name(folder_str: &str) -> CaGame {
    let subdir_str_ca_game_map = get_subdir_str_ca_game_map();
    let p = PathBuf::from(folder_str);
    let ca_game = CaGame::Warhammer2;
    let game_subdir = get_ca_game_subfolder(&ca_game);
    for _d in p.iter() {
        if subdir_str_ca_game_map.contains_key(game_subdir.as_str()) {
            return subdir_str_ca_game_map
                .get(game_subdir.as_str())
                .unwrap()
                .clone();
        }
    }

    CaGame::Warhammer2
}

pub fn get_ca_game_subfolder(ca_game: &CaGame) -> String {
    let game_subdir = match ca_game {
        CaGame::Attila => "todo",
        CaGame::Empire => "Empire",
        CaGame::Medieval2 => "todo",
        CaGame::Napoleon => "Napolean",
        CaGame::Rome => "todo",
        CaGame::Rome2 => "Rome2",
        CaGame::RomeRemastered => "todo",
        CaGame::Shogun2 => "Shogun2",
        CaGame::ThreeKingdoms => "ThreeKingdoms",
        CaGame::ThronesOfBritannia => "todo",
        CaGame::Warhammer => "Warhammer",
        CaGame::Warhammer2 => "Warhammer2",
    };
    String::from(game_subdir)
}

pub fn get_ca_game_title(ca_game: &CaGame) -> String {
    let game_subdir = match ca_game {
        CaGame::Attila => "Attila",
        CaGame::Empire => "Empire",
        CaGame::Medieval2 => "Medieval 2",
        CaGame::Napoleon => "Napolean",
        CaGame::Rome => "Rome",
        CaGame::Rome2 => "Rome 2",
        CaGame::RomeRemastered => "Rome Remastered",
        CaGame::Shogun2 => "Shogun 2",
        CaGame::ThreeKingdoms => "Three Kingdoms",
        CaGame::ThronesOfBritannia => "Thrones of Britannia",
        CaGame::Warhammer => "Warhammer",
        CaGame::Warhammer2 => "Warhammer 2",
    };
    String::from(game_subdir)
}

//
pub fn get_ca_game_army_setups_folder(ca_game: CaGame) -> Result<PathBuf, String> {
    let army_setups_subdir = match ca_game.clone() {
        CaGame::Empire => "battle_preferences",
        _ => "army_setups",
    };

    let game_subdir = get_ca_game_subfolder(&ca_game);

    if let Some(mut p) = dirs::home_dir() {
        p = p.join("AppData\\Roaming\\The Creative Assembly");
        p = p.join(game_subdir);
        p = p.join(army_setups_subdir);
        return Ok(p);
    }
    let err = format!("dirs::home_dir() None",);
    Err(err)
}

pub fn get_ca_game_army_setup_ext(ca_game: CaGame) -> String {
    match ca_game {
        CaGame::Empire => String::from("battle_preferences"),
        _ => String::from("army_setup"),
    }
}

#[cfg_attr(
    feature = "persistence",
    derive(serde::Deserialize, serde::Serialize, Clone)
)]
#[derive(Debug)]
pub struct GameSelector {
    pub ca_game: CaGame,
}

impl Default for GameSelector {
    fn default() -> Self {
        GameSelector {
            ca_game: CaGame::Warhammer2,
        }
    }
}

impl GameSelector {
    pub fn central_panel_ui(
        &mut self,
        ui: &mut Ui,
        army_setups_manager: &mut ArmySetupsManager,
        app_state: &mut AppState,
    ) {
        for ca_game in CaGame::into_enum_iter() {
            if ui
                .selectable_label(ca_game == self.ca_game, get_ca_game_title(&ca_game))
                .clicked()
            {
                self.ca_game = ca_game;

                //TODO plugin bevy and use ecs
                army_setups_manager.set_selected_game(self.ca_game.clone());
                army_setups_manager.selected_game_update();

                app_state.ca_game = self.ca_game.clone();
            }
        }
    }
}
