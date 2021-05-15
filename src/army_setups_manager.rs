use crate::army_build::{
    funds_slider_ui, get_army_build_display_column_title, show_army_build_header_row, ArmyBuild,
    ArmyBuildDisplayColumns,
};
use crate::army_setups_folder::{
    get_owaagh_army_setups_dir, get_tmp_default_army_setups_dir, load_army_builds,
    validate_load_folder, ArmySetupsFolder,
};
use crate::ca_game::{
    get_ca_game_army_setup_ext, get_ca_game_army_setups_folder, get_ca_game_title, CaGame,
};
use crate::factions::{faction_dropdown_button, Wh2Factions};
use crate::ymd_hms_dash_format::YMD_HMS_FORMAT;
use chrono::offset::Utc;
use chrono::DateTime;
use eframe::egui;
use eframe::egui::{Align, Color32, ScrollArea, Ui};
use enum_iterator::IntoEnumIterator;
use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::iter::FromIterator;
use std::path::PathBuf;
use std::time::SystemTime;
use wfd::{DialogParams, FOS_PICKFOLDERS};

#[cfg_attr(
    feature = "persistence",
    derive(serde::Deserialize, serde::Serialize, Clone)
)]
pub struct ArmySetupsManager {
    selected_game: CaGame,

    load_folder: ArmySetupsFolder,
    pub(crate) army_builds: HashMap<CaGame, HashSet<ArmyBuild>>,

    display_builds: Vec<ArmyBuild>,
    search_string: String,
    search_mod: String,
    search_faction: Wh2Factions,
    search_vs_faction: Wh2Factions,
    pub(crate) selected_army_build: ArmyBuild,
    edit_build: ArmyBuild,

    selected_display_build_indx: usize,
    tack_item_align: Align,
    offset: f32,

    pub(crate) insert_name: String,
    pub(crate) insert_folder: ArmySetupsFolder,
}

impl Default for ArmySetupsManager {
    fn default() -> Self {

        let default_load_path = get_ca_game_army_setups_folder(CaGame::Warhammer2)
            .unwrap_or(
                PathBuf::from("C:\\Users\\DaBiggestBoss\\AppData\\Roaming\\The Creative Assembly\\Warhammer2\\army_setups")
            );
        let load_folder =
            ArmySetupsFolder::new(default_load_path.to_string_lossy().to_string().as_str());

        let default_insert_path = get_ca_game_army_setups_folder(CaGame::Warhammer2)
            .unwrap_or(
                PathBuf::from("C:\\Users\\DaBiggestBoss\\AppData\\Roaming\\The Creative Assembly\\Warhammer2\\army_setups")
            );
        let insert_folder =
            ArmySetupsFolder::new(default_insert_path.to_string_lossy().to_string().as_str());


        let mut army_builds = ArmySetupsManager::get_ca_army_builds();

        if ArmySetupsFolder::get_tmp_defaults_folder().exists() {
            ArmySetupsManager::append_default_army_builds(&mut army_builds);
            std::fs::remove_dir_all(ArmySetupsFolder::get_tmp_defaults_folder());
        }

        let selected_game = CaGame::Warhammer2;
        let display_builds = match army_builds.get(&selected_game) {
            Some(army_set) => army_set.iter().cloned().collect(),
            None => vec![],
        };

        Self {
            selected_game,

            load_folder,
            army_builds,

            display_builds,
            search_string: "".to_owned(),
            search_faction: Wh2Factions::ALL,
            search_vs_faction: Wh2Factions::ALL,
            search_mod: "".to_owned(),
            selected_army_build: ArmyBuild::default(),
            edit_build: ArmyBuild::default(),

            selected_display_build_indx: usize::MAX,
            tack_item_align: Align::Center,
            offset: 0.0,

            insert_name: "AAAAAAGHOWAAAAAAA".to_owned(),
            insert_folder,
        }
    }
}

impl ArmySetupsManager {
    fn get_ca_army_builds() -> HashMap<CaGame, HashSet<ArmyBuild>> {
        let mut army_builds: HashMap<CaGame, HashSet<ArmyBuild>> = HashMap::new();
        for ca_game in CaGame::into_enum_iter() {
            let mut folder = String::new();
            match get_ca_game_army_setups_folder(ca_game.clone()) {
                Ok(p) => folder = p.to_string_lossy().to_string(),
                Err(_) => continue,
            }
            let game_army_builds = load_army_builds(folder.as_str(), &ca_game);
            if game_army_builds.len() > 0 {
                army_builds.insert(
                    ca_game,
                    HashSet::from_iter(game_army_builds.iter().cloned()),
                );
            }
        }
        army_builds
    }

