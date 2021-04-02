use crate::army_build::ArmyBuild;
use crate::factions::{parse_faction, parse_vs_faction};
use directories::BaseDirs;
use dirs;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

#[cfg_attr(
    feature = "persistence",
    derive(serde::Deserialize, serde::Serialize, Clone)
)]
pub struct ArmySetupsFolder {
    pub folder_string: String,
}

impl ArmySetupsFolder {
    pub fn is_folder(&self) -> bool {
        let path = std::path::Path::new(self.folder_string.as_str());
        return path.exists() && path.is_dir();
    }

    pub fn is_load_folder(&self) -> bool {
        validate_load_folder(self.folder_string.as_str()).is_ok()
    }

    pub fn is_insert_folder(&self) -> bool {
        let valid_components = ["The Creative Assembly", "army_setups"];
        validate_insert_folder(self.folder_string.as_str(), &valid_components).is_ok()
    }
}

impl Default for ArmySetupsFolder {
    fn default() -> Self {
        println!("default army setups");

        Self {
            folder_string: "WarbossWaaghitSetups\\Warhammer2".to_string(),
        }
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

fn _get_path_subdirs(dir: &Path) -> Vec<String> {
    let mut subdirs = vec![];
    for component in dir.components() {
        let c = component.as_os_str().to_string_lossy().to_string();
        subdirs.push(c);
    }
    subdirs
}

//TODO figure out how to do this with globwalk
// searches for folder in a directory with handling for * any wildcard in path component
fn _search_for_folder(
    dir: &Path,
    folder_depth: usize,
    path_components: &Vec<String>,
) -> Result<String, (usize, String)> {
    if folder_depth >= path_components.len() {
        return Err((folder_depth, "Gone too far".to_string()));
    } else if folder_depth == path_components.len() - 1 {
        let f = dir
            .file_name()
            .unwrap_or(OsStr::new(""))
            .to_string_lossy()
            .to_string();
        if f == path_components[folder_depth] || path_components[folder_depth] == "*" {
            return Ok(dir.to_string_lossy().to_string());
        }
    }

    let mut max_err_depth = 0 as usize;
    let mut err_message = String::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir).expect("read_dir fail") {
            let entry = entry.expect("DirEntry");
            let path = entry.path();
            if path.is_dir() {
                match path.file_name() {
                    Some(dir_name) => {
                        let dir_string = dir_name.to_string_lossy().to_string();
                        //If match or wildcard Handle wildcard
                        if dir_string == path_components[folder_depth]
                            || path_components[folder_depth] == "*"
                        {
                            let res = _search_for_folder(
                                path.as_path(),
                                folder_depth + 1,
                                path_components,
                            );
                            match res {
                                Ok(file) => {
                                    return Ok(file);
                                }
                                Err((d, s)) => {
                                    if max_err_depth < d {
                                        max_err_depth = d;
                                        err_message = s;
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        return Err((folder_depth, "none path.file_name()".to_string()));
                    }
                }
            } else {
                continue;
            }
        }
    }
    Err((max_err_depth, err_message))
}

//this code block is failing
//returns path to folder if exists
pub fn get_user_default_load_army_setups_folder_dirs(game: &str) -> Result<PathBuf, String> {
    if let Some(mut p) = dirs::home_dir() {
        p = p.join("AppData\\Roaming\\OWAAGH");
        p = p.join(game);
        p = p.join("army_setups");
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

//this code block is failing
//returns path to folder if exists
pub fn get_user_default_army_setups_folder_dirs(game: &str) -> Result<PathBuf, String> {
    if let Some(mut p) = dirs::home_dir() {
        p = p.join("AppData\\Roaming\\The Creative Assembly");
        p = p.join(game);
        p = p.join("army_setups");
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

//this code block is failing
//returns path to folder if exists
fn _get_user_default_army_setups_folder_directories(game: &str) -> Result<PathBuf, String> {
    if let Some(base_dirs) = BaseDirs::new() {
        let mut p = base_dirs.data_dir().to_path_buf();
        p = p.join("The Creative Assembly");
        p = p.join(game);
        p = p.join("army_setups");

        if p.exists() {
            return Ok(p);
        } else {
            let err = format!(
                " get_user_default_army_setups_folder_directories dne {}",
                p.to_string_lossy()
            );
            return Err(err);
        }
    }
    let err = format!("BaseDirs::new() None",);
    Err(err)
}
