use eframe::egui;
use eframe::egui::{Response, Ui};

#[cfg_attr(
    feature = "persistence",
    derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)
)]
pub enum Wh2Factions {
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
    ALL,
}

pub fn get_faction_abbreviations(faction: Wh2Factions) -> &'static str {
    match faction {
        Wh2Factions::BM => "BM",
        Wh2Factions::BRT => "BRT",
        Wh2Factions::CH => "CH",
        Wh2Factions::DE => "DE",
        Wh2Factions::DW => "DW",
        Wh2Factions::EMP => "EMP",
        Wh2Factions::GS => "GS",
        Wh2Factions::HE => "HE",
        Wh2Factions::LM => "LM",
        Wh2Factions::NRS => "NRS",
        Wh2Factions::SKV => "SKV",
        Wh2Factions::TK => "TK",
        Wh2Factions::VC => "VC",
        Wh2Factions::VP => "VP",
        Wh2Factions::WE => "WE",
        Wh2Factions::UNKNOWN => "UNKNOWN",
        Wh2Factions::ALL => "ALL",
    }
}

pub fn get_faction_names(faction: &Wh2Factions) -> &'static str {
    match faction {
        Wh2Factions::BM => "Beastmen",
        Wh2Factions::BRT => "Bretonnia",
        Wh2Factions::CH => "Chaos",
        Wh2Factions::DE => "Dark Elves",
        Wh2Factions::DW => "Dwarfs",
        Wh2Factions::EMP => "Empire",
        Wh2Factions::GS => "Greenskins",
        Wh2Factions::HE => "High Elves",
        Wh2Factions::LM => "Lizardmen",
        Wh2Factions::NRS => "Norsca",
        Wh2Factions::SKV => "Skaven",
        Wh2Factions::TK => "Tomb Kings",
        Wh2Factions::VC => "Vampire Counts",
        Wh2Factions::VP => "Vampire Coast",
        Wh2Factions::WE => "Woodelves",
        Wh2Factions::UNKNOWN => "Unknown",
        Wh2Factions::ALL => "All",
    }
}

pub fn parse_faction(file_name: &String) -> Wh2Factions {
    let lower_file = file_name.to_ascii_lowercase();
    if lower_file.contains("bm vs") {
        return Wh2Factions::BM;
    } else if lower_file.contains("brt vs") {
        return Wh2Factions::BRT;
    } else if lower_file.contains("ch vs") {
        return Wh2Factions::CH;
    } else if lower_file.contains("dw vs") {
        return Wh2Factions::DW;
    } else if lower_file.contains("emp vs") {
        return Wh2Factions::EMP;
    } else if lower_file.contains("ew vs") {
        return Wh2Factions::DW;
    } else if lower_file.contains("de vs") {
        return Wh2Factions::DE;
    } else if lower_file.contains("gs vs") {
        return Wh2Factions::GS;
    } else if lower_file.contains("he vs") {
        return Wh2Factions::HE;
    } else if lower_file.contains("lm vs") {
        return Wh2Factions::LM;
    } else if lower_file.contains("nrs vs") {
        return Wh2Factions::NRS;
    } else if lower_file.contains("skv vs") {
        return Wh2Factions::SKV;
    } else if lower_file.contains("tk vs") {
        return Wh2Factions::TK;
    } else if lower_file.contains("vc vs") {
        return Wh2Factions::VC;
    } else if lower_file.contains("vp vs") {
        return Wh2Factions::VP;
    } else if lower_file.contains("we vs") {
        return Wh2Factions::WE;
    }
    Wh2Factions::UNKNOWN
}

