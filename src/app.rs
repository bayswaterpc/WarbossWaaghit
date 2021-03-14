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
        "egui template"
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
            ui.heading("Side Panel");


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

        egui::CentralPanel::default().show(ctx, |_ui| {
            // ui.heading("egui template");
            // ui.hyperlink("https://github.com/emilk/egui_template");
            // ui.add(egui::github_link_file_line!(
            //     "https://github.com/emilk/egui_template/blob/master/",
            //     "Direct link to source code."
            // ));
            //egui::warn_if_debug_build(ui);

            // ui.separator();
            //
            // ui.heading("Central Panel");
            // ui.label("The central panel the region left after adding TopPanel's and SidePanel's");
            // ui.label("It is often a great place for big things, like drawings:");
            //
            // ui.heading("Draw with your mouse to paint:");
            // painting.ui_control(ui);
            // egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
            //     painting.ui_content(ui);
            // });
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
    army_setup_folder: String,
    valid_folder: bool,
    show: bool
}

impl Default for ArmySetupsFolder {
    fn default() -> Self {
        Self {
            army_setup_folder: "C:\\Users\\DaBiggestBoss\\AppData\\Roaming\\The Creative Assembly\\Warhammer2\\army_setups".to_string(),
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
            insert_name: "AAAAAAGH_WAAAAAAA".to_owned(),
            search_string: "".to_owned(),
            army_builds: vec![],
            displayed_builds: vec![]
        }
    }
}



pub fn _validate_folder(folder_path: &str) -> Option<bool>{
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
            if _is_army_setup_bool(&entry){
                return Some(true);
            }
        }
    }

    Some(false)
}


pub fn _load_army_builds(_folder_path: &str) -> Vec<ArmyBuild>{
    vec![]
}

fn _is_army_setup_bool(file: &DirEntry) -> bool {
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


