use eframe::{egui, epi};
use std::io;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use std::ffi::{OsStr, OsString};
use eframe::egui::{Align, Slider, DragValue, Ui, Widget, Color32, ScrollArea};
use std::io::Error;
use rand::Rng;


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct WarbossWaaghitApp {
    army_setups_folder: ArmySetupsFolder,
    army_setups_manager: ArmySetupsManager
}

impl Default for WarbossWaaghitApp {
    fn default() -> Self {
        Self {
            army_setups_folder: Default::default(),
            army_setups_manager: Default::default()
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
            army_setups_manager
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
            ui.heading("Warboss Waaghit");

            ui.add(egui::Button::new("Army Setups"));
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


            egui::CollapsingHeader::new("Load Army Setups")
                .default_open(army_setups_manager.insert_folder.show)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Load Folder").clicked() {
                            match army_setups_manager.load_folder(){
                                Ok(_) => {}
                                Err(e) => {println!("{}", e)}
                            }
                        }
                        if ui.text_edit_singleline(&mut army_setups_manager.insert_folder.folder_str).lost_kb_focus() && ctx.input().key_pressed(egui::Key::Enter) {
                            match army_setups_manager.load_folder(){
                                Ok(_) => {}
                                Err(e) => {println!("{}", e)}
                            }
                        }
                    }

                    );




                    egui::CollapsingHeader::new("Hint")
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.label("This can be any folder with a \'.army_setup\' file");
                            ui.label("The default army setup save folder can be found in your TWW2 Roaming AppData folder ex:");
                            ui.label("C:\\Users\\DaBiggestBoss\\AppData\\Roaming\\The Creative Assembly\\Warhammer2\\army_setups");
                        });
                });




            //ARMIES Section
            // ui.heading("Armies");
            // ui.label("This shows how you can scroll to a specific item or pixel offset");

            egui::CollapsingHeader::new("Select Army Setup").default_open(false).show(ui, |ui| {
                if army_setups_manager.army_builds.is_empty() {
                    ui.label("You got to load some armies first");
                }else {
                    if ui.button("Search").clicked() {
                        println!("search {} todo search & update display function", army_setups_manager.search_string);
                    }
                    if ui.text_edit_singleline(&mut army_setups_manager.search_string).lost_kb_focus() && ctx.input().key_pressed(egui::Key::Enter){
                        println!("enter search :) {} todo search & update display function", army_setups_manager.search_string);
                    }

                    let mut open = true;
                    let mut scroll_widget = Scrolling::default();
                    scroll_widget.show(ctx, &mut open);
                }
                });


            egui::CollapsingHeader::new("Insert Army Setup").default_open(false).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Selected ");
                    ui.label(army_setups_manager.selected_army_build.file_name.to_str().unwrap());
                });

                ui.horizontal(|ui| {
                    if ui.button("Insert Build as ").clicked() {
                        let mut rng = rand::thread_rng();
                        let indx : usize = rng.gen_range(0..army_setups_manager.army_builds.len());
                        println!("inserting {}", army_setups_manager.army_builds[indx].file_name.to_str().unwrap());
                        army_setups_manager.selected_army_build = army_setups_manager.army_builds[indx].clone();
                        match insert_army(&army_setups_manager, &army_setups_manager.insert_folder){
                            Ok(()) => {println!("inserted")},
                            Err(e) => {println!("err {}", e)}
                        }
                    }
                    if ui.text_edit_singleline(&mut army_setups_manager.insert_name).lost_kb_focus() && ctx.input().key_pressed(egui::Key::Enter){
                        let mut rng = rand::thread_rng();
                        let indx : usize = rng.gen_range(0..army_setups_manager.army_builds.len());
                        println!("inserting {}", army_setups_manager.army_builds[indx].file_name.to_str().unwrap());
                        army_setups_manager.selected_army_build = army_setups_manager.army_builds[indx].clone();
                        match insert_army(&army_setups_manager, &army_setups_manager.insert_folder){
                            Ok(()) => {println!("inserted")},
                            Err(e) => {println!("err {}", e)}
                        }
                    }
                });
            });
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

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize, Clone))]
pub struct ArmySetupsFolder {
    folder_str: String,
    valid_folder: bool,
    show: bool
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

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize, Debug, Clone))]
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

