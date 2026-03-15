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
use gtk::{gdk, gio, glib};

use crate::dice_area::DiceArea;

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
        pub dice_overlay: TemplateChild<gtk::Overlay>,
        #[template_child]
        pub dice_labels: TemplateChild<gtk::Fixed>,
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

            let dice_area = self.dice_area.clone();
            let dice_labels = self.dice_labels.clone();
            self.obj().add_tick_callback(move |_widget, _clock| {
                // Remove old labels
                let mut child = dice_labels.first_child();
                while let Some(c) = child {
                    child = c.next_sibling();
                    dice_labels.remove(&c);
                }

                // Add labels for settled dice
                let infos = dice_area.settled_dice_info();
                for (wx, wy, val) in infos {
                    let label = gtk::Label::new(Some(&val.to_string()));
                    label.add_css_class("die-number");
                    label.set_can_target(false);
                    let (_, nat_w, _, _) = label.measure(gtk::Orientation::Horizontal, -1);
                    let (_, nat_h, _, _) = label.measure(gtk::Orientation::Vertical, -1);
                    dice_labels.put(&label, (wx - nat_w as f32 / 2.0) as f64, (wy - nat_h as f32 / 2.0) as f64);
                }

                glib::ControlFlow::Continue
            });

            let css = gtk::CssProvider::new();
            css.load_from_data(
                ".die-number { font-size: 24px; font-weight: bold; color: white; text-shadow: 0 1px 3px rgba(0,0,0,0.8); }",
            );
            gtk::style_context_add_provider_for_display(
                &gdk::Display::default().unwrap(),
                &css,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
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
