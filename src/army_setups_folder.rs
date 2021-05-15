use crate::army_build::ArmyBuild;
use crate::ca_game::CaGame::Warhammer2;
use crate::ca_game::{
    get_ca_game_army_setups_folder, get_ca_game_from_folder_name, get_ca_game_subfolder, CaGame,
};
use crate::factions::{get_faction_names, Wh2Factions};
use crate::factions::{parse_faction, parse_vs_faction};
use crate::ymd_hms_dash_format::YMD_HMS_FORMAT;
use chrono::{DateTime, Utc};
use dirs;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs;
use std::io::Error;
use std::path::PathBuf;

#[cfg_attr(
    feature = "persistence",
    derive(serde::Deserialize, serde::Serialize, Clone)
)]
#[derive(Debug)]
pub struct ArmySetupsFolder {
    pub folder_string: String,
    pub folder_error: String,
    pub ca_game: CaGame,
}

impl ArmySetupsFolder {
    pub fn new(folder: &str) -> Self {
        let folder_string = folder.to_string();
        let folder_error = match validate_load_folder(folder_string.as_str()) {
            Ok(_) => String::new(),
            Err(e) => e,
        };
        let ca_game = get_ca_game_from_folder_name(folder_string.as_str());
        Self {
            folder_string,
            folder_error,
            ca_game,
        }
    }

    pub fn is_folder(&self) -> bool {
        let path = std::path::Path::new(self.folder_string.as_str());
        return path.exists() && path.is_dir();
    }

    pub fn is_owaagh_appdata(&self) -> bool {
        validate_insert_folder(
            self.folder_string.as_str(),
            &["AppData\\Roaming\\WarbossWaaghit", "army_setups"],
        )
        .is_ok()
    }

    pub fn is_load_folder(&self) -> bool {
        validate_load_folder(self.folder_string.as_str()).is_ok()
    }

    pub fn is_ca_game_folder(&self) -> bool {
        let res_sub_dir = get_ca_game_army_setups_folder(self.ca_game.clone());
        if res_sub_dir.is_err() {
            return false;
        }
        let subdir_str = res_sub_dir.unwrap().to_string_lossy().to_string();
        validate_insert_folder(
            self.folder_string.as_str(),
            &[
                "AppData\\Roaming\\The Creative Assembly",
                subdir_str.as_str(),
            ],
        )
        .is_ok()
    }

    pub fn set_load_folder_error(&mut self) {
        self.folder_error = match validate_load_folder(self.folder_string.as_str()) {
            Ok(_) => String::new(),
            Err(e) => e,
        };
    }

    pub fn set_insert_folder_error(&mut self) {
        self.folder_error = match validate_insert_folder(
            self.folder_string.as_str(),
            &["AppData\\Roaming\\The Creative Assembly", "army_setups"],
        ) {
            Ok(_) => String::new(),
            Err(e) => e,
        };
    }

    pub fn get_tmp_defaults_folder() -> PathBuf {
        PathBuf::from("tmp_defaults")
    }

    pub fn get_defaults_folder() -> PathBuf {
        PathBuf::from("owaagh_defaults")
    }
}

impl Default for ArmySetupsFolder {
    fn default() -> Self {
        let folder_string = match get_owaagh_army_setups_dir(&CaGame::Warhammer2) {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(_e) => String::new(),
        };
        ArmySetupsFolder::new(folder_string.as_str())
    }
}

pub fn validate_load_folder(folder_path: &str) -> Result<(), String> {
    let path = std::path::Path::new(folder_path);
    if !path.exists() {
        return Err("Dat path dont even exist!!".to_string());
    }
    if !path.is_dir() {
        return Err("Dats a file not a folder!!".to_string());
    }

    //make sure there are .army_setup files in the directory
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            continue;
        } else {
            if is_army_setup_file(&entry) {
                return Ok(());
            }
        }
    }

    Err("The folder got no \'.army_setup\' files".to_string())
}