pub fn get_army_faction(file_name: &OsString) -> Faction{
    let lower_file = file_name.to_str().unwrap().to_ascii_lowercase();
    if lower_file.contains("bm vs") {
        return Faction::BM;
    } else if lower_file.contains("brt vs") {
        return Faction::BRT;
    } else if lower_file.contains("ch vs") {
        return Faction::CH;
    } else if lower_file.contains("dw vs") {
        return Faction::DW;
    } else if lower_file.contains("emp vs") {
        return Faction::EMP;
    } else if lower_file.contains("ew vs") {
        return Faction::DW;
    } else if lower_file.contains("de vs") {
        return Faction::DE;
    }
    else if lower_file.contains("gs vs") {
        return Faction::GS;
    } else if lower_file.contains("he vs") {
        return Faction::HE;
    }else if lower_file.contains("lm vs") {
        return Faction::LM;
    }else if lower_file.contains("nrs vs") {
        return Faction::NRS;
    }else if lower_file.contains("skv vs") {
        return Faction::SKV;
    }else if lower_file.contains("tk vs") {
        return Faction::TK;
    }else if lower_file.contains("vc vs") {
        return Faction::VC;
    }else if lower_file.contains("vp vs") {
        return Faction::VP;
    }else if lower_file.contains("we vs") {
        return Faction::WE;
    }
    Faction::UNKNOWN
}

pub fn get_vs_faction(file_name: &OsString) -> Faction{
    let lower_file = file_name.to_str().unwrap().to_ascii_lowercase();
    if lower_file.contains("vs bm") {
        return Faction::BM;
    } else if lower_file.contains("vs brt") {
        return Faction::BRT;
    } else if lower_file.contains("vs ch") {
        return Faction::CH;
    } else if lower_file.contains("vs de") {
        return Faction::DE;
    } else if lower_file.contains("vs dw") {
        return Faction::DW;
    } else if lower_file.contains("vs emp") {
        return Faction::EMP;
    } else if lower_file.contains("vs gs") {
        return Faction::GS;
    } else if lower_file.contains("vs he") {
        return Faction::HE;
    }else if lower_file.contains("vs lm") {
        return Faction::LM;
    }else if lower_file.contains("vs nrs") {
        return Faction::NRS;
    }else if lower_file.contains("vs skv") {
        return Faction::SKV;
    }else if lower_file.contains("vs tk") {
        return Faction::TK;
    }else if lower_file.contains("vs vc") {
        return Faction::VC;
    }else if lower_file.contains("vs vp") {
        return Faction::VP;
    }else if lower_file.contains("vs we") {
        return Faction::WE;
    }else if lower_file.contains("vs aa") {
        return Faction::ALL;
    }
    Faction::UNKNOWN
}


#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize, Clone))]
pub struct ArmyBuild {
    file: PathBuf,
    file_name: OsString,
    faction: Faction,
    vs_faction: Faction
}

impl Default for ArmyBuild {
    fn default() -> Self {
        Self {
            file: PathBuf::new(),
            file_name: OsString::new(),
            faction: Faction::UNKNOWN,
            vs_faction: Faction::UNKNOWN
        }
    }
}


#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize, Clone))]
pub struct ArmySetupsManager {
    load_folder: ArmySetupsFolder,
    army_builds: Vec<ArmyBuild>,
    displayed_builds: Vec<ArmyBuild>,
    search_string: String,
    search_faction: Faction,
    search_vs_faction: Faction,
    selected_army_build: ArmyBuild,
    insert_name: String,
    insert_folder: ArmySetupsFolder,
}

impl Default for ArmySetupsManager {
    fn default() -> Self {
        Self {
            load_folder: ArmySetupsFolder::default(),
            army_builds: vec![],
            displayed_builds: vec![],
            search_string: "".to_owned(),
            search_faction: Faction::ALL,
            search_vs_faction: Faction::ALL,
            selected_army_build: ArmyBuild::default(),
            insert_name: "AAAAAGGGHHWWWAAAAAAA".to_owned(),
            insert_folder: ArmySetupsFolder::default(),
        }
    }
}

impl ArmySetupsManager {
    pub fn valid_insert_name(&self) -> Result<String, String> {
        match self.insert_name.clone().chars().nth(0) {
            Some( c ) => {
                if c == '.' {
                    return Err("Can't start a name with special characters".to_string())
                }
            },
            None => { return Err("Oy ya got to write something here".to_string()); }
        }

        match OsString::from( self.insert_name.as_str()).to_str() {
            Some(str) => Ok((str.to_string())),
            None => Err("Can't have no funny characters".to_string())
        }
    }

    pub fn load_folder(&mut self) -> Result<(), String>{
        match valid_load_folder(& self.insert_folder.folder_str){
            Ok(()) => {
                let armies =  load_army_builds(self.insert_folder.folder_str.as_str());
                self.insert_folder.valid_folder = true;
                self.army_builds = armies;
            },
            Err(e) => return Err(e),
        }
        Ok(())
    }
}

