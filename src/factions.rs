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
