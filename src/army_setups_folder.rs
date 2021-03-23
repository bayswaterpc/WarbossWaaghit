use crate::army_build::ArmyBuild;
use crate::factions::{parse_faction, parse_vs_faction};
use std::ffi::OsStr;
use std::fs::DirEntry;
use std::path::Path;
use std::{fs, io};

#[cfg_attr(
    feature = "persistence",
    derive(serde::Deserialize, serde::Serialize, Clone)
)]
pub struct ArmySetupsFolder {
    pub folder_str: String,
    pub valid_folder: bool,
    pub show: bool,
}

impl Default for ArmySetupsFolder {
    fn default() -> Self {
        Self {
            folder_str: "C:\\Users\\DaBiggestBoss\\AppData\\Roaming\\The Creative Assembly\\Warhammer2\\army_setups".to_string(),
            valid_folder: false,
            show: true
        }
    }
}

pub fn valid_load_folder(folder_path: &str) -> Result<(), String> {
    println!("into validate folder");
    let path = std::path::Path::new(folder_path);
    if !path.exists() {
        return Err("Dat path dont even exist!!".to_string());
    }
    if !path.is_dir() {
        return Err("Dat path dont even exist!!".to_string());
    }

    //make sure there are .army_setup files in the directory
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            println!("skip sub folder");
            continue;
        } else {
            if is_army_setup_file(&entry) {
                return Ok(());
            }
        }
    }

    Err("The folder got no \'.army_setup\' files".to_string())
}

pub fn valid_insert_folder(folder_path: &str) -> Result<(), String> {
    println!("into validate folder");
    let path = std::path::Path::new(folder_path);
    if !path.exists() {
        return Err("Dat path dont even exist!!".to_string());
    }
    if !path.is_dir() {
        return Err("Dat path dont even exist!!".to_string());
    }

    let setup_root = "AppData/Roaming/The Creative Assembly/Warhammer2/";
    if !folder_path.contains(setup_root) || !folder_path.contains("army_setups") {
        let s = format!("Da path needs dis \'{}\'", setup_root);
        return Err(s);
    }

    Ok(())
}

pub fn load_army_builds(folder_path: &str) -> Vec<ArmyBuild> {
    let mut builds = vec![];
    match valid_load_folder(folder_path) {
        Ok(_) => {}
        Err(_) => return builds,
    }

    let path = std::path::Path::new(folder_path);

    //make sure there are .army_setup files in the directory
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            //skip subfolder
            continue;
        } else {
            if is_army_setup_file(&entry) {
                let file_stem = entry
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
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
    builds
}

fn is_army_setup_file(file: &DirEntry) -> bool {
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

// the use for this is a directory search
fn _visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry) -> bool) -> io::Result<bool> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                _visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(true)
}
