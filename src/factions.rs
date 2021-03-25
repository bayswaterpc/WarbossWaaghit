use eframe::egui;
use eframe::egui::{Response, Ui};

#[cfg_attr(
    feature = "persistence",
    derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)
)]
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
    ALL,
}

pub fn get_faction_abbreviations(faction: Faction) -> &'static str {
    match faction {
        Faction::BM => "BM",
        Faction::BRT => "BRT",
        Faction::CH => "CH",
        Faction::DE => "DE",
        Faction::DW => "DW",
        Faction::EMP => "EMP",
        Faction::GS => "GS",
        Faction::HE => "HE",
        Faction::LM => "LM",
        Faction::NRS => "NRS",
        Faction::SKV => "SKV",
        Faction::TK => "TK",
        Faction::VC => "VC",
        Faction::VP => "VP",
        Faction::WE => "WE",
        Faction::UNKNOWN => "UNKNOWN",
        Faction::ALL => "ALL",
    }
}

pub fn _get_faction_names(faction: Faction) -> &'static str {
    match faction {
        Faction::BM => "Beastmen",
        Faction::BRT => "Bretonnia",
        Faction::CH => "Chaos",
        Faction::DE => "Dark Elves",
        Faction::DW => "Dwarfs",
        Faction::EMP => "Empire",
        Faction::GS => "Greenskins",
        Faction::HE => "High Elves",
        Faction::LM => "Lizardmen",
        Faction::NRS => "Norsca",
        Faction::SKV => "Skaven",
        Faction::TK => "Tomb Kings",
        Faction::VC => "Vampire Counts",
        Faction::VP => "Vampire Coast",
        Faction::WE => "Woodelves",
        Faction::UNKNOWN => "Unknown",
        Faction::ALL => "All",
    }
}

pub fn parse_faction(file_name: &String) -> Faction {
    let lower_file = file_name.to_ascii_lowercase();
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
    } else if lower_file.contains("gs vs") {
        return Faction::GS;
    } else if lower_file.contains("he vs") {
        return Faction::HE;
    } else if lower_file.contains("lm vs") {
        return Faction::LM;
    } else if lower_file.contains("nrs vs") {
        return Faction::NRS;
    } else if lower_file.contains("skv vs") {
        return Faction::SKV;
    } else if lower_file.contains("tk vs") {
        return Faction::TK;
    } else if lower_file.contains("vc vs") {
        return Faction::VC;
    } else if lower_file.contains("vp vs") {
        return Faction::VP;
    } else if lower_file.contains("we vs") {
        return Faction::WE;
    }
    Faction::UNKNOWN
}

pub fn parse_vs_faction(file_name: &String) -> Faction {
    let lower_file = file_name.to_ascii_lowercase();
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
    } else if lower_file.contains("vs lm") {
        return Faction::LM;
    } else if lower_file.contains("vs nrs") {
        return Faction::NRS;
    } else if lower_file.contains("vs skv") {
        return Faction::SKV;
    } else if lower_file.contains("vs tk") {
        return Faction::TK;
    } else if lower_file.contains("vs vc") {
        return Faction::VC;
    } else if lower_file.contains("vs vp") {
        return Faction::VP;
    } else if lower_file.contains("vs we") {
        return Faction::WE;
    } else if lower_file.contains("vs aa") {
        return Faction::ALL;
    }
    Faction::UNKNOWN
}

pub fn faction_dropdown_button(
    ui: &mut Ui,
    faction: &mut Faction,
    label: &str,
    is_vs: bool,
) -> Response {
    let faction_btn_response =
        egui::combo_box_with_label(ui, label, format!("{:?}", faction), |ui| {
            ui.selectable_value(
                faction,
                Faction::ALL,
                get_faction_abbreviations(Faction::ALL),
            );
            ui.selectable_value(faction, Faction::BM, get_faction_abbreviations(Faction::BM));
            ui.selectable_value(
                faction,
                Faction::BRT,
                get_faction_abbreviations(Faction::BRT),
            );
            if !is_vs {
                ui.selectable_value(faction, Faction::CH, get_faction_abbreviations(Faction::CH));
            }
            ui.selectable_value(faction, Faction::DE, get_faction_abbreviations(Faction::DE));
            ui.selectable_value(faction, Faction::DW, get_faction_abbreviations(Faction::DW));
            ui.selectable_value(
                faction,
                Faction::EMP,
                get_faction_abbreviations(Faction::EMP),
            );
            ui.selectable_value(faction, Faction::GS, get_faction_abbreviations(Faction::GS));
            ui.selectable_value(faction, Faction::HE, get_faction_abbreviations(Faction::HE));
            ui.selectable_value(faction, Faction::LM, get_faction_abbreviations(Faction::LM));
            ui.selectable_value(
                faction,
                Faction::NRS,
                get_faction_abbreviations(Faction::NRS),
            );
            ui.selectable_value(
                faction,
                Faction::SKV,
                get_faction_abbreviations(Faction::SKV),
            );
            ui.selectable_value(faction, Faction::TK, get_faction_abbreviations(Faction::TK));
            ui.selectable_value(faction, Faction::VC, get_faction_abbreviations(Faction::VC));
            ui.selectable_value(faction, Faction::VP, get_faction_abbreviations(Faction::VP));
            ui.selectable_value(faction, Faction::WE, get_faction_abbreviations(Faction::WE));
            ui.selectable_value(faction, Faction::WE, get_faction_abbreviations(Faction::WE));
            ui.selectable_value(
                faction,
                Faction::UNKNOWN,
                get_faction_abbreviations(Faction::UNKNOWN),
            );
        });
    faction_btn_response
}