pub fn insert_army(picker: &ArmySetupsManager, folder: &ArmySetupsFolder) -> Result<(), String> {
    //Check If Inputs Valid
    if !folder.valid_folder{return Err("You're folder's no good".to_string())}
    let insert_name = match picker.valid_insert_name() {
        Ok(valid_name) => valid_name,
        Err(e) => {return Err(e);}
    };

    let selected_file = picker.selected_army_build.file.to_str().unwrap();//osstring prevalidated so none option should be fine
    if !picker.selected_army_build.file.is_file(){
        println!("{}", selected_file);
        return Err("Da army file went missing!!!!".to_string());
    }

    //Do Copy
    let insert_file = folder.folder_str.clone() + "/" + insert_name.as_str() + ".army_setup";
    match std::fs::copy(selected_file, insert_file.as_str()){
        Ok(_) => {Ok(())}
        Err(e) => {
            println!("{}", e);
            Err("Couldn't copy".to_string())
        }
    }
}


pub fn valid_load_folder(folder_path: &str) -> Result<(), String> {
    println!("into validate folder");
    let path = std::path::Path::new(folder_path);
    if ! path.exists() {
        return Err("Dat path dont even exist!!".to_string());

    }
    if ! path.is_dir() {
        return Err("Dat path dont even exist!!".to_string());
    }

    //make sure there are .army_setup files in the directory
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            println!("skip sub folder");
            continue
        } else {
            if is_army_setup_file(&entry){
                return Ok(());
            }
        }
    }

    Err("The folder got no \'.army_setup\' files".to_string())
}

pub fn valid_insert_folder(folder_path: &str) -> Result<(), String> {
    println!("into validate folder");
    let path = std::path::Path::new(folder_path);
    if ! path.exists() {
        return Err("Dat path dont even exist!!".to_string());

    }
    if ! path.is_dir() {
        return Err("Dat path dont even exist!!".to_string());
    }

    let setup_root = "AppData/Roaming/The Creative Assembly/Warhammer2/";
    if !folder_path.contains(setup_root) ||
        !folder_path.contains("army_setups"){
        let s = format!("Da path needs dis \'{}\'", setup_root);
        return Err(s);
    }


    Ok(())
}


pub fn load_army_builds(folder_path: &str) -> Vec<ArmyBuild>{
    let mut builds = vec![];
    match valid_load_folder(folder_path) {
        Ok(_) => {},
        Err(_) => {return builds}
    }


    let path = std::path::Path::new(folder_path);

    //make sure there are .army_setup files in the directory
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() { //skip subfolder
            continue
        } else {
            if is_army_setup_file(&entry) {
                let file_stem = entry.path().file_stem().unwrap().to_os_string();
                builds.push(
                    ArmyBuild {
                        file: entry.path(),
                        file_name: file_stem.clone(),
                        faction: get_army_faction(&file_stem),
                        vs_faction: get_vs_faction(&file_stem)
                    }
                );
                //println!("{:?} {:?} {:?}", builds.last().unwrap().file_name, builds.last().unwrap().faction, builds.last().unwrap().vs_faction);
            }
        }
    }
    builds
}