    fn get_tmp_default_builds() -> HashMap<CaGame, HashSet<ArmyBuild>> {
        let mut army_builds: HashMap<CaGame, HashSet<ArmyBuild>> = HashMap::new();
        for ca_game in CaGame::into_enum_iter() {
            let mut folder;
            match get_tmp_default_army_setups_dir(&ca_game) {
                Ok(p) => folder = p.to_string_lossy().to_string(),
                Err(_) => continue,
            }
            let game_army_builds = load_army_builds(folder.as_str(), &ca_game);
            if game_army_builds.len() > 0 {
                army_builds.insert(
                    ca_game,
                    HashSet::from_iter(game_army_builds.iter().cloned()),
                );
            }
        }
        army_builds
    }

    fn append_default_army_builds(army_builds: &mut HashMap<CaGame, HashSet<ArmyBuild>>) {
        let mut default_game_armies = ArmySetupsManager::get_tmp_default_builds();

        for (ca_game, default_armies) in default_game_armies.into_iter() {
            let mut tmp_default_game_path;
            match get_tmp_default_army_setups_dir(&ca_game) {
                Ok(p) => {
                    tmp_default_game_path = p;
                }
                Err(e) => {
                    println!("append_default_army_builds {}", e);
                    continue;
                }
            }
            let game_extension = format!(".{}", get_ca_game_army_setup_ext(ca_game.clone()));

            let res1 = get_owaagh_army_setups_dir(&ca_game);
            if res1.is_err() {
                return;
            }
            let owaagh_appdata_path = res1.unwrap().clone();
            if !owaagh_appdata_path.exists() {
                match std::fs::create_dir(owaagh_appdata_path.as_path()) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("couldn't create owaaagh app data folder {}", e);
                        return;
                    }
                }
            }

