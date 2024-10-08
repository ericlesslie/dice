/* window.rs
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
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::dice_area::DiceArea;

use std::time::Duration;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/lesslie/dice/window.ui")]
    pub struct DiceWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub dice_area: TemplateChild<DiceArea>,
        #[template_child]
        pub four_side: TemplateChild<gtk::Button>,
        #[template_child]
        pub six_side: TemplateChild<gtk::Button>,
        #[template_child]
        pub eight_side: TemplateChild<gtk::Button>,
        #[template_child]
        pub twelve_side: TemplateChild<gtk::Button>,
        #[template_child]
        pub twenty_side: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DiceWindow {
        const NAME: &'static str = "DiceWindow";
        type Type = super::DiceWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DiceWindow {
        fn constructed(&self) {
            self.parent_constructed();

            self.dice_area.set_has_depth_buffer(true);
        }
    }
    impl WidgetImpl for DiceWindow {}
    impl WindowImpl for DiceWindow {}
    impl ApplicationWindowImpl for DiceWindow {}
    impl AdwApplicationWindowImpl for DiceWindow {}
}

glib::wrapper! {
    pub struct DiceWindow(ObjectSubclass<imp::DiceWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

#[gtk::template_callbacks]
impl DiceWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    #[template_callback]
    fn handle_four_clicked(&self) {
        println!("Four clicked");
        let imp = &self.imp();
        imp.dice_area.add_four();
    }

    #[template_callback]
    fn handle_six_clicked(&self) {
        println!("Six clicked");
        let imp = &self.imp();
        imp.dice_area.add_six();
    }

    #[template_callback]
    fn handle_eight_clicked(&self) {
        println!("Eight clicked");
        let imp = &self.imp();
        imp.dice_area.add_eight();
    }

    #[template_callback]
    fn handle_ten_clicked(&self) {
        println!("Ten clicked");
        let imp = &self.imp();
        imp.dice_area.add_ten();
    }

    #[template_callback]
    fn handle_twelve_clicked(&self) {
        println!("Twelve clicked");
        let imp = &self.imp();
        imp.dice_area.add_twelve();
    }

    #[template_callback]
    fn handle_twenty_clicked(&self) {
        println!("Twenty clicked");
        let imp = &self.imp();
        imp.dice_area.add_twenty();
    }
}
