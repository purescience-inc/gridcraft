//! The Pure edition's title bar: one row on the frame colour. Quick access (Save, Undo, Redo) at
//! the left, the workbook's name and its saved state in the centre, then the command search box,
//! Comments and, when a host is connected, Share.
//!
//! What upstream's title bar has and this one leaves out: the AutoSave switch (the host saves),
//! the Home button and its start screen, the More commands menu (New, Open and Save As are the
//! host's), the light/dark switch (the host sets the theme through `view.theme`) and the Share
//! menu of export formats (File ▸ Export has them).

use egui::{Align2, Color32, Rect, Response, Sense, Stroke, StrokeKind, Ui, pos2, vec2};
use serde_json::json;

use super::{BAR_ICON, CONTROL_H, RADIUS, TYPE_CONTROL, TYPE_TITLE};
use crate::SheetApp;
use crate::icons::{self, Icon};
use crate::theme::{self, Tokens};
use crate::widgets::icon_button;

/// The bar's height. On macOS the bar is AppKit's own 28 pt band (see upstream's title bar).
const HEIGHT: f32 = if cfg!(target_os = "macos") { crate::ribbon::MAC_TITLE_BAR } else { 36.0 };

/// A title-bar control's height: smaller in the macOS band so it keeps a margin.
const CONTROL: f32 = if cfg!(target_os = "macos") { 22.0 } else { CONTROL_H };

/// The search box at its widest, and the window width under which it becomes an icon button.
const SEARCH_W: f32 = 240.0;
const SEARCH_MIN_WINDOW: f32 = 760.0;

/// Gap kept between the centred title and the controls beside it.
const TITLE_GAP: f32 = 16.0;

pub(crate) fn show(app: &mut SheetApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    egui::Panel::top("title_bar").exact_size(HEIGHT).frame(egui::Frame::NONE.fill(t.window)).show(ui, |ui| {
        let rect = ui.max_rect();
        // Dragging the empty title area moves the window, as upstream's does (a no-op on the web).
        let bg = ui.interact(rect, ui.scope_id().with("drag_title"), Sense::click_and_drag());
        if bg.drag_started() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }
        if bg.double_clicked() {
            let max = ui.ctx().input(|i| i.viewport().maximized.unwrap_or(false));
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(!max));
        }
        ui.painter()
            .line_segment([pos2(rect.left(), rect.bottom() - 0.5), pos2(rect.right(), rect.bottom() - 0.5)], Stroke::new(1.0, t.ribbon_border));
        ui.horizontal_centered(|ui| {
            ui.spacing_mut().item_spacing.x = 2.0;
            ui.add_space(crate::ribbon::TITLE_LEFT_PAD);
            quick_access(app, ui, &t);
            let left_end = ui.min_rect().right();
            let right = ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                ui.add_space(8.0);
                share(app, ui, &t);
                let comments = icon_button(ui, Icon::Comment, t.text_dim, "Comments", vec2(28.0, CONTROL));
                crate::ribbon::control_keytip(app, ui, &comments, "comments");
                if comments.clicked() {
                    app.open_dialog("comments", json!({}));
                }
                ui.add_space(4.0);
                let search = if rect.width() >= SEARCH_MIN_WINDOW {
                    search_box(ui, &t, app.ui.language.tr("Search commands"))
                } else {
                    icon_button(ui, Icon::Search, t.text_dim, "Search commands", vec2(28.0, CONTROL))
                };
                crate::ribbon::control_keytip(app, ui, &search, "commandSearch");
                if search.clicked() {
                    app.open_dialog("commandSearch", json!({}));
                }
                ui.min_rect().left()
            });
            let (title, dirty) = app.session.active().map(|d| (d.display_title(), d.is_dirty())).unwrap_or_default();
            let state = app.ui.language.tr(if dirty { "Edited" } else { "Saved" });
            let half = (rect.center().x - left_end).min(right.inner - rect.center().x) - TITLE_GAP;
            paint_title(ui, &t, rect.center(), &title, state, (2.0 * half).max(0.0));
        });
    });
}

