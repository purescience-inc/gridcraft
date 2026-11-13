//! The Pure edition's File screen: which pages it keeps, and the pages that are ours.
//!
//! Kept: Info, Save (asks the host to save now), Print, Export, Options and About. Home, New,
//! Open, Save As, Close and Account are the host's or have no place in a hosted app; a command
//! that names one of them (`ui.backstage {"page": "home"}`) lands on Info.

use egui::{RichText, Ui};
use serde_json::json;

use super::chrome as theme;
use super::{NAME, UPSTREAM};
use crate::SheetApp;
use crate::backstage::Page;
use crate::theme::Tokens;

/// Whether the Pure edition's File screen has `page`.
pub(crate) fn has(page: Page) -> bool {
    matches!(page, Page::Info | Page::Print | Page::Export | Page::Options | Page::Account)
}

/// The pages of `pages` the File screen lists: all of them upstream, ours in the Pure edition.
pub(crate) fn kept<const N: usize>(pages: [Page; N]) -> Vec<Page> {
    pages.into_iter().filter(|page| !super::ON || has(*page)).collect()
}

/// The page shown for `page`: itself, or Info for one the Pure edition leaves out.
pub(crate) fn page(page: Page) -> Page {
    if super::ON && !has(page) { Page::Info } else { page }
}

/// "Built on GridCraft 0.19.1 (MIT or Apache-2.0), © its contributors."
pub fn built_on() -> String {
    format!("Built on {UPSTREAM} {} ({}), © its contributors.", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_LICENSE").replace(" OR ", " or "))
}

/// Where the licence texts are: they ship with every build, as upstream's do.
pub const LICENCE_FILES: &str =
    "The licence texts, notices and asset attributions come with the app: LICENSE-MIT, LICENSE-APACHE, NOTICE and ATTRIBUTION.md.";

/// The About tab of the About window.
pub(crate) fn about_dialog(ui: &mut Ui) {
    ui.heading(NAME);
    ui.add_space(4.0);
    ui.label(built_on());
    ui.add_space(4.0);
    ui.label(LICENCE_FILES);
}

/// File ▸ About (upstream's Account page): what the app is and what it is built on, with the
/// contributor credits upstream compiled in one click away.
pub(crate) fn about(app: &mut SheetApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let lang = app.ui.language;
    ui.label(RichText::new(NAME).font(theme::ui_bold(14.0)).color(t.text));
    ui.add_space(4.0);
    ui.label(RichText::new(built_on()).color(t.text_dim));
    ui.add_space(16.0);
    let rows = [
        ("Version", env!("CARGO_PKG_VERSION").to_string()),
        ("Platform", crate::backstage::platform()),
        ("Licence", env!("CARGO_PKG_LICENSE").to_string()),
    ];
    for (label, value) in rows {
        property_row(ui, &t, lang.tr(label), |ui| {
            ui.label(value);
        });
    }
    ui.add_space(12.0);
    ui.label(RichText::new(LICENCE_FILES).color(t.text_dim));
    ui.add_space(12.0);
    if ui.button(lang.tr("Contributors")).clicked() {
        app.open_dialog("about", json!({"tab": "Contributors"}));
    }
}

/// Width of the label column of a ruled property list.
const LABEL_W: f32 = 160.0;
/// Height of one ruled row.
const ROW_H: f32 = 32.0;

/// One row of a ruled list: a secondary label, its value, and a rule under both.
pub(crate) fn property_row(ui: &mut Ui, t: &Tokens, label: &str, value: impl FnOnce(&mut Ui)) {
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, ROW_H), egui::Sense::hover());
    let mut row = ui.new_child(egui::UiBuilder::new().max_rect(rect).layout(egui::Layout::left_to_right(egui::Align::Center)));
    let (cell, _) = row.allocate_exact_size(egui::vec2(LABEL_W, ROW_H), egui::Sense::hover());
    row.painter().text(cell.left_center(), egui::Align2::LEFT_CENTER, label, theme::ui_font(13.0), t.text_dim);
    value(&mut row);
    ui.painter().line_segment([rect.left_bottom(), rect.right_bottom()], egui::Stroke::new(1.0, t.separator));
}

/// The line at the File side list's far edge (Pure edition), between it and the page.
pub(crate) fn nav_edge(ui: &Ui) {
    if !super::ON {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let r = ui.max_rect();
    ui.painter().line_segment(
        [egui::pos2(r.max.x - 0.5, r.min.y - 10.0), egui::pos2(r.max.x - 0.5, r.max.y + 10.0)],
        egui::Stroke::new(1.0, t.ribbon_border),
    );
}
