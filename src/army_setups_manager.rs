use crate::army_build::ArmyBuild;
use crate::army_setups_folder::{
    get_owaagh_merge_conflict_dir, load_army_builds, validate_load_folder, ArmySetupsFolder,
};
use crate::ca_game::{get_ca_game_folder, CaGame};
use crate::factions::{faction_dropdown_button, Wh2Factions};
use eframe::egui;
use eframe::egui::{Align, Color32, ScrollArea, Ui};
use std::collections::HashSet;
use std::ffi::OsString;
use std::iter::FromIterator;
use std::path::PathBuf;
use wfd::{DialogParams, FOS_PICKFOLDERS};

#[cfg_attr(
    feature = "persistence",
    derive(serde::Deserialize, serde::Serialize, Clone)
)]
pub struct ArmySetupsManager {
    load_folder: ArmySetupsFolder,
    pub(crate) army_builds: HashSet<ArmyBuild>,
    has_merge_conflicts: bool,

    display_builds: Vec<ArmyBuild>,
    search_string: String,
    search_faction: Wh2Factions,
    search_vs_faction: Wh2Factions,
    pub(crate) selected_army_build: ArmyBuild,
    max_display_builds: usize,
    track_item: usize,
    tack_item_align: Align,
    offset: f32,

    pub(crate) insert_name: String,
    pub(crate) insert_folder: ArmySetupsFolder,
}

impl Default for ArmySetupsManager {
    fn default() -> Self {
        println!("default manager");
        let load_folder = ArmySetupsFolder::default();
        println!("load asf {:?}", load_folder);
        let army_builds = if load_folder.is_folder() {
            HashSet::from_iter(
                load_army_builds(load_folder.folder_string.as_str())
                    .iter()
                    .cloned(),
            )
        } else {
            HashSet::new()
        };

        let default_insert = get_ca_game_folder(CaGame::Warhammer2)
            .unwrap_or(
                PathBuf::from("C:\\Users\\DaBiggestBoss\\AppData\\Roaming\\The Creative Assembly\\Warhammer2\\army_setups")
            );
        let insert_folder =
            ArmySetupsFolder::new(default_insert.to_string_lossy().to_string().as_str());
        println!("insert asf {:?}", insert_folder);

        Self {
            load_folder,
            army_builds,
            has_merge_conflicts: false,

            display_builds: vec![],
            search_string: "".to_owned(),
            search_faction: Wh2Factions::ALL,
            search_vs_faction: Wh2Factions::ALL,
            selected_army_build: ArmyBuild::default(),
            max_display_builds: 50,
            track_item: usize::MAX,
            tack_item_align: Align::Center,
            offset: 0.0,

            insert_name: "AAAAAAGHOWAAAAAAA".to_owned(),
            insert_folder,
        }
    }
}

impl ArmySetupsManager {
    pub fn valid_insert_name(&self) -> Result<String, String> {
        match self.insert_name.clone().chars().nth(0) {
            Some(c) => {
                if c == '.' {
                    return Err(String::from("Can't start a name with special characters"));
                }
            }
            None => {
                return Err(String::from("Oy ya got to write something here"));
            }
        }

        match OsString::from(self.insert_name.as_str()).to_str() {
            Some(str) => Ok(str.to_string()),
            None => Err(String::from("Can't have no funny characters")),
        }
    }

    pub fn load_folder_to_owaagh_appdata(&mut self) -> Result<String, String> {
        match validate_load_folder(&self.load_folder.folder_string) {
            //todo flush merge pending folder
            Ok(()) => {
                let mut added_or_merged_notification = String::new();
                let armies = load_army_builds(self.insert_folder.folder_string.as_str());
                if self.load_folder.is_appdata_folder() {
                    self.army_builds = HashSet::from_iter(armies.iter().cloned());
                    let mut n_before = self.army_builds.len();
                    added_or_merged_notification =
                        format!("{} Builds In Folder", self.army_builds.len());
                    //self.army_builds = armies;
                    return Ok(added_or_merged_notification);
                } else {
                    let mut merge_conflict_folder = PathBuf::new();
                    match get_owaagh_merge_conflict_dir() {
                        Ok(p) => merge_conflict_folder = p,
                        Err(e) => return Err(e),
                    };

                    let mut n_before = self.army_builds.len();
                    let n_in_folder = armies.len();
                    for a in armies {
                        if !self.army_builds.contains(&a) {
                            self.army_builds.insert(a);
                        } else {
                            //todo write to merge folder and popup
                        }
                    }
                    let n_added = self.army_builds.len() - n_before;
                    if n_added == 0 {
                        self.has_merge_conflicts = false;
                    } else if 0 < n_added && n_added != n_before {
                        added_or_merged_notification = format!(
                            "{} Builds Added, {} Require Merge Handling",
                            n_added,
                            n_in_folder - n_added
                        );
                        self.has_merge_conflicts = true;
                    }

                    //self.army_builds = armies;
                    return Ok(added_or_merged_notification);
                }

                Ok(added_or_merged_notification)
            }
            Err(e) => return Err(e),
        }
    }

