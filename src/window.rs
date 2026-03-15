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

use std::cell::RefCell;
use std::rc::Rc;

use crate::dice_area::DiceArea;
use crate::sidebar::Sidebar;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/lesslie/dice/window.ui")]
    pub struct DiceWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub split_view: TemplateChild<adw::OverlaySplitView>,
        #[template_child]
        pub sidebar_button: TemplateChild<gtk::ToggleButton>,
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
        #[template_child]
        pub total_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub reroll_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub clear_button: TemplateChild<gtk::Button>,

        pub sidebar: RefCell<Option<Rc<RefCell<Sidebar>>>>,
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

            // Bind sidebar button to split view
            self.split_view.bind_property("show-sidebar", &*self.sidebar_button, "active")
                .bidirectional()
                .sync_create()
                .build();

            // Create sidebar
            let dice_area_for_restore = self.dice_area.clone();
            let sidebar = Sidebar::new(move |dice| {
                dice_area_for_restore.restore_roll(dice);
            });

            self.split_view.set_sidebar(Some(sidebar.borrow().widget()));
            *self.sidebar.borrow_mut() = Some(sidebar);

            // Register window actions for keyboard shortcuts
            let settings = gio::Settings::new("org.lesslie.dice");

            let dice_area = self.dice_area.clone();
            let sidebar_ref = self.sidebar.borrow().clone();
            let settings_clone = settings.clone();
            let action = gio::SimpleAction::new("roll-d4", None);
            action.connect_activate(move |_, _| {
                if settings_clone.boolean("record-all-rolls") {
                    if let Some(ref sidebar_rc) = sidebar_ref {
                        let snapshot = dice_area.dice_snapshot();
                        sidebar_rc.borrow().add_recent(snapshot, sidebar_rc);
                    }
                }
                dice_area.add_four();
            });
            self.obj().add_action(&action);

            let dice_area = self.dice_area.clone();
            let sidebar_ref = self.sidebar.borrow().clone();
            let settings_clone = settings.clone();
            let action = gio::SimpleAction::new("roll-d6", None);
            action.connect_activate(move |_, _| {
                if settings_clone.boolean("record-all-rolls") {
                    if let Some(ref sidebar_rc) = sidebar_ref {
                        let snapshot = dice_area.dice_snapshot();
                        sidebar_rc.borrow().add_recent(snapshot, sidebar_rc);
                    }
                }
                dice_area.add_six();
            });
            self.obj().add_action(&action);

            let dice_area = self.dice_area.clone();
            let sidebar_ref = self.sidebar.borrow().clone();
            let settings_clone = settings.clone();
            let action = gio::SimpleAction::new("roll-d8", None);
            action.connect_activate(move |_, _| {
                if settings_clone.boolean("record-all-rolls") {
                    if let Some(ref sidebar_rc) = sidebar_ref {
                        let snapshot = dice_area.dice_snapshot();
                        sidebar_rc.borrow().add_recent(snapshot, sidebar_rc);
                    }
                }
                dice_area.add_eight();
            });
            self.obj().add_action(&action);

            let dice_area = self.dice_area.clone();
            let sidebar_ref = self.sidebar.borrow().clone();
            let settings_clone = settings.clone();
            let action = gio::SimpleAction::new("roll-d10", None);
            action.connect_activate(move |_, _| {
                if settings_clone.boolean("record-all-rolls") {
                    if let Some(ref sidebar_rc) = sidebar_ref {
                        let snapshot = dice_area.dice_snapshot();
                        sidebar_rc.borrow().add_recent(snapshot, sidebar_rc);
                    }
                }
                dice_area.add_ten();
            });
            self.obj().add_action(&action);

            let dice_area = self.dice_area.clone();
            let sidebar_ref = self.sidebar.borrow().clone();
            let settings_clone = settings.clone();
            let action = gio::SimpleAction::new("roll-d12", None);
            action.connect_activate(move |_, _| {
                if settings_clone.boolean("record-all-rolls") {
                    if let Some(ref sidebar_rc) = sidebar_ref {
                        let snapshot = dice_area.dice_snapshot();
                        sidebar_rc.borrow().add_recent(snapshot, sidebar_rc);
                    }
                }
                dice_area.add_twelve();
            });
            self.obj().add_action(&action);

            let dice_area = self.dice_area.clone();
            let sidebar_ref = self.sidebar.borrow().clone();
            let settings_clone = settings.clone();
            let action = gio::SimpleAction::new("roll-d20", None);
            action.connect_activate(move |_, _| {
                if settings_clone.boolean("record-all-rolls") {
                    if let Some(ref sidebar_rc) = sidebar_ref {
                        let snapshot = dice_area.dice_snapshot();
                        sidebar_rc.borrow().add_recent(snapshot, sidebar_rc);
                    }
                }
                dice_area.add_twenty();
            });
            self.obj().add_action(&action);

            let dice_area = self.dice_area.clone();
            let sidebar_ref = self.sidebar.borrow().clone();
            let action = gio::SimpleAction::new("reroll", None);
            action.connect_activate(move |_, _| {
                if let Some(ref sidebar_rc) = sidebar_ref {
                    let snapshot = dice_area.dice_snapshot();
                    sidebar_rc.borrow().add_recent(snapshot, sidebar_rc);
                }
                dice_area.roll();
            });
            self.obj().add_action(&action);

            let dice_area = self.dice_area.clone();
            let sidebar_ref = self.sidebar.borrow().clone();
            let action = gio::SimpleAction::new("clear", None);
            action.connect_activate(move |_, _| {
                if let Some(ref sidebar_rc) = sidebar_ref {
                    let snapshot = dice_area.dice_snapshot();
                    sidebar_rc.borrow().add_recent(snapshot, sidebar_rc);
                }
                dice_area.clear();
            });
            self.obj().add_action(&action);

            // Toggle sidebar action
            let split_view = self.split_view.clone();
            let action = gio::SimpleAction::new("toggle-sidebar", None);
            action.connect_activate(move |_, _| {
                split_view.set_show_sidebar(!split_view.shows_sidebar());
            });
            self.obj().add_action(&action);

            let dice_area = self.dice_area.clone();
            let dice_labels = self.dice_labels.clone();
            let total_label = self.total_label.clone();
            let reroll_button = self.reroll_button.clone();
            let clear_button = self.clear_button.clone();
            self.obj().add_tick_callback(move |_widget, _clock| {
                // Remove old labels
                let mut child = dice_labels.first_child();
                while let Some(c) = child {
                    child = c.next_sibling();
                    dice_labels.remove(&c);
                }

                // Add labels for settled dice
                let infos = dice_area.settled_dice_info();
                for (wx, wy, val) in &infos {
                    let label = gtk::Label::new(Some(&val.to_string()));
                    label.add_css_class("die-number");
                    label.set_can_target(false);
                    let (_, nat_w, _, _) = label.measure(gtk::Orientation::Horizontal, -1);
                    let (_, nat_h, _, _) = label.measure(gtk::Orientation::Vertical, -1);
                    dice_labels.put(&label, (*wx - nat_w as f32 / 2.0) as f64, (*wy - nat_h as f32 / 2.0) as f64);
                }

                // Update total label
                let has_dice = dice_area.has_dice();
                if !infos.is_empty() {
                    let sum: u32 = infos.iter().map(|(_, _, v)| v).sum();
                    total_label.set_text(&format!("{}", sum));
                    total_label.set_visible(true);
                } else if !has_dice {
                    total_label.set_visible(false);
                }

                // Update button sensitivity
                reroll_button.set_sensitive(has_dice);
                clear_button.set_sensitive(has_dice);

                glib::ControlFlow::Continue
            });

            let css = gtk::CssProvider::new();
            css.load_from_string(
                ".die-number { font-size: 24px; font-weight: bold; color: white; text-shadow: 0 1px 3px rgba(0,0,0,0.8); } .total-pill { font-weight: bold; padding: 4px 12px; }",
            );
            self.total_label.add_css_class("total-pill");
            self.total_label.add_css_class("dim-label");
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
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