pub fn parse_vs_faction(file_name: &String) -> Wh2Factions {
    let lower_file = file_name.to_ascii_lowercase();
    if lower_file.contains("vs bm") {
        return Wh2Factions::BM;
    } else if lower_file.contains("vs brt") {
        return Wh2Factions::BRT;
    } else if lower_file.contains("vs ch") {
        return Wh2Factions::CH;
    } else if lower_file.contains("vs de") {
        return Wh2Factions::DE;
    } else if lower_file.contains("vs dw") {
        return Wh2Factions::DW;
    } else if lower_file.contains("vs emp") {
        return Wh2Factions::EMP;
    } else if lower_file.contains("vs gs") {
        return Wh2Factions::GS;
    } else if lower_file.contains("vs he") {
        return Wh2Factions::HE;
    } else if lower_file.contains("vs lm") {
        return Wh2Factions::LM;
    } else if lower_file.contains("vs nrs") {
        return Wh2Factions::NRS;
    } else if lower_file.contains("vs skv") {
        return Wh2Factions::SKV;
    } else if lower_file.contains("vs tk") {
        return Wh2Factions::TK;
    } else if lower_file.contains("vs vc") {
        return Wh2Factions::VC;
    } else if lower_file.contains("vs vp") {
        return Wh2Factions::VP;
    } else if lower_file.contains("vs we") {
        return Wh2Factions::WE;
    } else if lower_file.contains("vs aa") {
        return Wh2Factions::ALL;
    }
    Wh2Factions::UNKNOWN
}

pub fn faction_dropdown_button(
    ui: &mut Ui,
    faction: &mut Wh2Factions,
    label: &str,
    is_vs: bool,
) -> Response {
    let faction_btn_response = egui::ComboBox::from_label(label)
        .selected_text(format!("{:?}", faction))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                faction,
                Wh2Factions::ALL,
                get_faction_abbreviations(Wh2Factions::ALL),
            );
            ui.selectable_value(
                faction,
                Wh2Factions::BM,
                get_faction_abbreviations(Wh2Factions::BM),
            );
            ui.selectable_value(
                faction,
                Wh2Factions::BRT,
                get_faction_abbreviations(Wh2Factions::BRT),
            );
            if !is_vs {
                ui.selectable_value(
                    faction,
                    Wh2Factions::CH,
                    get_faction_abbreviations(Wh2Factions::CH),
                );
            }
            ui.selectable_value(
                faction,
                Wh2Factions::DE,
                get_faction_abbreviations(Wh2Factions::DE),
            );
            ui.selectable_value(
                faction,
                Wh2Factions::DW,
                get_faction_abbreviations(Wh2Factions::DW),
            );
            ui.selectable_value(
                faction,
                Wh2Factions::EMP,
                get_faction_abbreviations(Wh2Factions::EMP),
            );
            ui.selectable_value(
                faction,
                Wh2Factions::GS,
                get_faction_abbreviations(Wh2Factions::GS),
            );
            ui.selectable_value(
                faction,
                Wh2Factions::HE,
                get_faction_abbreviations(Wh2Factions::HE),
            );
            ui.selectable_value(
                faction,
                Wh2Factions::LM,
                get_faction_abbreviations(Wh2Factions::LM),
            );
            ui.selectable_value(
                faction,
                Wh2Factions::NRS,
                get_faction_abbreviations(Wh2Factions::NRS),
            );
            ui.selectable_value(
                faction,
                Wh2Factions::SKV,
                get_faction_abbreviations(Wh2Factions::SKV),
            );
            ui.selectable_value(
                faction,
                Wh2Factions::TK,
                get_faction_abbreviations(Wh2Factions::TK),
            );
            ui.selectable_value(
                faction,
                Wh2Factions::VC,
                get_faction_abbreviations(Wh2Factions::VC),
            );
            ui.selectable_value(
                faction,
                Wh2Factions::VP,
                get_faction_abbreviations(Wh2Factions::VP),
            );
            ui.selectable_value(
                faction,
                Wh2Factions::WE,
                get_faction_abbreviations(Wh2Factions::WE),
            );
            ui.selectable_value(
                faction,
                Wh2Factions::WE,
                get_faction_abbreviations(Wh2Factions::WE),
            );
            ui.selectable_value(
                faction,
                Wh2Factions::UNKNOWN,
                get_faction_abbreviations(Wh2Factions::UNKNOWN),
            );
        });
    faction_btn_response
}