pub fn load_army_builds(folder_path: &str, ca_game: &CaGame) -> Vec<ArmyBuild> {
    let mut builds = vec![];
    match validate_load_folder(folder_path) {
        Ok(_) => {}
        Err(_) => return builds,
    }

    let path = std::path::Path::new(folder_path);

    //use expect because of valid checks check
    //make sure there are .army_setup files in the directory
    for entry in fs::read_dir(path).unwrap() {
        match entry {
            Ok(entry) => {
                if entry.path().is_dir() {
                    //skip subfolder
                    continue;
                } else {
                    if is_army_setup_file(&entry) {
                        let file_string = entry.path().to_string_lossy().to_string();
                        let file_stem = entry
                            .path()
                            .file_stem()
                            .expect("load_army_builds file stem fail")
                            .to_str()
                            .expect("load_army_builds to_str fail")
                            .to_string();

                        let mut created_on: DateTime<Utc>;
                        match std::fs::metadata(entry.path()) {
                            Ok(m) => {
                                let t = m.created().unwrap_or(std::time::SystemTime::now());
                                created_on = t.into();
                            }
                            Err(e) => {
                                println!("Getting metadata err {}", e);
                                continue;
                            }
                        }

                        let mut faction = Wh2Factions::ALL;
                        let mut vs_faction = Wh2Factions::ALL;
                        let mut faction_str = String::new();
                        let mut vs_faction_str = String::new();
                        if *ca_game == CaGame::Warhammer2 {
                            faction = parse_faction(&file_stem);
                            vs_faction = parse_vs_faction(&file_stem);
                            faction_str = get_faction_names(&faction).to_string();
                            vs_faction_str = get_faction_names(&vs_faction).to_string();
                        }

                        builds.push(ArmyBuild {
                            file: entry.path(),
                            file_stem: file_stem.clone(),
                            faction: parse_faction(&file_stem),
                            funds: 12400,
                            vs_faction: parse_vs_faction(&file_stem),
                            created_on,
                            original_file: entry.path(),
                            ca_game: get_ca_game_from_folder_name(file_string.as_str()),
                            created_by: String::new(),
                            game_mod: String::new(),
                            faction_str,
                            vs_faction_str,
                            win_count: 0,
                            loss_count: 0,
                            image_files: vec![],
                            notes: String::new(),
                        });
                        //println!("{:?} {:?} {:?}", builds.last().unwrap().file_name, builds.last().unwrap().faction, builds.last().unwrap().vs_faction);
                    }
                }
            }
            Err(_e) => {
                continue;
            }
        }
    }
    builds
}

fn is_army_setup_file(file: &fs::DirEntry) -> bool {
    match file.file_type() {
        Ok(ft) => {
            if ft.is_file() {
                let path = file.path();
                let file_stem = path.file_stem().unwrap().to_str().unwrap();
                if file_stem.len() == 0 {
                    return false;
                };
                if file_stem.chars().next().unwrap() == '.' {
                    return false;
                }
                //if file_stem[0] == '.' {return false}

                match path.extension().and_then(OsStr::to_str) {
                    None => {
                        return false;
                    }
                    Some(ext) => {
                        if ext == "army_setup" {
                            return true;
                        }
                    }
                }
            }
        }
        Err(_) => {
            return false;
        }
    }
    false
}

// let user_dir = "Users";
// let setup_root = "AppData/Roaming/The Creative Assembly/Warhammer2/";
pub fn validate_insert_folder(
    folder_path: &str,
    required_path_components: &[&str],
) -> Result<(), String> {
    let path = std::path::Path::new(folder_path);
    if !path.exists() {
        return Err("Dat path dont even exist!!".to_string());
    }
    if !path.is_dir() {
        return Err("Dat path dont even exist!!".to_string());
    }
    for rpc in required_path_components {
        if !folder_path.contains(rpc) {
            let s = format!("Path is missing component \'{}\'", rpc);
            return Err(s);
        }
    }

    Ok(())
}

//this code block is failing
//folder guarenteed to exist if return ok
pub fn get_owaagh_army_setups_dir(game: &CaGame) -> Result<PathBuf, String> {
    let mut game_subdir = get_ca_game_subfolder(game);

    if let Some(mut p) = dirs::home_dir() {
        p = p.join("AppData\\Roaming\\WarbossWaaghit");
        p = p.join(game_subdir.as_str());
        p = p.join("army_setups");
        if !p.exists() {
            match std::fs::create_dir(p.clone()) {
                Ok(_) => {}
                Err(e) => {
                    return Err(format!("{}", e));
                }
            }
        }
        return Ok(p);
    }
    let err = format!("dirs::home_dir() None",);
    println!("get_owaagh_army_setups_dir err2 {}", err);

    Err(err)
}

//this code block is failing
//returns path to folder if exists
pub fn get_tmp_default_army_setups_dir(game: &CaGame) -> Result<PathBuf, String> {
    let mut game_subdir;
    match get_ca_game_army_setups_folder(game.clone()) {
        Ok(p) => {
            game_subdir = p.to_string_lossy().to_string(); //p.as_path().clone();
        }
        Err(e) => {
            return Err(e);
        }
    }

    let mut p = PathBuf::from(game_subdir.as_str());
    p = p.join("army_setups");
    if !p.exists() {
        match std::fs::create_dir(p.clone()) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!("{}", e));
            }
        }
    }
    return Ok(p);
}