    fn update_display_builds(&mut self) {
        let mut display_builds: Vec<ArmyBuild> = vec![];
        //check faction
        if self.search_faction == Wh2Factions::ALL {
            display_builds = Vec::from_iter(self.army_builds.iter().cloned());
        } else {
            for build in self.army_builds.iter() {
                if build.faction == self.search_faction {
                    display_builds.push(build.clone());
                }
            }
        }

        if self.search_vs_faction != Wh2Factions::ALL {
            for ii in (0..display_builds.len()).rev() {
                if display_builds[ii].vs_faction != self.search_vs_faction {
                    display_builds.remove(ii);
                }
            }
        }

        let lower_case_search = self.search_string.to_ascii_lowercase();
        for ii in (0..display_builds.len()).rev() {
            let lower_file = display_builds[ii].file_stem.to_ascii_lowercase();
            if !lower_file.contains(lower_case_search.as_str()) {
                display_builds.remove(ii);
            }
        }

        self.display_builds = display_builds;
    }

    pub(crate) fn army_selector_scrolling_ui(&mut self, ui: &mut Ui, ctx: &egui::CtxRef) {
        if self.army_builds.is_empty() {
            ui.label("You got to load some armies first");
            return;
        }

        ui.horizontal(|ui| {
            if ui.button("Search").clicked() {
                self.update_display_builds();
                println!(
                    "search {} todo search & update display function",
                    self.display_builds.len()
                );
            }
            if ui
                .text_edit_singleline(&mut self.search_string)
                .lost_kb_focus()
                && ctx.input().key_pressed(egui::Key::Enter)
            {
                println!(
                    "enter search :) {} todo search & update display function",
                    self.search_string
                );
                self.update_display_builds();
            }
        });

        ui.horizontal(|ui| {
            //ui.add(doc_link_label("Combo box", "faction_search"));
            let faction_btn_response =
                faction_dropdown_button(ui, &mut self.search_faction, "Faction", false);
            if faction_btn_response.clicked() {
                println!("change faction");
                self.update_display_builds();
            }

            let vs_faction_btn_response =
                faction_dropdown_button(ui, &mut self.search_vs_faction, "vs Faction", true);
            if vs_faction_btn_response.clicked() {
                println!("change vs faction");
                self.update_display_builds();
            }
        });

        let mut scroll_top = false;
        let mut scroll_bottom = false;

        ui.horizontal(|ui| {
            scroll_top |= ui.button("Scroll to top").clicked();
            scroll_bottom |= ui.button("Scroll to bottom").clicked();
        });

        let scroll_area = ScrollArea::from_max_height(200.0);

        ui.separator();
        let (_current_scroll, _max_scroll) = scroll_area.show(ui, |ui| {
            if scroll_top {
                ui.scroll_to_cursor(Align::TOP);
            }
            ui.vertical(|ui| {
                for item in 0..self.display_builds.len() {
                    if item == self.track_item {
                        ui.colored_label(
                            Color32::YELLOW,
                            self.display_builds[item].file_stem.as_str(),
                        );
                    } else {
                        let response = ui.label(self.display_builds[item].file_stem.as_str());
                        if response.clicked() {
                            self.selected_army_build = self.display_builds[item].clone();
                            self.track_item = item;
                        }
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
    }

    pub fn insert_army(&self) -> Result<(), String> {
        //Check If Inputs Valid
        if !self.insert_folder.is_insert_folder() {
            return Err("You're folder's no good".to_string());
        }
        let insert_name = match self.valid_insert_name() {
            Ok(valid_name) => valid_name,
            Err(e) => {
                return Err(e);
            }
        };

        let selected_file = self.selected_army_build.file.to_str().unwrap(); //osstring prevalidated so none option should be fine
        if !self.selected_army_build.file.is_file() {
            println!("{}", selected_file);
            return Err("Da army file went missing!!!!".to_string());
        }

        let insert_file =
            self.insert_folder.folder_string.clone() + "/" + insert_name.as_str() + ".army_setup";
        match std::fs::copy(selected_file, insert_file.as_str()) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{}", e);
                Err("Couldn't copy".to_string())
            }
        }
    }

    pub fn insert_army_ui(&mut self, ui: &mut Ui, ctx: &egui::CtxRef) {
        if self.track_item > self.display_builds.len() {
            ui.label("You got to select an army first");
            return;
        }
        ui.horizontal(|ui| {
            ui.label("Selected ");
            ui.label(self.selected_army_build.file_stem.as_str());
        });
        ui.horizontal(|ui| {
            if ui.button("Insert Build as ").clicked() {
                match self.insert_army() {
                    Ok(()) => {
                        println!("inserted")
                    }
                    Err(e) => {
                        println!("err {}", e)
                    }
                }
            }
            if ui
                .text_edit_singleline(&mut self.insert_name)
                .lost_kb_focus()
                && ctx.input().key_pressed(egui::Key::Enter)
            {
                match self.insert_army() {
                    Ok(()) => {
                        println!("inserted")
                    }
                    Err(e) => {
                        println!("err {}", e)
                    }
                }
            }
        });
    }

    pub fn selector_central_panel_ui(&mut self, ui: &mut Ui, ctx: &egui::CtxRef) {
        egui::CollapsingHeader::new("Load Army Setups")
            .default_open(self.load_folder.is_load_folder())
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Load Folder").clicked() {
                        match self.load_folder_to_owaagh_appdata() {
                            Ok(_) => {}
                            Err(e) => {
                                println!("btn {}", e);
                                self.load_folder.set_load_folder_error();
                            }
                        }
                    }
                    if ui
                        .text_edit_singleline(&mut self.load_folder.folder_string)
                        .lost_kb_focus()
                        && ctx.input().key_pressed(egui::Key::Enter)
                    {
                        match self.load_folder_to_owaagh_appdata() {
                            Ok(_) => {}
                            Err(e) => {
                                println!("enter {}", e);
                                self.load_folder.set_load_folder_error();
                            }
                        }
                    }
                    if ui.button("...").clicked() {
                        println!("aa {}", self.load_folder.folder_string.as_str());

                        let params = DialogParams {
                            default_folder: self.load_folder.folder_string.as_str(),
                            options: FOS_PICKFOLDERS,
                            ..Default::default()
                        };

                        match wfd::open_dialog(params) {
                            Ok(res) => {
                                println!("{:?}", res.selected_file_path);
                                self.load_folder.folder_string =
                                    res.selected_file_path.to_string_lossy().to_string();
                                self.load_folder.set_load_folder_error();
                            }
                            Err(e) => {
                                println!("load folder dialog e {:?}", e);
                            }
                        }
                    }
                });

                if !self.load_folder.folder_error.is_empty() {
                    ui.label(self.load_folder.folder_error.clone());

                    egui::CollapsingHeader::new("Hint")
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.label("This can be any folder with a \'.army_setup\' file ex:");
                            ui.label("C:\\Users\\DaBiggestBoss\\Downloads\\ArmySetups");
                        });
                }
            });

        egui::CollapsingHeader::new("Select Army Setup")
            .default_open(false)
            .show(ui, |ui| {
                self.army_selector_scrolling_ui(ui, ctx);
            });

        egui::CollapsingHeader::new("Insert Army Setup")
            .default_open(self.insert_folder.is_insert_folder())
            .show(ui, |ui| {
                //
                ui.horizontal(|ui| {
                    if ui.button("Insert Folder").clicked() {
                        self.insert_folder.set_insert_folder_error();
                    }
                    if ui
                        .text_edit_singleline(&mut self.insert_folder.folder_string)
                        .lost_kb_focus()
                        && ctx.input().key_pressed(egui::Key::Enter)
                    {
                        self.insert_folder.set_insert_folder_error();
                    }

                    if ui.button("...").clicked() {
                        println!("aa {}", self.insert_folder.folder_string.as_str());

                        let params = DialogParams {
                            default_folder: self.insert_folder.folder_string.as_str(),
                            options: FOS_PICKFOLDERS,
                            ..Default::default()
                        };

                        match wfd::open_dialog(params) {
                            Ok(res) => {
                                println!("{:?}", res.selected_file_path);
                                self.insert_folder.folder_string =
                                    res.selected_file_path.to_string_lossy().to_string();
                                self.insert_folder.set_insert_folder_error();
                            }
                            Err(e) => {
                                println!("load folder dialog e {:?}", e);
                            }
                        }
                    }
                });

                if !self.insert_folder.folder_error.is_empty() {
                    ui.label(self.insert_folder.folder_error.clone());

                    egui::CollapsingHeader::new("Hint")
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.label("This must match the default army setup save folder for your game ex:");
                            ui.label("C:\\Users\\DaBiggestBoss\\Downloads\\Roaming\\The Creative Assembly\\Warhammer2\\army_setups");
                        });
                }

                if self.insert_folder.is_insert_folder() {
                    self.insert_army_ui(ui, ctx);
                }
            });
    }
}