            match army_builds.get_mut(&ca_game) {
                Some(army_set) => {
                    for mut d_a in default_armies.into_iter() {
                        let mut new_file_path = owaagh_appdata_path.clone();
                        new_file_path.push(d_a.file_stem.as_str());
                        new_file_path.push(game_extension.as_str());

                        match std::fs::copy(d_a.file.clone(), new_file_path.clone()) {
                            Ok(_) => d_a.file = new_file_path,
                            Err(e) => {
                                println!("{} for file_stem {}", e, d_a.file_stem);
                                return;
                            }
                        }
                        army_set.insert(d_a);
                    }
                }
                None => {
                    army_builds.insert(ca_game, default_armies);
                }
            }
        }
    }

    pub fn get_selected_game(&self) -> String {
        get_ca_game_title(&self.selected_game)
    }

    pub fn set_selected_game(&mut self, ca_game: CaGame) {
        self.selected_game = ca_game;
    }

    fn get_game_army_builds(&mut self, ca_game: CaGame) -> HashSet<ArmyBuild> {
        match self.army_builds.get(&ca_game) {
            Some(h) => h.clone(),
            None => {
                self.army_builds.insert(ca_game.clone(), HashSet::new());
                self.army_builds.get(&ca_game).unwrap().clone()
            }
        }
    }

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
        let res = validate_load_folder(&self.load_folder.folder_string);
        if res.is_err() {
            return Err(format!(
                "load_folder_to_owaagh_appdata {}",
                res.err().unwrap()
            ));
        }
        let res1 = get_owaagh_army_setups_dir(&self.selected_game);
        if res1.is_err() {
            return Err(format!(
                "load_folder_to_owaagh_appdata {}",
                res1.err().unwrap()
            ));
        }
        let owaagh_appdata_path = res1.unwrap().clone();

        let mut added_or_merged_notification = String::new();

        //Prepping army builds folder
        let mut armies = load_army_builds(
            self.insert_folder.folder_string.as_str(),
            &self.selected_game,
        );
        let game_extension = format!(
            ".{}",
            get_ca_game_army_setup_ext(self.selected_game.clone())
        );

        //If loading from CA game folder, have that be dominant file naming system, will always match what you have there.
        if self.load_folder.is_ca_game_folder() {
            //copy over everything
            for a in armies.iter_mut() {
                let mut new_file_path: PathBuf = owaagh_appdata_path.clone();

                new_file_path.push(format!("{}{}", a.file_stem, game_extension).as_str());

                match std::fs::copy(a.file.clone(), new_file_path.clone()) {
                    Ok(_) => a.file = new_file_path,
                    Err(e) => {
                        let err = format!(
                            "load_folder_to_owaagh_appdata Couldn't copy from {} to {} err {}",
                            a.file.to_string_lossy().to_string(),
                            new_file_path.to_string_lossy().to_string(),
                            e
                        );
                        println!("{}", err);
                    }
                }
            }

            let mut n_added = 0;
            match self.army_builds.get_mut(&self.selected_game) {
                None => {
                    let army_set = HashSet::from_iter(armies.iter().cloned());
                    n_added = army_set.len();
                    self.army_builds
                        .insert(self.selected_game.clone(), army_set);
                }
                Some(army_set) => {
                    let n_before = army_set.len();
                    for a in armies {
                        army_set.insert(a); //hashset insert doesn't replace if exists
                    }
                    n_added = army_set.len() - n_before;
                }
            }
            added_or_merged_notification = format!("{} Builds Added", n_added);
            return Ok(added_or_merged_notification);
        } else {
            let game_army_builds = self.get_game_army_builds(self.selected_game.clone());
            let n_before = game_army_builds.len();

            //Copy & Rename loaded army builds
            for a in armies.iter_mut() {
                let mut new_file_path = owaagh_appdata_path.clone();

                //Check unique & rename
                if game_army_builds.contains(a) {
                    match std::fs::metadata(a.file.as_path()) {
                        Ok(m) => {
                            let t = m.created().unwrap_or(std::time::SystemTime::now());
                            let datetime: DateTime<Utc> = t.into();
                            a.file_stem =
                                format!("{} {}", a.file_stem, datetime.format(YMD_HMS_FORMAT));
                        }
                        Err(e) => {
                            return Err(format!("Getting metadata err {}", e));
                        }
                    }
                    new_file_path.push(a.file_stem.as_str());
                    new_file_path.push(game_extension.as_str());
                }

                //copy and add in new army builds
                match std::fs::copy(a.file.clone(), new_file_path.clone()) {
                    Ok(_) => a.file = new_file_path,
                    Err(e) => {
                        let err = format!("{} for file_stem {}", e, a.file_stem);
                        return Err(err);
                    }
                }
            }

            let n_added = game_army_builds.len() - n_before;
            added_or_merged_notification = format!("{} Builds Added", n_added);
            self.army_builds
                .insert(self.selected_game.clone(), game_army_builds);
            //self.army_builds = armies;
            return Ok(added_or_merged_notification);
        }
    }

    pub fn update_display_builds(&mut self) {
        let game_builds = self.get_game_army_builds(self.selected_game.clone());
        //check factions first
        let mut display_builds: Vec<ArmyBuild> = if (self.selected_game == CaGame::Warhammer2) {
            //only implimented faction & vs faction enums for wh2
            game_builds
                .iter()
                .filter(|ab| {
                    (self.search_faction == Wh2Factions::ALL || ab.faction == self.search_faction)
                        && (self.search_vs_faction == Wh2Factions::ALL
                            || ab.vs_faction == self.search_vs_faction)
                })
                .cloned()
                .collect()
        } else {
            game_builds.into_iter().collect()
        };

        //now do string manipulation, slower so on fewer
        let lower_case_search = self.search_string.to_ascii_lowercase();
        display_builds = display_builds
            .iter()
            .filter(|ab| {
                ab.file_stem
                    .to_ascii_lowercase()
                    .contains(lower_case_search.as_str())
            })
            .cloned()
            .collect();

        display_builds.sort();
        self.display_builds = display_builds;
    }

    pub fn update_load_folder(&mut self) {
        let f_string: String;
        match get_ca_game_army_setups_folder(self.selected_game.clone()) {
            Ok(p) => {
                f_string = p.to_string_lossy().to_string();
            }
            Err(_) => {
                return;
            }
        }
        self.load_folder = ArmySetupsFolder::new(f_string.as_str());
    }
    pub fn update_insert_folder(&mut self) {
        let f_string: String;
        match get_ca_game_army_setups_folder(self.selected_game.clone()) {
            Ok(p) => {
                f_string = p.to_string_lossy().to_string();
            }
            Err(_) => {
                return;
            }
        }
        self.insert_folder = ArmySetupsFolder::new(f_string.as_str());
    }

    pub fn selected_game_update(&mut self) {
        self.update_load_folder();
        match self.load_folder_to_owaagh_appdata() {
            Ok(_) => {}
            Err(e) => {
                self.load_folder.set_load_folder_error();
            }
        }
        self.update_display_builds();
        self.update_insert_folder();
    }

    pub fn army_selector_scrolling_table(
        &mut self,
        ui: &mut Ui,
        scroll_top: bool,
        scroll_bottom: bool,
    ) {
        let scroll_area = ScrollArea::from_max_height(200.0);

        ui.separator();
        let (_current_scroll, _max_scroll) = scroll_area.show(ui, |ui| {
            if scroll_top {
                ui.scroll_to_cursor(Align::TOP);
            }
            ui.vertical(|ui| {});
            egui::Grid::new("army_build_body")
                .striped(true)
                .min_col_width(1.0)
                .max_col_width(200.0)
                .show(ui, |ui| {
                    show_army_build_header_row(ui);
                    ui.end_row();
                    for (row, display_build) in self.display_builds.iter_mut().enumerate() {
                        let selected = display_build.show_selectable_army_build_row(
                            ui,
                            self.selected_army_build == *display_build,
                        );

                        if selected {
                            self.selected_army_build = display_build.clone();
                            self.edit_build = display_build.clone();
                            self.selected_display_build_indx = row;
                        }
                        ui.end_row();
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
    }

    fn army_selector_search_section_ui(&mut self, ui: &mut Ui, ctx: &egui::CtxRef) {}

    pub(crate) fn army_selector_scrolling_ui(&mut self, ui: &mut Ui, ctx: &egui::CtxRef) {
        if self.army_builds.is_empty() {
            ui.label("You got to load some armies first");
            return;
        }

        ui.horizontal(|ui| {
            if ui.button("Search").clicked() {
                println!("searching {}", get_ca_game_title(&self.selected_game));
                self.update_display_builds();
            }
            if ui
                .text_edit_singleline(&mut self.search_string)
                .lost_focus()
                && ctx.input().key_pressed(egui::Key::Enter)
            {
                println!(
                    "enter search :) {} todo search & update display function",
                    self.search_string
                );
                self.update_display_builds();
            }

            if self.selected_game == CaGame::Warhammer2 {
                //ui.add(doc_link_label("Combo box", "faction_search"));
                let prior_selected_faction = self.search_faction.clone();
                let faction_btn_response =
                    faction_dropdown_button(ui, &mut self.search_faction, "Faction", false);
                if faction_btn_response.clicked() && prior_selected_faction != self.search_faction {
                    println!("change faction");
                    self.update_display_builds();
                }

                let prior_selected_faction = self.search_vs_faction.clone();
                let vs_faction_btn_response =
                    faction_dropdown_button(ui, &mut self.search_vs_faction, "vs Faction", true);
                if vs_faction_btn_response.clicked()
                    && prior_selected_faction != self.search_vs_faction
                {
                    println!("change vs faction");
                    self.update_display_builds();
                }
            }
        });

        let mut scroll_top = false;
        let mut scroll_bottom = false;

        ui.separator();

        ui.horizontal(|ui| {
            scroll_top |= ui.button("⬆").clicked();
            scroll_bottom |= ui.button("⬇").clicked();
            // if ui.button("🚫").clicked() {
            //     println!("To do delete pop");
            // }
            // if ui.button("🗋").clicked() {
            //     println!("To do delete pop");
            // }
        });

        self.army_selector_scrolling_table(ui, scroll_top, scroll_bottom);
        ui.separator();
    }

    pub fn insert_army(&self) -> Result<(), String> {
        //Check If Inputs Valid
        if !self.insert_folder.is_ca_game_folder() {
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
        if self.selected_display_build_indx > self.display_builds.len() {
            ui.label("You got to select an army first");
            return;
        }

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
            if ui.text_edit_singleline(&mut self.insert_name).lost_focus()
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

    fn edit_section_errorless_str_edit(ui: &mut Ui, title: String, edit_str: &mut String) {
        ui.horizontal(|ui| {
            ui.label(title);
            ui.text_edit_singleline(edit_str);
        });
    }

    fn army_card_image_file_select_ui_row(&mut self, ui: &mut Ui, ctx: &egui::CtxRef) {
        ui.horizontal(|ui| {
            if ui.button("Add Army Image File").clicked() {
                //if  { }
                println!("aa {}", self.insert_folder.folder_string.as_str());

                let params = DialogParams {
                    default_folder: self.insert_folder.folder_string.as_str(),
                    options: FOS_PICKFOLDERS,
                    ..Default::default()
                };

                match wfd::open_dialog(params) {
                    Ok(res) => {
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
    }

    pub fn edit_section_ui(&mut self, ui: &mut Ui, ctx: &egui::CtxRef) {
        let mut has_errors = false;

        for display_col in ArmyBuildDisplayColumns::into_enum_iter() {
            match display_col {
                ArmyBuildDisplayColumns::Name => {
                    ui.horizontal(|ui| {
                        ui.label(get_army_build_display_column_title(&display_col));
                        ui.text_edit_singleline(&mut self.edit_build.file_stem);
                        if self.edit_build.file_stem != self.selected_army_build.file_stem {
                            if self
                                .army_builds
                                .get(&self.selected_game)
                                .expect("edit_section_ui didn't have builds for game")
                                .contains(&self.edit_build)
                            {
                                ui.label("That name already exists, pick another");
                                has_errors = true;
                            }
                        }
                    });
                }
                ArmyBuildDisplayColumns::Faction => {
                    ArmySetupsManager::edit_section_errorless_str_edit(
                        ui,
                        get_army_build_display_column_title(&display_col),
                        &mut self.edit_build.faction_str,
                    )
                }
                ArmyBuildDisplayColumns::VsFaction => {
                    ArmySetupsManager::edit_section_errorless_str_edit(
                        ui,
                        get_army_build_display_column_title(&display_col),
                        &mut self.edit_build.vs_faction_str,
                    )
                }
                ArmyBuildDisplayColumns::Funds => {
                    funds_slider_ui(&mut self.edit_build, ui);
                }
                ArmyBuildDisplayColumns::CreatedBy => {
                    ArmySetupsManager::edit_section_errorless_str_edit(
                        ui,
                        get_army_build_display_column_title(&display_col),
                        &mut self.edit_build.created_by,
                    )
                }
                ArmyBuildDisplayColumns::GameMod => {
                    ArmySetupsManager::edit_section_errorless_str_edit(
                        ui,
                        get_army_build_display_column_title(&display_col),
                        &mut self.edit_build.game_mod,
                    )
                }
                ArmyBuildDisplayColumns::Notes => {
                    ui.label(get_army_build_display_column_title(&display_col));
                    ui.text_edit_multiline(&mut self.edit_build.notes);
                }

                _ => {}
            }
        }
        self.army_card_image_file_select_ui_row(ui, ctx);
        //let mut file
        if !has_errors {
            if ui.button("Apply Edits").clicked() {
                self.selected_army_build = self.edit_build.clone();
            }
        }
    }

    pub fn central_panel_ui(&mut self, ui: &mut Ui, ctx: &egui::CtxRef) {
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
                        .lost_focus()
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

        if self.selected_army_build.file_stem.len() > 0 {
            //file stem is required so
            // egui::CollapsingHeader::new(format!("Edit {}", self.selected_army_build.file_stem))
            //     .default_open(false)
            //     .show(ui, |ui| {
            //         self.edit_section_ui(ui, ctx);
            //     });

            egui::CollapsingHeader::new(format!("Insert {}", self.selected_army_build.file_stem))
                .default_open(self.insert_folder.is_ca_game_folder())
                .show(ui, |ui| {
                    //
                    ui.horizontal(|ui| {
                        if ui.button("Insert Folder").clicked() {
                            self.insert_folder.set_insert_folder_error();
                        }
                        if ui
                            .text_edit_singleline(&mut self.insert_folder.folder_string)
                            .lost_focus()
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

                    if self.insert_folder.is_ca_game_folder() {
                        self.insert_army_ui(ui, ctx);
                    }
                });
        }
    }
}
