use enum_iterator::IntoEnumIterator;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[cfg_attr(
    feature = "persistence",
    derive(
        serde::Deserialize,
        serde::Serialize,
        Debug,
        Clone,
        PartialEq,
        IntoEnumIterator
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
    Unknown,
}

pub fn is_game_folder(folder_str: &str, ca_game: CaGame) -> bool {
    if ca_game == CaGame::Unknown {
        return false;
    }
    let p = PathBuf::from(folder_str);
    let game_subdir = get_ca_game_subfolder(ca_game.clone());
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
    // for e in CaGame::into_enum_iterator() {
    //     let subdir_str = get_ca_game_subfolder(e);
    //     m.insert(subdir_str, e)
    // }
    m
}

pub fn get_ca_game(folder_str: &str) -> CaGame {
    let subdir_str_ca_game_map = get_subdir_str_ca_game_map();
    let p = PathBuf::from(folder_str);
    let ca_game = CaGame::Unknown;
    let game_subdir = get_ca_game_subfolder(ca_game);
    for d in p.iter() {
        if subdir_str_ca_game_map.contains_key(game_subdir.as_str()) {
            return subdir_str_ca_game_map
                .get(game_subdir.as_str())
                .unwrap()
                .clone();
        }
    }

    CaGame::Unknown
}

//this code block is failing
//returns path to folder if exists
fn get_game_default_army_setups_dir(
    game: &str,
    army_setups_subdir: &str,
) -> Result<PathBuf, String> {
    if let Some(mut p) = dirs::home_dir() {
        p = p.join("AppData\\Roaming\\The Creative Assembly");
        p = p.join(game);
        p = p.join(army_setups_subdir);
        if p.exists() {
            return Ok(p);
        } else {
            let err = format!(
                "get_user_default_army_setups_folder_dirs dne {}",
                p.to_string_lossy()
            );
            return Err(err);
        }
    }
    let err = format!("dirs::home_dir() None",);
    Err(err)
}

pub fn get_ca_game_subfolder(ca_game: CaGame) -> String {
    let game_subdir = match ca_game {
        CaGame::Attila => "don't know",
        CaGame::Empire => "Empire",
        CaGame::Medieval2 => "don't know",
        CaGame::Napoleon => "Napolean",
        CaGame::Rome => "don't know",
        CaGame::Rome2 => "Rome2",
        CaGame::RomeRemastered => "",
        CaGame::Shogun2 => "Shogun2",
        CaGame::ThreeKingdoms => "ThreeKingdoms",
        CaGame::ThronesOfBritannia => "don't know",
        CaGame::Warhammer => "Warhammer",
        CaGame::Warhammer2 => "Warhammer2",
        CaGame::Unknown => "Unknown",
    };
    String::from(game_subdir)
}

pub fn get_ca_game_folder(ca_game: CaGame) -> Result<PathBuf, String> {
    if ca_game == CaGame::Medieval2 {
        return Err(String::from("Medieval2 not yet supported"));
    }
    if ca_game == CaGame::Unknown {
        return Err(String::from("Can't folder get for unknown game"));
    }

    let army_setups_subdir = match ca_game.clone() {
        CaGame::Empire => "battle_preferences",
        CaGame::Medieval2 => "todo_support", //this goes into steamapps and is harder to find ex: D:\Personal\SteamLibrary\steamapps\common
        _ => "army_setups",
    };

    let game_subdir = get_ca_game_subfolder(ca_game.clone());

    get_game_default_army_setups_dir(game_subdir.as_str(), army_setups_subdir)
}
