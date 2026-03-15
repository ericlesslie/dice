use gtk::prelude::*;
use adw::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::die::DieKind;
use crate::roll_history::{RollEntry, RollHistory};

pub struct Sidebar {
    widget: gtk::Box,
    recents_listbox: gtk::ListBox,
    favorites_listbox: gtk::ListBox,
    history: Rc<RefCell<RollHistory>>,
    on_restore: Rc<dyn Fn(&[(DieKind, u32)])>,
}

impl std::fmt::Debug for Sidebar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sidebar").finish_non_exhaustive()
    }
}

impl Sidebar {
    pub fn new(on_restore: impl Fn(&[(DieKind, u32)]) + 'static) -> Rc<RefCell<Self>> {
        let history = Rc::new(RefCell::new(RollHistory::new()));
        let on_restore = Rc::new(on_restore);

        let recents_listbox = gtk::ListBox::builder()
            .selection_mode(gtk::SelectionMode::None)
            .css_classes(vec!["boxed-list"])
            .build();

        let favorites_listbox = gtk::ListBox::builder()
            .selection_mode(gtk::SelectionMode::None)
            .css_classes(vec!["boxed-list"])
            .build();

        let recents_placeholder = gtk::Label::builder()
            .label("No recent rolls")
            .css_classes(vec!["dim-label"])
            .margin_top(24)
            .margin_bottom(24)
            .build();
        recents_listbox.set_placeholder(Some(&recents_placeholder));

        let favorites_placeholder = gtk::Label::builder()
            .label("No favorites")
            .css_classes(vec!["dim-label"])
            .margin_top(24)
            .margin_bottom(24)
            .build();
        favorites_listbox.set_placeholder(Some(&favorites_placeholder));

        let recents_scroll = gtk::ScrolledWindow::builder()
            .vexpand(true)
            .child(&recents_listbox)
            .build();

        let favorites_scroll = gtk::ScrolledWindow::builder()
            .vexpand(true)
            .child(&favorites_listbox)
            .build();

        let stack = gtk::Stack::new();
        stack.add_titled(&recents_scroll, Some("recents"), "Recents");
        stack.add_titled(&favorites_scroll, Some("favorites"), "Favorites");

        let switcher = gtk::StackSwitcher::builder()
            .stack(&stack)
            .halign(gtk::Align::Center)
            .margin_top(8)
            .margin_bottom(8)
            .margin_start(8)
            .margin_end(8)
            .build();

        let widget = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .width_request(280)
            .build();
        widget.append(&switcher);
        widget.append(&stack);

        let sidebar = Rc::new(RefCell::new(Self {
            widget,
            recents_listbox,
            favorites_listbox,
            history,
            on_restore,
        }));

        // Load existing favorites into the listbox
        {
            let s = sidebar.borrow();
            let history = s.history.borrow();
            for entry in &history.favorites {
                let row = Self::build_favorite_row_static(entry, &sidebar);
                s.favorites_listbox.append(&row);
            }
        }

        sidebar
    }

    pub fn add_recent(&self, dice: Vec<(DieKind, u32)>, sidebar_rc: &Rc<RefCell<Self>>) {
        if dice.is_empty() { return; }
        let entry = self.history.borrow_mut().add_recent(dice);
        let row = self.build_recent_row(&entry, sidebar_rc);
        self.recents_listbox.prepend(&row);
    }

    pub fn widget(&self) -> &gtk::Box {
        &self.widget
    }

    fn build_recent_row(&self, entry: &RollEntry, sidebar_rc: &Rc<RefCell<Self>>) -> adw::ActionRow {
        let (title, subtitle) = RollHistory::format_roll(entry);
        let row = adw::ActionRow::builder()
            .title(&title)
            .subtitle(&subtitle)
            .activatable(true)
            .build();

        let star_button = gtk::Button::builder()
            .icon_name("starred-symbolic")
            .valign(gtk::Align::Center)
            .css_classes(vec!["flat"])
            .tooltip_text("Add to favorites")
            .build();

        let id = entry.id;
        let sidebar_weak = Rc::downgrade(sidebar_rc);
        star_button.connect_clicked(move |_| {
            if let Some(sidebar_rc) = sidebar_weak.upgrade() {
                let s = sidebar_rc.borrow();
                s.history.borrow_mut().favorite(id);
                s.refresh_favorites(&sidebar_rc);
            }
        });
        row.add_suffix(&star_button);

        let dice = entry.dice.clone();
        let restore = self.on_restore.clone();
        row.connect_activated(move |_| {
            restore(&dice);
        });

        row
    }

    fn build_favorite_row_static(entry: &RollEntry, sidebar_rc: &Rc<RefCell<Self>>) -> adw::ActionRow {
        let (title, subtitle) = RollHistory::format_roll(entry);
        let row = adw::ActionRow::builder()
            .title(&title)
            .subtitle(&subtitle)
            .activatable(true)
            .build();

        let remove_button = gtk::Button::builder()
            .icon_name("user-trash-symbolic")
            .valign(gtk::Align::Center)
            .css_classes(vec!["flat"])
            .tooltip_text("Remove from favorites")
            .build();

        let id = entry.id;
        let sidebar_weak = Rc::downgrade(sidebar_rc);
        remove_button.connect_clicked(move |_| {
            if let Some(sidebar_rc) = sidebar_weak.upgrade() {
                let s = sidebar_rc.borrow();
                s.history.borrow_mut().remove_favorite(id);
                s.refresh_favorites(&sidebar_rc);
            }
        });
        row.add_suffix(&remove_button);

        let dice = entry.dice.clone();
        let restore = {
            let s = sidebar_rc.borrow();
            s.on_restore.clone()
        };
        row.connect_activated(move |_| {
            restore(&dice);
        });

        row
    }

    fn refresh_favorites(&self, sidebar_rc: &Rc<RefCell<Self>>) {
        // Remove all rows
        while let Some(child) = self.favorites_listbox.first_child() {
            self.favorites_listbox.remove(&child);
        }

        let history = self.history.borrow();
        for entry in &history.favorites {
            let row = Self::build_favorite_row_static(entry, sidebar_rc);
            self.favorites_listbox.append(&row);
        }
    }
}