fn is_army_setup_file(file: &DirEntry) -> bool {
    match file.file_type(){
        Ok(ft) => {if  ft.is_file(){


            let path = file.path();
            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            if file_stem.len() ==0 {return false};
            if file_stem.chars().next().unwrap() == '.' {return false}
            //if file_stem[0] == '.' {return false}

            match  path.extension().and_then(OsStr::to_str){
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


#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
#[derive(PartialEq)]
pub struct Scrolling {
    track_item: usize,
    tack_item_align: Align,
    offset: f32,
}

impl Default for Scrolling {
    fn default() -> Self {
        Self {
            track_item: 25,
            tack_item_align: Align::Center,
            offset: 0.0,
        }
    }
}

impl Scrolling {
    fn name(&self) -> &'static str {
        "↕ Scrolling"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .resizable(false)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.label("This shows how you can scroll to a specific item or pixel offset");

        let mut track_item = false;
        let mut go_to_scroll_offset = false;
        let mut scroll_top = false;
        let mut scroll_bottom = false;

        ui.horizontal(|ui| {
            ui.label("Scroll to a specific item index:");
            track_item |= ui
                .add(Slider::usize(&mut self.track_item, 1..=50).text("Track Item"))
                .dragged();
        });

        ui.horizontal(|ui| {
            ui.label("Item align:");
            track_item |= ui
                .radio_value(&mut self.tack_item_align, Align::Min, "Top")
                .clicked();
            track_item |= ui
                .radio_value(&mut self.tack_item_align, Align::Center, "Center")
                .clicked();
            track_item |= ui
                .radio_value(&mut self.tack_item_align, Align::Max, "Bottom")
                .clicked();
        });

        ui.horizontal(|ui| {
            ui.label("Scroll to a specific offset:");
            go_to_scroll_offset |= ui
                .add(DragValue::f32(&mut self.offset).speed(1.0).suffix("px"))
                .dragged();
        });

        ui.horizontal(|ui| {
            scroll_top |= ui.button("Scroll to top").clicked();
            scroll_bottom |= ui.button("Scroll to bottom").clicked();
        });

        let mut scroll_area = ScrollArea::from_max_height(200.0);
        if go_to_scroll_offset {
            scroll_area = scroll_area.scroll_offset(self.offset);
        }

        ui.separator();
        let (current_scroll, max_scroll) = scroll_area.show(ui, |ui| {
            if scroll_top {
                ui.scroll_to_cursor(Align::TOP);
            }
            ui.vertical(|ui| {
                for item in 1..=50 {
                    if track_item && item == self.track_item {
                        let response =
                            ui.colored_label(Color32::YELLOW, format!("This is item {}", item));
                        response.scroll_to_me(self.tack_item_align);
                    } else {
                        ui.label(format!("This is item {}", item));
                    }
                }
            });

            if scroll_bottom {
                ui.scroll_to_cursor(Align::BOTTOM);
            }

            let margin = ui.visuals().clip_rect_margin;

            let current_scroll = ui.clip_rect().top() - ui.min_rect().top() + margin;
            let max_scroll = ui.min_rect().height() - ui.clip_rect().height() + 2.0 * margin;
            (current_scroll, max_scroll)
        });
        ui.separator();

        ui.label(format!(
            "Scroll offset: {:.0}/{:.0} px",
            current_scroll, max_scroll
        ));

        ui.separator();
        ui.vertical_centered(|ui| {
            egui::reset_button(ui, self);
            //ui.add(crate::__egui_github_link_file!());
        });
    }
}


#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
#[derive(PartialEq)]
pub struct ArmyDisplayScroll {
    track_item: usize,
    tack_item_align: Align,
    offset: f32,
}

impl Default for ArmyDisplayScroll {
    fn default() -> Self {
        Self {
            track_item: 25,
            tack_item_align: Align::Center,
            offset: 0.0,
        }
    }
}

impl ArmyDisplayScroll {
    fn name(&self) -> &'static str {
        "↕ Scrolling"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .resizable(false)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.label("This shows how you can scroll to a specific item or pixel offset");

        let mut track_item = false;
        let mut go_to_scroll_offset = false;
        let mut scroll_top = false;
        let mut scroll_bottom = false;

        ui.horizontal(|ui| {
            ui.label("Scroll to a specific item index:");
            track_item |= ui
                .add(Slider::usize(&mut self.track_item, 1..=50).text("Track Item"))
                .dragged();
        });

        ui.horizontal(|ui| {
            ui.label("Item align:");
            track_item |= ui
                .radio_value(&mut self.tack_item_align, Align::Min, "Top")
                .clicked();
            track_item |= ui
                .radio_value(&mut self.tack_item_align, Align::Center, "Center")
                .clicked();
            track_item |= ui
                .radio_value(&mut self.tack_item_align, Align::Max, "Bottom")
                .clicked();
        });

        ui.horizontal(|ui| {
            ui.label("Scroll to a specific offset:");
            go_to_scroll_offset |= ui
                .add(DragValue::f32(&mut self.offset).speed(1.0).suffix("px"))
                .dragged();
        });

        ui.horizontal(|ui| {
            scroll_top |= ui.button("Scroll to top").clicked();
            scroll_bottom |= ui.button("Scroll to bottom").clicked();
        });

        let mut scroll_area = ScrollArea::from_max_height(200.0);
        if go_to_scroll_offset {
            scroll_area = scroll_area.scroll_offset(self.offset);
        }

        ui.separator();
        let (current_scroll, max_scroll) = scroll_area.show(ui, |ui| {
            if scroll_top {
                ui.scroll_to_cursor(Align::TOP);
            }
            ui.vertical(|ui| {
                for item in 1..=50 {
                    if track_item && item == self.track_item {
                        let response =
                            ui.colored_label(Color32::YELLOW, format!("This is item {}", item));
                        response.scroll_to_me(self.tack_item_align);
                    } else {
                        ui.label(format!("This is item {}", item));
                    }
                }
            });

            if scroll_bottom {
                ui.scroll_to_cursor(Align::BOTTOM);
            }

            let margin = ui.visuals().clip_rect_margin;

            let current_scroll = ui.clip_rect().top() - ui.min_rect().top() + margin;
            let max_scroll = ui.min_rect().height() - ui.clip_rect().height() + 2.0 * margin;
            (current_scroll, max_scroll)
        });
        ui.separator();

        ui.label(format!(
            "Scroll offset: {:.0}/{:.0} px",
            current_scroll, max_scroll
        ));

        ui.separator();
        ui.vertical_centered(|ui| {
            egui::reset_button(ui, self);
            //ui.add(crate::__egui_github_link_file!());
        });
    }
}