/// Save, Undo (with its list) and Redo.
fn quick_access(app: &mut SheetApp, ui: &mut Ui, t: &Tokens) {
    let size = vec2(28.0, CONTROL);
    // A cell still being edited is part of what is saved, as File ▸ Save has it.
    if icon_button(ui, Icon::Save, t.text_dim, "Save", size).clicked() && app.commit_edit(0, 0, false, false) {
        app.run_or_alert("file.save", json!({}));
    }
    // The commands' own availability, named and marked for assistive technology as upstream does.
    let can_undo = gridcraft_engine::cmd::can_undo(&app.session).is_ok();
    let can_redo = gridcraft_engine::cmd::can_redo(&app.session).is_ok();
    let undo = icon_button(ui, Icon::Undo, if can_undo { t.text_dim } else { t.text_disabled }, "Undo", size);
    undo.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, can_undo, "Undo"));
    if undo.clicked() && can_undo {
        app.run_or_alert("edit.undo", json!({}));
    }
    let undo_list = icon_button(ui, Icon::Chevron, t.text_dim, "Undo list", vec2(14.0, CONTROL));
    egui::Popup::menu(&undo_list).show(|ui| {
        let labels: Vec<String> = if can_undo {
            app.session.active().map(|d| d.undo.iter().rev().take(20).map(|e| e.label.clone()).collect()).unwrap_or_default()
        } else {
            Vec::new()
        };
        if labels.is_empty() {
            ui.label("Can't Undo");
        }
        for (i, l) in labels.iter().enumerate() {
            if ui.button(format!("Undo {l}")).clicked() {
                app.run_or_alert("edit.undo", json!({"steps": i + 1}));
            }
        }
    });
    let redo = icon_button(ui, Icon::Redo, if can_redo { t.text_dim } else { t.text_disabled }, "Redo", size);
    redo.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, can_redo, "Redo"));
    if redo.clicked() && can_redo {
        app.run_or_alert("edit.redo", json!({}));
    }
}

/// Share: an outlined button with an icon and a label, drawn only while a host is connected. A
/// click tells the host, which owns sharing; nothing is shown by the app itself.
fn share(app: &mut SheetApp, ui: &mut Ui, t: &Tokens) {
    if app.services.host_share.is_none() {
        return;
    }
    let label = app.ui.language.tr("Share");
    let font = theme::ui_font(TYPE_CONTROL);
    let text_w = ui.painter().layout_no_wrap(label.to_string(), font.clone(), t.text).size().x;
    let (rect, resp) = ui.allocate_exact_size(vec2(8.0 + BAR_ICON + 6.0 + text_w + 10.0, CONTROL), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, label));
    let fill = if resp.is_pointer_button_down_on() {
        t.pressed
    } else if resp.hovered() {
        t.hover
    } else {
        Color32::TRANSPARENT
    };
    let p = ui.painter();
    p.rect_filled(rect, RADIUS, fill);
    p.rect_stroke(rect, RADIUS, Stroke::new(1.0, if resp.hovered() { super::strong_line(t) } else { t.input_border }), StrokeKind::Inside);
    icons::paint(
        p,
        Rect::from_center_size(pos2(rect.left() + 8.0 + BAR_ICON / 2.0, rect.center().y), vec2(BAR_ICON, BAR_ICON)),
        Icon::Share,
        t.text_dim,
    );
    p.text(pos2(rect.left() + 8.0 + BAR_ICON + 6.0, rect.center().y), Align2::LEFT_CENTER, label, font, t.text);
    if resp.on_hover_text(app.ui.language.tr("Share this workbook")).clicked() {
        app.host_share();
    }
}

/// The command search box: it looks like a field and opens the command search when clicked.
fn search_box(ui: &mut Ui, t: &Tokens, hint: &str) -> Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(SEARCH_W, CONTROL), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, hint));
    let p = ui.painter();
    p.rect_filled(rect, RADIUS, t.input_bg);
    p.rect_stroke(rect, RADIUS, Stroke::new(1.0, if resp.hovered() { super::strong_line(t) } else { t.input_border }), StrokeKind::Inside);
    icons::paint(
        p,
        Rect::from_center_size(pos2(rect.left() + 8.0 + BAR_ICON / 2.0, rect.center().y), vec2(BAR_ICON, BAR_ICON)),
        Icon::Search,
        t.text_dim,
    );
    p.text(pos2(rect.left() + 8.0 + BAR_ICON + 6.0, rect.center().y), Align2::LEFT_CENTER, hint, theme::ui_font(TYPE_CONTROL), t.text_dim);
    resp.on_hover_cursor(egui::CursorIcon::Text)
}

/// The workbook's name (semibold) and its saved state (secondary), centred on `center` and cut
/// with "…" so the name never runs under the controls on either side. The state stays whole.
fn paint_title(ui: &Ui, t: &Tokens, center: egui::Pos2, title: &str, state: &str, max_w: f32) {
    let p = ui.painter();
    let state = p.layout_no_wrap(state.to_string(), theme::ui_font(TYPE_TITLE), t.text_dim);
    let gap = 10.0;
    let mut job = egui::text::LayoutJob::simple_singleline(title.to_string(), theme::ui_bold(TYPE_TITLE), t.text);
    job.wrap = egui::text::TextWrapping::truncate_at_width((max_w - state.size().x - gap).max(0.0));
    let name = p.layout_job(job);
    let width = name.size().x + gap + state.size().x;
    if width > max_w {
        return;
    }
    let left = center.x - width / 2.0;
    let name_w = name.size().x;
    let top = |g: &egui::Galley| center.y - g.size().y / 2.0;
    let (name_top, state_top) = (top(&name), top(&state));
    p.galley(pos2(left, name_top), name, t.text);
    p.galley(pos2(left + name_w + gap, state_top), state, t.text_dim);
}
