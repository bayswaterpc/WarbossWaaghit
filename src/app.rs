use crate::army_setups_manager::ArmySetupsManager;
use crate::ca_game::{get_ca_game_title, GameSelector};
use crate::central_panel_state::{AppState, CentralPanelState};
use eframe::{egui, epi};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct OwaaghApp {
    app_state: AppState,
    pub army_setups_manager: ArmySetupsManager,
    game_selector: GameSelector,
}

impl Default for OwaaghApp {
    fn default() -> Self {
        Self {
            army_setups_manager: Default::default(),
            app_state: Default::default(),
            game_selector: Default::default(),
        }
    }
}

//Git note on debugging
impl epi::App for OwaaghApp {
    fn name(&self) -> &str {
        "OWAAGH"
    }

    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn load(&mut self, storage: &dyn epi::Storage) {
        println!("persistence load happens??");
        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        println!("persistence save happens??");
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let OwaaghApp {
            army_setups_manager,
            app_state,
            game_selector,
        } = self;

        egui::SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
            app_state.side_bar_ui(ui, ctx);

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

        egui::CentralPanel::default().show(ctx, |ui| match app_state.central_panel_state.clone() {
            CentralPanelState::OwaaghSettings => {
                ui.label("To Do");
            }
            CentralPanelState::GameSelection => {
                game_selector.central_panel_ui(ui, army_setups_manager, app_state);
            }
            CentralPanelState::BuildManager => army_setups_manager.central_panel_ui(ui, ctx),
            CentralPanelState::TierList => {
                ui.label("Greenskins da Best");
            }
            CentralPanelState::Replays => {
                ui.label("To Do");
            }
            CentralPanelState::Resources => {
                ui.label("To Do");
            }
            CentralPanelState::Acknowledgements => {
                ui.label("To Do");
            }
        });
    }
}

// ----------------------------------------------------------------------------
