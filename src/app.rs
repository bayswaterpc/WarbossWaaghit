use eframe::{egui, epi};
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::ffi::OsStr;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct WarbossWaaghitApp {
    army_setups_folder: ArmySetupsFolder,
    army_picker: ArmyPicker
}

impl Default for WarbossWaaghitApp {
    fn default() -> Self {
        Self {
            army_setups_folder: Default::default(),
            army_picker: Default::default()
        }
    }
}

impl epi::App for WarbossWaaghitApp {
    fn name(&self) -> &str {
        "Generals Chest"
    }

    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn load(&mut self, storage: &dyn epi::Storage) {
        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let WarbossWaaghitApp {
            army_setups_folder,
            army_picker
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
            ui.heading("Warboss Waaghit");

            ui.add(egui::Button::new("Army Picker"));
            // ui.add(egui::Button::new("Tier Lists"));
            // ui.add(egui::Button::new("Personal Stats"));
            // ui.add(egui::Button::new("Resources"));



            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(
                    egui::Hyperlink::new("https://github.com/emilk/egui/").text("powered by egui"),
                );
            });
        });

        egui::TopPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {


            egui::CollapsingHeader::new("Army Setup Path")
                .default_open(army_setups_folder.show)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("File Path: ");
                        ui.text_edit_singleline(&mut army_setups_folder.army_setups_folder);
                    });


                    if ui.button("Load Builds From Folder").clicked() {
                        match validate_folder(& army_setups_folder.army_setups_folder){
                            Some(is_army_setups_folder) => {
                                if is_army_setups_folder {
                                    //load_army_builds(army_setups_folder.army_setups_folder.as_str());
                                    println!("you found the correct folder!!!!");
                                }
                            }
                            None => {
                                println!("error opening file");
                            }
                        }
                        println!("todo folder reader function");
                    }


                    egui::CollapsingHeader::new("Hint")
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.label("This folder can be found in your TWW2 Roaming App Data folder ex:");
                            ui.label("C:\\Users\\DaBiggestBoss\\AppData\\Roaming\\The Creative Assembly\\Warhammer2\\army_setups");
                        });
                });

            ui.heading("Army Picker");
            ui.horizontal(|ui| {
                ui.label("Selected ");
                ui.label(army_picker.selected_army_build.file_name.as_str());
            });

            ui.horizontal(|ui| {
                if ui.button("Insert Build as ").clicked() {
                    println!("todo make insert function")
                }
                ui.text_edit_singleline(&mut army_picker.insert_name);
            });


            //ARMIES Section
            ui.heading("Armies");
            ui.label("This shows how you can scroll to a specific item or pixel offset");

        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}


// ----------------------------------------------------------------------------

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
struct ArmySetupsFolder {
    army_setups_folder: String,
    valid_folder: bool,
    show: bool
}

impl Default for ArmySetupsFolder {
    fn default() -> Self {
        Self {
            army_setups_folder: "C:\\Users\\DaBiggestBoss\\AppData\\Roaming\\The Creative Assembly\\Warhammer2\\army_setups".to_string(),
            valid_folder: false,
            show: true
        }
    }
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Faction {
    BM,
    BRT,
    CH,
    DE,
    DW,
    EMP,
    GS,
    HE,
    LM,
    NRS,
    SKV,
    TK,
    VC,
    VP,
    WE,
    UNKNOWN,
    ALL
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct ArmyBuild {
    file_name: String,
    faction: Faction,
    vs_faction: Faction
}

impl Default for ArmyBuild {
    fn default() -> Self {
        Self {
            file_name: "".to_owned(),
            faction: Faction::UNKNOWN,
            vs_faction: Faction::UNKNOWN
        }
    }
}


#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
struct ArmyPicker {
    selected_army_build: ArmyBuild,
    insert_name: String,
    search_string: String,
    army_builds: Vec<ArmyBuild>,
    displayed_builds: Vec<ArmyBuild>
}

impl Default for ArmyPicker {
    fn default() -> Self {
        Self {
            selected_army_build: ArmyBuild::default(),
            insert_name: "AAAAAGGGHHWWWAAAAAAA".to_owned(),
            search_string: "".to_owned(),
            army_builds: vec![],
            displayed_builds: vec![]
        }
    }
}



pub fn validate_folder(folder_path: &str) -> Option<bool>{
    println!("into validate folder");
    let path = std::path::Path::new(folder_path);
    if ! path.exists() {
        return Some(false);

    }
    if ! path.is_dir() {
        return Some(false);
    }

    if !folder_path.contains("The Creative Assembly") ||
        !folder_path.contains("army_setups"){
        println!("Wrong Folder");
        return Some(false);
    }

    //make sure there are .army_setup files in the directory
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            println!("skip sub folder");
            continue
        } else {
            if is_army_setup_bool(&entry){
                return Some(true);
            }
        }
    }

    Some(false)
}


pub fn load_army_builds(_folder_path: &str) -> Vec<ArmyBuild>{
    vec![]
}

fn is_army_setup_bool(file: &DirEntry) -> bool {
    match file.file_type(){
        Ok(ft) => {if  ft.is_file(){
            match  file.path().extension().and_then(OsStr::to_str){
                None => { return false;}
                Some(ext) => {
                    if ext == "army_setup" {
                        return true;
                    }
                }
            }
        }}
        Err(_) => { return false; }
    }
    false
}


// the use for this is a directory search
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry) -> bool) -> io::Result<bool> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(true)
}


