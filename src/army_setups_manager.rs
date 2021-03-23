use crate::army_build::ArmyBuild;
use crate::army_setups_folder::{load_army_builds, valid_load_folder, ArmySetupsFolder};
use crate::factions::{get_faction_abbreviations, Faction};
use eframe::egui;
use eframe::egui::{Align, Color32, ScrollArea, Ui};
use std::ffi::OsString;

#[cfg_attr(
    feature = "persistence",
    derive(serde::Deserialize, serde::Serialize, Clone)
)]
pub struct ArmySetupsManager {
    load_folder: ArmySetupsFolder,

    pub(crate) army_builds: Vec<ArmyBuild>,
    display_builds: Vec<ArmyBuild>,
    search_string: String,
    search_faction: Faction,
    search_vs_faction: Faction,
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
        Self {
            load_folder: ArmySetupsFolder::default(),

            army_builds: vec![],
            display_builds: vec![],
            search_string: "".to_owned(),
            search_faction: Faction::ALL,
            search_vs_faction: Faction::ALL,
            selected_army_build: ArmyBuild::default(),
            max_display_builds: 50,
            track_item: usize::MAX,
            tack_item_align: Align::Center,
            offset: 0.0,

            insert_name: "AAAAAGGGHHWWWAAAAAAA".to_owned(),
            insert_folder: ArmySetupsFolder::default(),
        }
    }
}

impl ArmySetupsManager {
    pub fn valid_insert_name(&self) -> Result<String, String> {
        match self.insert_name.clone().chars().nth(0) {
            Some(c) => {
                if c == '.' {
                    return Err("Can't start a name with special characters".to_string());
                }
            }
            None => {
                return Err("Oy ya got to write something here".to_string());
            }
        }

        match OsString::from(self.insert_name.as_str()).to_str() {
            Some(str) => Ok(str.to_string()),
            None => Err("Can't have no funny characters".to_string()),
        }
    }

    pub fn load_folder(&mut self) -> Result<(), String> {
        match valid_load_folder(&self.insert_folder.folder_str) {
            Ok(()) => {
                let armies = load_army_builds(self.insert_folder.folder_str.as_str());
                self.insert_folder.valid_folder = true;
                self.army_builds = armies;
            }
            Err(e) => return Err(e),
        }
        Ok(())
    }

    fn update_display_builds(&mut self) {
        let mut display_builds: Vec<ArmyBuild> = vec![];
        //check faction
        if self.search_faction == Faction::ALL {
            display_builds = self.army_builds.clone();
        } else {
            for build in self.army_builds.iter() {
                if build.faction == self.search_faction {
                    display_builds.push(build.clone());
                }
            }
        }

        if self.search_vs_faction != Faction::ALL {
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
        ui.horizontal(|ui| {
            if ui.button("Search").clicked() {
                println!(
                    "search {} todo search & update display function",
                    self.search_string
                );
                self.update_display_builds();
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
            let faction_btn_response = egui::combo_box_with_label(
                ui,
                "Faction",
                format!("{:?}", &mut self.search_faction),
                |ui| {
                    ui.selectable_value(
                        &mut self.search_faction,
                        Faction::ALL,
                        get_faction_abbreviations(Faction::ALL),
                    );
                    ui.selectable_value(
                        &mut self.search_faction,
                        Faction::BM,
                        get_faction_abbreviations(Faction::BM),
                    );
                    ui.selectable_value(
                        &mut self.search_faction,
                        Faction::BRT,
                        get_faction_abbreviations(Faction::BRT),
                    );
                    ui.selectable_value(
                        &mut self.search_faction,
                        Faction::DE,
                        get_faction_abbreviations(Faction::DE),
                    );
                },
            );
            if faction_btn_response.changed() {
                self.update_display_builds();
            }

            let vs_faction_btn_response = egui::combo_box_with_label(
                ui,
                " vs Faction",
                format!("{:?}", &mut self.search_vs_faction),
                |ui| {
                    ui.selectable_value(
                        &mut self.search_vs_faction,
                        Faction::ALL,
                        get_faction_abbreviations(Faction::ALL),
                    );
                    ui.selectable_value(
                        &mut self.search_vs_faction,
                        Faction::BM,
                        get_faction_abbreviations(Faction::BM),
                    );
                    ui.selectable_value(
                        &mut self.search_vs_faction,
                        Faction::BRT,
                        get_faction_abbreviations(Faction::BRT),
                    );
                    ui.selectable_value(
                        &mut self.search_vs_faction,
                        Faction::DE,
                        get_faction_abbreviations(Faction::DE),
                    );
                },
            );
            if vs_faction_btn_response.changed() {
                self.update_display_builds()
            }
        });

        let go_to_scroll_offset = true;
        let mut scroll_top = false;
        let mut scroll_bottom = false;

        ui.horizontal(|ui| {
            scroll_top |= ui.button("Scroll to top").clicked();
            scroll_bottom |= ui.button("Scroll to bottom").clicked();
        });

        let mut scroll_area = ScrollArea::from_max_height(200.0);
        if go_to_scroll_offset {
            scroll_area = scroll_area.scroll_offset(self.offset);
        }

        ui.separator();
        let (_current_scroll, _max_scroll) = scroll_area.show(ui, |ui| {
            if scroll_top {
                ui.scroll_to_cursor(Align::TOP);
            }
            ui.vertical(|ui| {
                for item in 1..=50 {
                    if item == self.track_item {
                        let response =
                            ui.colored_label(Color32::YELLOW, format!("This is item {}", item));
                        response.scroll_to_me(self.tack_item_align);
                        if response.clicked() {
                            println!("Clicked Selected");
                        }
                    } else {
                        let response = ui.label(format!("This is item {}", item));
                        if response.clicked() {
                            self.track_item = item;
                            println!("New Selected {}", self.track_item);
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

        ui.horizontal(|ui| {
            ui.label("Selected ");
            ui.label(self.selected_army_build.file_stem.as_str());
        });
    }
}
