use crate::ca_game::{get_ca_game_title, CaGame};
use crate::factions::Wh2Factions;
use crate::ymd_hms_dash_format;
use crate::ymd_hms_dash_format::YMD_HMS_FORMAT;
use chrono::{DateTime, Utc};
use eframe::egui;
use eframe::egui::Ui;
use enum_iterator::IntoEnumIterator;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

const MAXFUNDS: u32 = 100000;

#[cfg_attr(
    feature = "persistence",
    derive(serde::Deserialize, serde::Serialize, Clone)
)]
#[derive()]
pub struct ArmyBuild {
    pub file: PathBuf,
    pub file_stem: String, //also acts as display name
    pub original_file: PathBuf,

    #[serde(with = "ymd_hms_dash_format")]
    pub created_on: chrono::DateTime<Utc>,

    pub faction: Wh2Factions,
    pub vs_faction: Wh2Factions,

    pub ca_game: CaGame,
    pub game_mod: String,
    pub created_by: String,

    pub faction_str: String,    // for display, nonwarhammer2 games, & mods
    pub vs_faction_str: String, // for display, nonwarhammer2 games, & mods

    pub win_count: u32,
    pub loss_count: u32,

    pub funds: u32,

    pub image_files: Vec<PathBuf>,
    pub notes: String,
}

#[cfg_attr(
    feature = "persistence",
    derive(
        serde::Deserialize,
        serde::Serialize,
        Debug,
        Clone,
        PartialEq,
        IntoEnumIterator
    )
)]
pub enum ArmyBuildDisplayColumns {
    Name,
    Faction,
    VsFaction,
    Funds,
    CreatedBy,
    GameMod,
    WinPercent,
    NumGamesPlayed,
    CreatedOn,
    Notes,
}

pub fn get_army_build_display_column_title(display_col: &ArmyBuildDisplayColumns) -> String {
    match display_col {
        ArmyBuildDisplayColumns::Name => "Name".to_string(),
        ArmyBuildDisplayColumns::Faction => "Faction".to_string(),
        ArmyBuildDisplayColumns::VsFaction => "Vs Faction".to_string(),
        ArmyBuildDisplayColumns::Funds => "Funds".to_string(),
        ArmyBuildDisplayColumns::CreatedBy => "Created By".to_string(),
        ArmyBuildDisplayColumns::GameMod => "Mod".to_string(),
        ArmyBuildDisplayColumns::WinPercent => "% W".to_string(),
        ArmyBuildDisplayColumns::NumGamesPlayed => "Played".to_string(),
        ArmyBuildDisplayColumns::CreatedOn => "Date Created".to_string(),
        ArmyBuildDisplayColumns::Notes => "Notes".to_string(),
    }
}

pub fn show_army_build_header_row(ui: &mut Ui) {
    for display_col in ArmyBuildDisplayColumns::into_enum_iter() {
        ui.label(get_army_build_display_column_title(&display_col));
    }
}

impl PartialEq for ArmyBuild {
    fn eq(&self, other: &Self) -> bool {
        self.file_stem == other.file_stem
    }
}

impl Eq for ArmyBuild {}

impl Hash for ArmyBuild {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file_stem.hash(state);
    }
}

impl Default for ArmyBuild {
    fn default() -> Self {
        Self {
            file: PathBuf::new(),
            file_stem: String::new(),
            faction: Wh2Factions::UNKNOWN,
            vs_faction: Wh2Factions::UNKNOWN,
            created_on: std::time::SystemTime::now().into(),
            original_file: PathBuf::new(),
            ca_game: CaGame::Warhammer2,
            game_mod: String::new(),
            created_by: String::new(),
            faction_str: String::new(),
            vs_faction_str: String::new(),
            win_count: 0,
            loss_count: 0,
            funds: 12400,
            image_files: vec![],
            notes: String::new(),
        }
    }
}

impl ArmyBuild {
    pub fn get_win_percent(&self) -> f64 {
        self.win_count as f64 / self.loss_count as f64
    }
    pub fn get_games_played_count(&self) -> u32 {
        self.win_count + self.loss_count
    }

    pub fn get_display_string(&self, display_col: &ArmyBuildDisplayColumns) -> String {
        match display_col {
            ArmyBuildDisplayColumns::Name => self.file_stem.clone(),
            ArmyBuildDisplayColumns::Faction => self.faction_str.clone(),
            ArmyBuildDisplayColumns::Funds => format!("{}", self.funds),
            ArmyBuildDisplayColumns::VsFaction => self.vs_faction_str.clone(),
            ArmyBuildDisplayColumns::CreatedBy => self.created_by.clone(),
            ArmyBuildDisplayColumns::GameMod => self.game_mod.clone(),
            ArmyBuildDisplayColumns::WinPercent => {
                let win_percent = self.get_win_percent();
                if !win_percent.is_nan() {
                    format!("{:.1}", win_percent)
                } else {
                    "".to_string()
                }
            }
            ArmyBuildDisplayColumns::NumGamesPlayed => {
                format!("{}", self.win_count + self.loss_count)
            }
            ArmyBuildDisplayColumns::CreatedOn => {
                format!("{}", self.created_on.format(YMD_HMS_FORMAT))
            }
            ArmyBuildDisplayColumns::Notes => "...".to_string(),
        }
    }

    //returns if user clicked
    pub fn show_selectable_army_build_row(&mut self, ui: &mut Ui, selected_row: bool) -> bool {
        let mut clicked = false;
        for display_col in ArmyBuildDisplayColumns::into_enum_iter() {
            match display_col {
                ArmyBuildDisplayColumns::Notes => {
                    if ui.button(self.get_display_string(&display_col)).clicked() {
                        println!("TODO popup of notes");
                    }
                }

                _ => {
                    if (ui.selectable_label(selected_row, self.get_display_string(&display_col)))
                        .clicked()
                    {
                        clicked = true;
                    }
                }
            }
        }
        clicked
    }
}

#[cfg_attr(
    feature = "persistence",
    derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)
)]
pub enum FundsLevel {
    Small,
    Medium,
    Large,
    Ultra,
    Custom,
}

pub fn get_funds_amount(funds_level: FundsLevel, ca_game: &CaGame) -> u32 {
    match ca_game {
        _ => match funds_level {
            FundsLevel::Small => 5000,
            FundsLevel::Medium => 8600,
            FundsLevel::Large => 12400,
            FundsLevel::Ultra => 17000,
            _ => 0,
        },
    }
}

pub fn get_funds_level(funds: u32, ca_game: &CaGame) -> FundsLevel {
    match ca_game {
        _ => match funds {
            5000 => FundsLevel::Small,
            8600 => FundsLevel::Medium,
            12400 => FundsLevel::Large,
            17000 => FundsLevel::Ultra,
            _ => FundsLevel::Custom,
        },
    }
}

pub fn funds_slider_ui(army_build: &mut ArmyBuild, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label("Funds");
        ui.add(egui::Slider::new(&mut army_build.funds, 0..=MAXFUNDS));

        //TODO check funds int value
        let mut funds_level = get_funds_level(army_build.funds, &army_build.ca_game);

        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", funds_level))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut funds_level, FundsLevel::Small, "Small");
                ui.selectable_value(&mut funds_level, FundsLevel::Medium, "Medium");
                ui.selectable_value(&mut funds_level, FundsLevel::Large, "Large");
                ui.selectable_value(&mut funds_level, FundsLevel::Ultra, "Ultra");
            });
        if funds_level != FundsLevel::Custom {
            army_build.funds = get_funds_amount(funds_level, &army_build.ca_game);
        }
    });
}
