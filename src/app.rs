use eframe::{egui, epi};

use crate::army_setups_manager::ArmySetupsManager;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct WarbossWaaghitApp {
    army_setups_manager: ArmySetupsManager,
}

impl Default for WarbossWaaghitApp {
    fn default() -> Self {
        Self {
            army_setups_manager: Default::default(),
        }
    }
}

//Git note on debugging
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
            army_setups_manager,
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
                    });

                    egui::CollapsingHeader::new("Hint")
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.label("This can be any folder with a \'.army_setup\' file");
                            ui.label("The default army setup save folder can be found in your TWW2 Roaming AppData folder ex:");
                            ui.label("C:\\Users\\DaBiggestBoss\\AppData\\Roaming\\The Creative Assembly\\Warhammer2\\army_setups");
                        });
                });




            egui::CollapsingHeader::new("Select Army Setup").default_open(false).show(ui, |ui| {
                if army_setups_manager.army_builds.is_empty() {
                    ui.label("You got to load some armies first");
                }else {
                    army_setups_manager.army_selector_scrolling_ui(ui, ctx);
                }
            });


            egui::CollapsingHeader::new("Insert Army Setup").default_open(false).show(ui, |ui| {
                army_setups_manager.insert_army_ui(ui, ctx);
            });
        });
    }
}

// ----------------------------------------------------------------------------
