use eframe::egui;
use eframe::egui::{Ui};
use egui::{color::*, *};

const YOUTUBERS: [(&str, &str); 17] =
    [   ("A Rubber Duck Of War", "https://www.youtube.com/channel/UC2Ar9ggXnOj3SkNv9uC7TiA"),
        ("Colonel Damneders", "https://www.youtube.com/watch?v=g2-9XioDOd4"),
        ("DahvPlays", "https://www.youtube.com/channel/UC6CLkxOWW-TxdGM9yVTyDfQ"),
        ("Enticity", "https://www.twitch.tv/enticity"),
        ("Felkon", "https://www.youtube.com/watch?v=e4ffnDfdD0I"),
        ("Italian Spartacus", "https://www.youtube.com/channel/UCUl_adPzC1h8--tD1Npay8Q"),
        ("Gudgitz Twitch", "https://www.twitch.tv/gudgitz"),
        ("Gudgitz YouTube", "https://www.youtube.com/channel/UCkiGutcR5kCJTbhl7xwQhhA"),
        ("Hier of Carthage", "https://www.youtube.com/user/HeirofCarthage/videos"),
        ("Loremaster of Sotek", "https://www.youtube.com/channel/UCH4nPsl2ctS365aEfFBwxbg"),
        ("Lotus_Moon", "https://www.twitch.tv/lotus_moon_"),
        ("Melkor", "https://www.youtube.com/c/MelkorGG/videos"),
        ("Milk and Cookies TW", "https://www.youtube.com/user/milkandcookiesTW"),
        ("My_Son_HW", "https://www.twitch.tv/my_son_hw"),
        ("The Great Book of Grudges", "https://www.youtube.com/channel/UCNxHDiW6i68RRzt4GQsbigQ"),
        ("Turin", "https://www.youtube.com/channel/UCNDJiDFJWaiKktyUBmVzGYA"),
        ("Zerkovich", "https://www.youtube.com/channel/UCWqIHkxwNkVTWal2mQbZn0Q")
    ];

const MISC_RESOURCES: [(&str, &str); 7] =
    [
        ("Warboss Waaghit Tutorial", "https://www.youtube.com/channel/UCkiGutcR5kCJTbhl7xwQhhA"),
        ("Dahv Plays Tier List", "https://docs.google.com/spreadsheets/d/10AY5xvoNOQUdIqw65thtcXKaTrYwj1gUu8MMHzJQ6NI/edit#gid=0"),
        ("The Felkon Build Guide", "https://www.youtube.com/watch?v=e4ffnDfdD0I"),
        ("Banner Rules", "https://drive.google.com/file/d/1blvwkYSM3l7KW2Zf6t2Nra4eY7MSjP4d/view"),
        ("Enticity Ranked Ladder Discord", "https://discord.com/invite/wERjDN3"),
        ("Dedicated Tournament Discord", "https://discord.gg/wPmg5n4"),
        ("twwstats.com", "https://twwstats.com/")
    ];


const CODING_RESOURCES: [(&str, &str); 6] =
    [
        ("Source Code Github", "https://github.com/bayswaterpc/WarbossWaaghit"),
        ("Future Development Github", "https://github.com/bayswaterpc/owaagh"),
        ("The Rust Programming Language", "https://doc.rust-lang.org/book/"),
        ("Getting Started With Rust on Windows", "https://docs.microsoft.com/en-us/windows/dev-environment/rust/setup"),
        ("Reddit Rust", "https://www.reddit.com/r/rust/"),
        ("Egui Project", "https://github.com/emilk/egui"),
    ];

const GUD_GITZ_LINKZ: [(&str, &str); 6] =
    [
        ("Source Code Github", "https://github.com/bayswaterpc/WarbossWaaghit"),
        ("Future Development Github", "https://github.com/bayswaterpc/owaagh"),
        ("The Rust Programming Language", "https://doc.rust-lang.org/book/"),
        ("Getting Started With Rust on Windows", "https://docs.microsoft.com/en-us/windows/dev-environment/rust/setup"),
        ("Reddit Rust", "https://www.reddit.com/r/rust/"),
        ("Egui Project", "https://github.com/emilk/egui"),
    ];



pub fn central_panel_ui(ui: &mut Ui, ctx: &egui::CtxRef) {
    for label_link in MISC_RESOURCES.iter() {
        ui.horizontal(|ui| {
            ui.label(label_link.0);
            ui.hyperlink(label_link.1);
        });
    }

    egui::CollapsingHeader::new("Twitch & Youtubers")
        .default_open(true)
        .show(ui, |ui| {
            for label_link in YOUTUBERS.iter() {
                ui.horizontal(|ui| {
                    ui.label(label_link.0);
                    ui.hyperlink(label_link.1);
                });
            }
        });

    egui::CollapsingHeader::new("Mekboy Stuff")
        .default_open(true)
        .show(ui, |ui| {

            ui.horizontal_wrapped(|ui| {
                // Trick so we don't have to add spaces in the text below:
                ui.spacing_mut().item_spacing.x = ui.fonts()[TextStyle::Body].glyph_width(' ');
                ui.label("This project was written entirely in Rust, is open sourced MIT licensed, and can be found at one of the links below");
            });


            for label_link in CODING_RESOURCES.iter() {
                ui.horizontal(|ui| {
                    ui.label(label_link.0);
                    ui.hyperlink(label_link.1);
                });
            }

        });

    egui::CollapsingHeader::new("Future Development")
        .default_open(true)
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                // Trick so we don't have to add spaces in the text below:
                ui.label("Hello I'm Gudgitz the creator of the project, thanks for checking this out.  I'm done with development on the Warboss Waaghit.  This project is what wez would call a case of");
                ui.add(Label::new("GDD").text_color(Color32::from_rgb(0,100,0)));
                ui.label("Gork Driven Development. I said WAAAAAAGH and started mashing at keyboard.");
                ui.label("It was a good scrap, but needs more cunning.  Next time we'ze going to do ");
                ui.add(Label::new("GMDD").text_color(Color32::from_rgb(0,100,0)));
                ui.label("Gork & Mork Driven Development.");
            });

            ui.horizontal_wrapped(|ui| {
                // Trick so we don't have to add spaces in the text below:
                ui.label("The new plan is");
                ui.colored_label(Color32::from_rgb(0, 200, 0), "Operation Whiskey Alpha Alpha Golf Hotel");
                ui.label("The plan is to track replays, link them to your army builds, record win loss, and more");
                ui.label("Follow me on youtube & twitch for updates.");
            });


            ui.horizontal_wrapped(|ui| {
                // Trick so we don't have to add spaces in the text below:
                ui.spacing_mut().item_spacing.x = ui.fonts()[TextStyle::Body].glyph_width(' ');
                ui.label("In the meantime feel free to make a fork of this, play around wiff it maybe youz can makes it even ");
                ui.colored_label(Color32::from_rgb(150, 0, 0), "Meaner"); // Shortcut version
                ui.label("and");
                ui.colored_label(Color32::from_rgb(0, 255, 0), "Greener"); // Shortcut version
            });
        });

}