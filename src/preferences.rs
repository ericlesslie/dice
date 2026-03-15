use gtk::{gio, gdk, prelude::*};
use adw::prelude::*;

const COLOR_KEYS: [(&str, &str); 6] = [
    ("color-d4", "D4"),
    ("color-d6", "D6"),
    ("color-d8", "D8"),
    ("color-d10", "D10"),
    ("color-d12", "D12"),
    ("color-d20", "D20"),
];

const RNG_VALUES: [&str; 3] = ["chacha", "stdrng", "smallrng"];

pub fn hex_to_rgb(hex: &str) -> [f32; 3] {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
    [r, g, b]
}

pub fn hex_to_rgba(hex: &str) -> gdk::RGBA {
    let [r, g, b] = hex_to_rgb(hex);
    gdk::RGBA::new(r, g, b, 1.0)
}

fn rgba_to_hex(rgba: &gdk::RGBA) -> String {
    let r = (rgba.red() * 255.0).round() as u8;
    let g = (rgba.green() * 255.0).round() as u8;
    let b = (rgba.blue() * 255.0).round() as u8;
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

pub fn build_preferences_dialog() -> adw::PreferencesDialog {
    let settings = gio::Settings::new("org.lesslie.dice");
    let dialog = adw::PreferencesDialog::builder()
        .title("Preferences")
        .build();

    let page = adw::PreferencesPage::new();

    // Die Colors group
    let colors_group = adw::PreferencesGroup::builder()
        .title("Die Colors")
        .build();

    for (key, label) in &COLOR_KEYS {
        let current_hex = settings.string(key);
        let rgba = hex_to_rgba(&current_hex);

        let color_dialog = gtk::ColorDialog::new();
        let button = gtk::ColorDialogButton::builder()
            .dialog(&color_dialog)
            .rgba(&rgba)
            .valign(gtk::Align::Center)
            .build();

        let row = adw::ActionRow::builder()
            .title(*label)
            .build();
        row.add_suffix(&button);

        let settings_clone = settings.clone();
        let key_owned = key.to_string();
        button.connect_notify_local(Some("rgba"), move |btn, _| {
            let hex = rgba_to_hex(&btn.rgba());
            settings_clone.set_string(&key_owned, &hex).ok();
        });

        colors_group.add(&row);
    }

    page.add(&colors_group);

    // RNG group
    let rng_group = adw::PreferencesGroup::builder()
        .title("Random Number Generator")
        .build();

    let model = gtk::StringList::new(&["ChaCha (default)", "StdRng", "SmallRng"]);
    let rng_row = adw::ComboRow::builder()
        .title("Algorithm")
        .model(&model)
        .build();

    let current_rng = settings.string("rng-algorithm");
    let selected = RNG_VALUES.iter().position(|v| *v == current_rng.as_str()).unwrap_or(0);
    rng_row.set_selected(selected as u32);

    let settings_clone = settings.clone();
    rng_row.connect_selected_notify(move |row| {
        let idx = row.selected() as usize;
        if idx < RNG_VALUES.len() {
            settings_clone.set_string("rng-algorithm", RNG_VALUES[idx]).ok();
        }
    });

    rng_group.add(&rng_row);
    page.add(&rng_group);
    dialog.add(&page);

    dialog
}
