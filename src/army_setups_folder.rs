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

pub fn load_army_builds(folder_path: &str) -> Vec<ArmyBuild> {
    // let star_path = r#"C:Users\*\AppData\Roaming\The Creative Assembly\*\army_setups"#;
    // let v_subdirs = get_path_subdirs(Path::new(star_path));
    // println!("b_subdirs {:?}", v_subdirs);
    //
    // let star_path = r#"C:\Users\jmw99\AppData\Roaming\The Creative Assembly\Warhammer2\army_setups\WE vs TK.army_setup"#;
    // let v_subdirs = get_path_subdirs(Path::new(star_path));
    // println!("c_subdirs {:?}", v_subdirs);

    match get_user_default_army_setups_folder_dirs("Warhammer2") {
        Ok(p) => {
            println!("got it! {}", p.to_string_lossy())
        }
        Err(e) => {
            println!("{}", e)
        }
    }

    match get_user_default_army_setups_folder_directories("Warhammer2") {
        Ok(p) => {
            println!("got it! {}", p.to_string_lossy())
        }
        Err(e) => {
            println!("{}", e)
        }
    }

    // fn get_path_subdirs(dir: &Path) -> Vec<String> {
    //     let mut subdirs = vec![];
    //     for component in dir.components() {
    //         let c = component.as_os_str().to_string_lossy().to_string();
    //         subdirs.push(c);
    //     }
    //     subdirs
    // }
    //
    // // searches for folder in a directory with handling for * any wildcard in path component
    // fn search_for_folder(

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
pub fn is_valid_insert_folder(
    folder_path: &str,
    required_path_components: &Vec<&str>,
) -> Result<(), String> {
    println!("into validate folder");
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

fn get_path_subdirs(dir: &Path) -> Vec<String> {
    let mut subdirs = vec![];
    for component in dir.components() {
        let c = component.as_os_str().to_string_lossy().to_string();
        subdirs.push(c);
    }
    subdirs
}

// searches for folder in a directory with handling for * any wildcard in path component
fn search_for_folder(
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
                            let res = search_for_folder(
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
fn get_user_default_army_setups_folder_dirs(game: &str) -> Result<PathBuf, String> {
    if let Some(mut p) = dirs::home_dir() {
        p = p.join("AppData\\Roaming\\The Creative Assembly");
        p = p.join(game);
        println!("{}", p.to_string_lossy());
        if p.exists() {
            return Ok(PathBuf::new());
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
fn get_user_default_army_setups_folder_directories(game: &str) -> Result<PathBuf, String> {
    if let Some(base_dirs) = BaseDirs::new() {
        let mut p = base_dirs.data_dir().to_path_buf();
        p = p.join("The Creative Assembly");
        p = p.join(game);

        println!("{}", p.to_string_lossy());

        if p.exists() {
            return Ok(PathBuf::new());
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
