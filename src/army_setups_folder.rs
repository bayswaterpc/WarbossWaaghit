use crate::army_build::ArmyBuild;
use crate::ca_game::{get_ca_game, get_ca_game_folder, CaGame};
use crate::factions::{parse_faction, parse_vs_faction};
use directories::BaseDirs;
use dirs;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};

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
        let ca_game = get_ca_game(folder_string.as_str());
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

    pub fn is_appdata_folder(&self) -> bool {
        validate_insert_folder(
            self.folder_string.as_str(),
            &["AppData\\Roaming\\OWAAGH", "army_setups"],
        )
        .is_ok()
    }

    pub fn is_load_folder(&self) -> bool {
        validate_load_folder(self.folder_string.as_str()).is_ok()
    }

    pub fn is_insert_folder(&self) -> bool {
        validate_insert_folder(
            self.folder_string.as_str(),
            &["AppData\\Roaming\\The Creative Assembly", "army_setups"],
        )
        .is_ok()
    }

    pub fn set_load_folder_error(&mut self) {
        self.folder_error = match validate_load_folder(self.folder_string.as_str()) {
            Ok(_) => String::new(),
            Err(e) => e,
        };
        println!("set_folder_error {}", self.folder_error);
    }

    pub fn set_insert_folder_error(&mut self) {
        self.folder_error = match validate_insert_folder(
            self.folder_string.as_str(),
            &["AppData\\Roaming\\The Creative Assembly", "army_setups"],
        ) {
            Ok(_) => String::new(),
            Err(e) => e,
        };
        println!("set_folder_error {}", self.folder_error);
    }
}

impl Default for ArmySetupsFolder {
    fn default() -> Self {
        let folder_string = match get_owaagh_army_setups_dir("Warhammer2") {
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

pub fn load_army_builds(folder_path: &str) -> Vec<ArmyBuild> {
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

                        builds.push(ArmyBuild {
                            file: entry.path(),
                            file_stem: file_stem.clone(),
                            faction: parse_faction(&file_stem),
                            vs_faction: parse_vs_faction(&file_stem),
                            original_file: entry.path(),
                            ca_game: get_ca_game(file_string.as_str()),
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
//returns path to folder if exists
pub fn get_owaagh_army_setups_dir(game: &str) -> Result<PathBuf, String> {
    if let Some(mut p) = dirs::home_dir() {
        p = p.join("AppData\\Roaming\\OWAAGH");
        p = p.join(game);
        p = p.join("army_setups");
        if p.exists() {
            println!(
                "get_owaagh_army_setups_dir p{}",
                p.to_string_lossy().to_string()
            );
            return Ok(p);
        } else {
            let err = format!(
                "get_user_default_army_setups_folder_dirs dne {}",
                p.to_string_lossy()
            );
            println!("get_owaagh_army_setups_dir err1 {}", err);

            return Err(err);
        }
    }
    let err = format!("dirs::home_dir() None",);
    println!("get_owaagh_army_setups_dir err2 {}", err);

    Err(err)
}

pub fn get_owaagh_merge_conflict_dir() -> Result<PathBuf, String> {
    if let Some(mut p) = dirs::home_dir() {
        p = p.join("AppData\\Roaming\\OWAAGH");
        p = p.join("merge_conflicts");
        p = p.join("army_setups");
        if p.exists() {
            println!(
                "get_owaagh_army_setups_dir p{}",
                p.to_string_lossy().to_string()
            );
            return Ok(p);
        } else {
            return match fs::create_dir(p.clone()) {
                Ok(_) => Ok(p),
                Err(e) => Err(e.to_string()),
            };
        }
    }
    let err = format!("dirs::home_dir() None",);
    println!("get_owaagh_army_setups_dir err2 {}", err);

    Err(err)
}