#[gtk::template_callbacks]
impl DiceWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    fn snapshot_if_recording(&self) {
        let settings = gio::Settings::new("org.lesslie.dice");
        if settings.boolean("record-all-rolls") {
            let imp = self.imp();
            if let Some(ref sidebar_rc) = *imp.sidebar.borrow() {
                let snapshot = imp.dice_area.dice_snapshot();
                sidebar_rc.borrow().add_recent(snapshot, sidebar_rc);
            }
        }
    }

    #[template_callback]
    fn handle_four_clicked(&self) {
        println!("Four clicked");
        self.snapshot_if_recording();
        self.imp().dice_area.add_four();
    }

    #[template_callback]
    fn handle_six_clicked(&self) {
        println!("Six clicked");
        self.snapshot_if_recording();
        self.imp().dice_area.add_six();
    }

    #[template_callback]
    fn handle_eight_clicked(&self) {
        println!("Eight clicked");
        self.snapshot_if_recording();
        self.imp().dice_area.add_eight();
    }

    #[template_callback]
    fn handle_ten_clicked(&self) {
        println!("Ten clicked");
        self.snapshot_if_recording();
        self.imp().dice_area.add_ten();
    }

    #[template_callback]
    fn handle_twelve_clicked(&self) {
        println!("Twelve clicked");
        self.snapshot_if_recording();
        self.imp().dice_area.add_twelve();
    }

    #[template_callback]
    fn handle_twenty_clicked(&self) {
        println!("Twenty clicked");
        self.snapshot_if_recording();
        self.imp().dice_area.add_twenty();
    }

    #[template_callback]
    fn handle_reroll_clicked(&self) {
        let imp = &self.imp();
        if let Some(ref sidebar_rc) = *imp.sidebar.borrow() {
            let snapshot = imp.dice_area.dice_snapshot();
            sidebar_rc.borrow().add_recent(snapshot, sidebar_rc);
        }
        imp.dice_area.roll();
    }

    #[template_callback]
    fn handle_clear_clicked(&self) {
        let imp = &self.imp();
        if let Some(ref sidebar_rc) = *imp.sidebar.borrow() {
            let snapshot = imp.dice_area.dice_snapshot();
            sidebar_rc.borrow().add_recent(snapshot, sidebar_rc);
        }
        imp.dice_area.clear();
    }
}
