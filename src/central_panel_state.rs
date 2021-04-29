use crate::ca_game::CaGame::Warhammer2;
use crate::ca_game::{get_ca_game_title, CaGame};
use eframe::egui;
use eframe::egui::{Color32, Ui};
use enum_iterator::IntoEnumIterator;

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
pub enum CentralPanelState {
    OwaaghSettings,
    GameSelection,
    BuildManager,
    TierList,
    Replays,
    Resources,
    Acknowledgements,
}

#[cfg_attr(
    feature = "persistence",
    derive(serde::Deserialize, serde::Serialize, Clone)
)]
#[derive(Debug)]
pub struct AppState {
    pub ca_game: CaGame,
    pub central_panel_state: CentralPanelState,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            ca_game: CaGame::Warhammer2,
            central_panel_state: CentralPanelState::GameSelection,
        }
    }
}

impl AppState {
    pub fn get_side_bar_title(&self, central_panel_state: &CentralPanelState) -> String {
        match central_panel_state {
            CentralPanelState::OwaaghSettings => "Settings".to_string(),
            CentralPanelState::GameSelection => get_ca_game_title(&self.ca_game),
            CentralPanelState::BuildManager => "Build Boss".to_string(),
            CentralPanelState::TierList => "Tier Lists".to_string(),
            CentralPanelState::Replays => "Replays".to_string(),
            CentralPanelState::Resources => "Resources".to_string(),
            CentralPanelState::Acknowledgements => "Acknowledgments".to_string(),
        }
    }

    pub fn side_bar_ui(&mut self, ui: &mut Ui, ctx: &egui::CtxRef) {
        for central_panel_state in CentralPanelState::into_enum_iter() {
            //let response = ui.label(self.get_side_bar_title(&central_panel_state));
            //if response.clicked() {
            if ui
                .selectable_label(
                    central_panel_state == self.central_panel_state,
                    self.get_side_bar_title(&central_panel_state),
                )
                .clicked()
            {
                self.central_panel_state = central_panel_state;
            }
        }
    }
}
