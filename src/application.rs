/* application.rs
 *
 * Copyright 2024 Eric Lesslie
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use gtk::prelude::*;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::config::VERSION;
use crate::DiceWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct DiceApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for DiceApplication {
        const NAME: &'static str = "DiceApplication";
        type Type = super::DiceApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for DiceApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
            obj.set_accels_for_action("win.roll-d4", &["4"]);
            obj.set_accels_for_action("win.roll-d6", &["6"]);
            obj.set_accels_for_action("win.roll-d8", &["8"]);
            obj.set_accels_for_action("win.roll-d10", &["0"]);
            obj.set_accels_for_action("win.roll-d12", &["<primary>2"]);
            obj.set_accels_for_action("win.roll-d20", &["<primary>0"]);
            obj.set_accels_for_action("win.reroll", &["r"]);
            obj.set_accels_for_action("win.clear", &["c"]);
        }
    }

    impl ApplicationImpl for DiceApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = DiceWindow::new(&*application);
                window.upcast()
            };

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for DiceApplication {}
    impl AdwApplicationImpl for DiceApplication {}
}

glib::wrapper! {
    pub struct DiceApplication(ObjectSubclass<imp::DiceApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl DiceApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        let preferences_action = gio::ActionEntry::builder("preferences")
            .activate(move |app: &Self, _, _| app.show_preferences())
            .build();
        self.add_action_entries([quit_action, about_action, preferences_action]);
    }

    fn show_preferences(&self) {
        let dialog = crate::preferences::build_preferences_dialog();
        dialog.present(Some(&self.active_window().unwrap()));
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutDialog::builder()
            .application_name("dice")
            .application_icon("org.lesslie.dice")
            .developer_name("Eric Lesslie")
            .version(VERSION)
            .developers(vec!["Eric Lesslie"])
            .copyright("© 2024 Eric Lesslie")
            .build();

        about.present(Some(&window));
    }
}
