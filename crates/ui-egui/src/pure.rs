//! The Pure edition (PureSheets): everything that is ours, in one place.
//!
//! This fork builds two apps from one tree. Without the cargo feature `pure` it is upstream's
//! GridCraft, unchanged. With it, the visible names, the palette, the sizes and a few pieces of
//! chrome come from this module. Upstream files reach it through short guards
//! (`if crate::pure::ON { … }`), so both sides always compile and a new upstream release merges
//! with conflicts of a line or two.
//!
//! - Names: [`NAME`], [`app_name`], [`rename`].
//! - Colours: the "Paper" palette ([`Paper`]), put into upstream's theme tokens by [`tokens`] and
//!   into egui's own widgets by [`style`].
//! - Measure and type: the constants below, [`radius`], [`flat`] and [`type_size`]; chrome files
//!   import [`chrome`] in place of `theme` so the sizes they name land on our type scale.
//! - Fonts: Inter for all chrome ([`fonts`]).

use egui::{Color32, FontFamily, Stroke};

use crate::theme::Tokens;

pub mod backstage;
pub(crate) mod title_bar;

/// Whether this build is the Pure edition.
pub const ON: bool = cfg!(feature = "pure");

/// The app's visible name in the Pure edition.
pub const NAME: &str = "PureSheets (Craft)";

/// The name inside a sentence ("PureSheets keeps them when you save").
pub const SHORT_NAME: &str = "PureSheets";

/// The upstream app this edition is built on, as About credits it.
pub const UPSTREAM: &str = "GridCraft";

/// The name the interface shows for the app: ours in the Pure edition, upstream's otherwise.
/// Message boxes and dialog titles pass the name upstream wrote through here.
pub fn app_name(upstream: &str) -> &str {
    if ON && upstream == UPSTREAM { NAME } else { upstream }
}

/// A sentence upstream wrote, with its product name replaced by ours in the Pure edition
/// (engine messages and notices name the app: "… which GridCraft can't open yet").
pub fn rename(text: &str) -> std::borrow::Cow<'_, str> {
    if ON && text.contains(UPSTREAM) { text.replace(UPSTREAM, SHORT_NAME).into() } else { text.into() }
}

/// The About window's title.
pub fn about_title(upstream: &'static str) -> &'static str {
    if ON { "About PureSheets (Craft)" } else { upstream }
}

/// Commands that are the host's in the Pure edition: New, Open and Close would leave the host's
/// tab showing another workbook, or none. A menu, a key or a key tip does not run them
/// (`SheetApp::run_or_alert`); a caller of the command API still can.
pub fn host_owns(command: &str) -> bool {
    ON && matches!(command, "file.new" | "file.open" | "file.close")
}

/// Commands the command search leaves out in the Pure edition: the host's, and Save As, which
/// hands the host the same file Save does.
pub fn hidden_command(command: &str) -> bool {
    host_owns(command) || (ON && command == "file.saveAs")
}

/// A contributor's name in the credits: a link to their GitHub profile upstream, plain text in
/// the Pure edition, which links to no outside site.
pub fn credit(ui: &mut egui::Ui, name: impl Into<egui::WidgetText>, url: String) -> egui::Response {
    if ON { ui.label(name) } else { ui.hyperlink_to(name, url) }
}

// ---------------------------------------------------------------- measure

/// Corners of buttons, inputs, menus and dialogs. Bars and panels have none.
pub const RADIUS: f32 = 3.0;

/// One control row: the height of a small ribbon button, an input, a bar button.
pub const CONTROL_H: f32 = 24.0;

/// Small ribbon icons and the icons of the title, tab and status bars.
pub const BAR_ICON: f32 = 16.0;

/// Large ribbon icons.
pub const LARGE_ICON: f32 = 24.0;

/// A large ribbon button: the height of the three small rows beside it.
pub const LARGE_H: f32 = 3.0 * CONTROL_H;

/// Space either side of the rule between two ribbon groups.
pub const GROUP_GAP: f32 = 8.0;

/// The type scale, in whole sizes: labels and the status bar; ribbon controls and inputs; ribbon
/// tabs, menus and the File side list; the document title; File screen headings.
pub const TYPE_LABEL: f32 = 11.0;
pub const TYPE_CONTROL: f32 = 12.0;
pub const TYPE_TAB: f32 = 13.0;
pub const TYPE_TITLE: f32 = 14.0;
pub const TYPE_HEADING: f32 = 20.0;

/// A corner radius upstream wrote for a button, input or menu: ours in the Pure edition.
pub fn radius(upstream: f32) -> f32 {
    if ON { RADIUS } else { upstream }
}

/// A corner radius upstream wrote for a bar, panel or strip: square in the Pure edition.
pub fn flat(upstream: f32) -> f32 {
    if ON { 0.0 } else { upstream }
}

/// A chrome text size upstream wrote, moved onto our type scale in the Pure edition.
pub fn type_size(upstream: f32) -> f32 {
    if !ON {
        upstream
    } else if upstream < 11.75 {
        TYPE_LABEL
    } else if upstream < 12.75 {
        TYPE_CONTROL
    } else if upstream < 13.75 {
        TYPE_TAB
    } else if upstream < 17.0 {
        TYPE_TITLE
    } else {
        TYPE_HEADING
    }
}

/// `theme`, as chrome files import it (`use crate::pure::chrome as theme;`): the same module,
/// except that the interface fonts land on our type scale in the Pure edition. Files that draw
/// document content (the grid, charts, the page view) keep `crate::theme`, whose sizes scale with
/// the zoom and must not be rounded.
pub(crate) mod chrome {
    pub(crate) use crate::theme::*;

    pub fn ui_font(size: f32) -> egui::FontId {
        crate::theme::ui_font(super::type_size(size))
    }

    pub fn ui_bold(size: f32) -> egui::FontId {
        crate::theme::ui_bold(super::type_size(size))
    }
}

// ---------------------------------------------------------------- colour

/// PureDesktop's "Paper" palette, one value per role.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Paper {
    /// Title bar and status bar.
    pub frame: Color32,
    /// Ribbon, tab strip, panels, File screen.
    pub surface: Color32,
    /// Menus, popups and dialogs.
    pub popup: Color32,
    /// The ground behind a page, and the well of an input in the dark theme.
    pub canvas: Color32,
    /// Dividers and group separators.
    pub rule: Color32,
    /// Control outlines and bar edges.
    pub line: Color32,
    /// Input focus and emphasis.
    pub strong_line: Color32,
    pub text: Color32,
    /// Secondary text and icons.
    pub text_2: Color32,
    pub disabled: Color32,
    pub hover: Color32,
    pub pressed: Color32,
    /// A toggled command or the chosen item: its fill and its mark.
    pub on_fill: Color32,
    pub on_mark: Color32,
    /// The one primary action, the active tab's rule, focus, links.
    pub accent: Color32,
    pub accent_hover: Color32,
    pub on_accent: Color32,
    /// Selected text in chrome text fields.
    pub selection: Color32,
    pub danger: Color32,
}

const fn rgb(hex: u32) -> Color32 {
    Color32::from_rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

pub const LIGHT: Paper = Paper {
    frame: rgb(0xF7F8F9),
    surface: rgb(0xFFFFFF),
    popup: rgb(0xFFFFFF),
    canvas: rgb(0xE9EBEE),
    rule: rgb(0xE3E5E9),
    line: rgb(0xCDD1D7),
    strong_line: rgb(0xAAB0B9),
    text: rgb(0x1F2328),
    text_2: rgb(0x4F5661),
    disabled: rgb(0x9AA1AB),
    hover: rgb(0xF0F2F4),
    pressed: rgb(0xE3E5E9),
    on_fill: rgb(0xE9EFF9),
    on_mark: rgb(0x1F3A5F),
    accent: rgb(0x1F3A5F),
    accent_hover: rgb(0x172D4B),
    on_accent: rgb(0xFFFFFF),
    selection: rgb(0xCFDCEE),
    danger: rgb(0xA8291F),
};

pub const DARK: Paper = Paper {
    frame: rgb(0x1B1F23),
    surface: rgb(0x1F2328),
    popup: rgb(0x262B31),
    canvas: rgb(0x16191D),
    rule: rgb(0x2D3238),
    line: rgb(0x3A4047),
    strong_line: rgb(0x555C65),
    text: rgb(0xE7E9EC),
    text_2: rgb(0xB3B9C2),
    disabled: rgb(0x6B727C),
    hover: rgb(0x2A2F35),
    pressed: rgb(0x32383F),
    on_fill: rgb(0x26364B),
    on_mark: rgb(0x9DBBE4),
    accent: rgb(0x9DBBE4),
    accent_hover: rgb(0xB7CDEC),
    on_accent: rgb(0x16191D),
    selection: rgb(0x34506F),
    danger: rgb(0xF0968D),
};

/// The palette for a theme.
pub const fn paper(dark: bool) -> &'static Paper {
    if dark { &DARK } else { &LIGHT }
}

/// Upstream's theme tokens with every chrome colour mapped to its Paper role (the Pure edition),
/// or as they are. Colours of the sheet itself (the white ground, gridlines, cell text, reference
/// and trace colours, on-sheet controls) are upstream's: the grid is not redesigned.
pub fn tokens(mut t: Tokens) -> Tokens {
    if !ON {
        return t;
    }
    let p = paper(t.dark);
    t.window = p.frame;
    t.ribbon = p.surface;
    t.ribbon_border = p.line;
    t.text = p.text;
    t.text_dim = p.text_2;
    t.text_disabled = p.disabled;
    t.accent = p.accent;
    t.accent_dark = p.accent_hover;
    t.accent_soft = p.on_fill;
    t.hover = p.hover;
    t.pressed = p.pressed;
    t.separator = p.rule;
    t.desk = p.canvas;
    // The row and column headings are chrome around the sheet: frame, line and secondary text.
    t.header_bg = p.frame;
    t.header_text = p.text_2;
    t.header_line = p.line;
    t.header_sel_bg = p.pressed;
    t.header_sel_text = p.accent;
    t.header_all_bg = p.on_fill;
    // The selection on the sheet takes the accent in place of upstream's green, with the same
    // strengths. The sheet is white in both themes, so these are the light values in both.
    let ink = LIGHT.accent;
    t.sel_border = ink;
    t.sel_fill = Color32::from_rgba_unmultiplied(ink.r(), ink.g(), ink.b(), if t.dark { 0x24 } else { 0x1C });
    t.active_row = Color32::from_rgba_unmultiplied(ink.r(), ink.g(), ink.b(), 0x24);
    t.sheet_fill = ink;
    t.sheet_focus = ink;
    // An input is a white field in the light theme and a well in the dark one.
    t.input_bg = if t.dark { p.canvas } else { p.surface };
    t.input_border = p.line;
    t.tab_bar = p.surface;
    t.tab_active = p.surface;
    t.status_bar = p.frame;
    t.menu_bg = p.popup;
    t.shadow = if t.dark { Color32::from_black_alpha(70) } else { Color32::from_black_alpha(24) };
    t.danger = p.danger;
    t.on_accent = p.on_accent;
    t.paper = if t.dark { p.pressed } else { p.surface };
    t.notice_info_bg = p.on_fill;
    t.notice_info_border = if t.dark { p.accent } else { rgb(0x5B7FB0) };
    t
}

/// The strong line (input focus, a hovered outline) for the theme `t` is in.
pub fn strong_line(t: &Tokens) -> Color32 {
    paper(t.dark).strong_line
}

/// The mark of a toggled command or chosen item for the theme `t` is in.
pub fn on_mark(t: &Tokens) -> Color32 {
    paper(t.dark).on_mark
}

/// After upstream's `theme::apply`: egui's own widgets (buttons, fields, menus, dialogs) take the
/// Pure edition's corners, lines and states, and the text styles its type scale.
pub fn style(ctx: &egui::Context) {
    if !ON {
        return;
    }
    let corner = egui::CornerRadius::same(RADIUS as u8);
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        let p = paper(theme == egui::Theme::Dark);
        ctx.style_mut_of(theme, |s| {
            let v = &mut s.visuals;
            v.panel_fill = p.surface;
            v.window_fill = p.popup;
            v.window_stroke = Stroke::new(1.0, p.line);
            v.window_corner_radius = corner;
            v.menu_corner_radius = corner;
            v.extreme_bg_color = if v.dark_mode { p.canvas } else { p.surface };
            v.faint_bg_color = p.hover;
            v.code_bg_color = p.hover;
            v.hyperlink_color = p.accent;
            v.error_fg_color = p.danger;
            v.selection.bg_fill = p.selection;
            v.selection.stroke = Stroke::new(1.0, p.on_mark);
            // No shadow at rest; a floating menu or dialog keeps a faint one so it reads as above.
            let lift = if v.dark_mode { Color32::from_black_alpha(90) } else { Color32::from_black_alpha(28) };
            v.popup_shadow = egui::epaint::Shadow { offset: [0, 2], blur: 8, spread: 0, color: lift };
            v.window_shadow = egui::epaint::Shadow { offset: [0, 4], blur: 16, spread: 0, color: lift };
            let w = &mut v.widgets;
            for state in [&mut w.noninteractive, &mut w.inactive, &mut w.hovered, &mut w.active, &mut w.open] {
                state.corner_radius = corner;
                state.expansion = 0.0;
            }
            w.noninteractive.bg_fill = p.surface;
            w.noninteractive.bg_stroke = Stroke::new(1.0, p.rule);
            w.noninteractive.fg_stroke = Stroke::new(1.0, p.text);
            w.inactive.bg_fill = p.pressed;
            w.inactive.weak_bg_fill = Color32::TRANSPARENT;
            w.inactive.bg_stroke = Stroke::new(1.0, p.line);
            w.inactive.fg_stroke = Stroke::new(1.0, p.text);
            w.hovered.bg_fill = p.hover;
            w.hovered.weak_bg_fill = p.hover;
            w.hovered.bg_stroke = Stroke::new(1.0, p.strong_line);
            w.hovered.fg_stroke = Stroke::new(1.0, p.text);
            w.active.bg_fill = p.pressed;
            w.active.weak_bg_fill = p.pressed;
            w.active.bg_stroke = Stroke::new(1.0, p.strong_line);
            w.active.fg_stroke = Stroke::new(1.0, p.text);
            w.open.bg_fill = p.pressed;
            w.open.weak_bg_fill = p.pressed;
            w.open.bg_stroke = Stroke::new(1.0, p.strong_line);
            w.open.fg_stroke = Stroke::new(1.0, p.text);
        });
    }
    ctx.all_styles_mut(|s| {
        s.spacing.interact_size.y = CONTROL_H;
        s.spacing.button_padding = egui::vec2(8.0, 3.0);
        s.spacing.menu_margin = egui::Margin::same(4);
        s.text_styles.insert(egui::TextStyle::Body, crate::theme::ui_font(TYPE_TAB));
        s.text_styles.insert(egui::TextStyle::Button, crate::theme::ui_font(TYPE_TAB));
        s.text_styles.insert(egui::TextStyle::Small, crate::theme::ui_font(TYPE_LABEL));
        s.text_styles.insert(egui::TextStyle::Heading, crate::theme::ui_bold(TYPE_TITLE));
    });
}

// ---------------------------------------------------------------- fonts

/// Inter, the chrome's one family: Regular for text, SemiBold for the active tab, the title and
/// headings. Embedded only in the Pure edition (OFL-1.1, `assets/fonts/OFL-Inter.txt`).
#[cfg(feature = "pure")]
const INTER: &[u8] = include_bytes!("../../../assets/fonts/Inter-Regular.ttf");
#[cfg(feature = "pure")]
const INTER_SEMIBOLD: &[u8] = include_bytes!("../../../assets/fonts/Inter-SemiBold.ttf");
#[cfg(not(feature = "pure"))]
const INTER: &[u8] = &[];
#[cfg(not(feature = "pure"))]
const INTER_SEMIBOLD: &[u8] = &[];

/// Puts Inter at the head of the interface font families (the Pure edition). Whatever upstream
/// bound stays behind it, so scripts and symbols Inter lacks draw as before. Cell fonts are
/// untouched.
pub fn fonts(fonts: &mut egui::FontDefinitions) {
    if !ON || INTER.is_empty() || INTER_SEMIBOLD.is_empty() {
        return;
    }
    for (key, bytes, role) in [("pure-inter", INTER, crate::theme::UI), ("pure-inter-semibold", INTER_SEMIBOLD, crate::theme::UI_BOLD)] {
        fonts.font_data.insert(key.to_string(), std::sync::Arc::new(egui::FontData::from_static(bytes)));
        fonts.families.entry(FontFamily::Name(role.into())).or_default().insert(0, key.to_string());
    }
}

// ---------------------------------------------------------------- the host

impl crate::SheetApp {
    /// Share: tells the host that the person pressed Share on the active workbook. Sharing is the
    /// host's; the app shows nothing itself, and without a host this does nothing.
    pub fn host_share(&self) {
        let Some(share) = &self.services.host_share else { return };
        let name = self.session.active().map(|d| d.display_title()).unwrap_or_default();
        share(&name);
    }
}

// ---------------------------------------------------------------- dialogs

/// A dialog's header: the title at the leading edge, a close button at the far edge, one rule
/// under them. Upstream's title bar centres the title in a grey band.
pub const DIALOG_HEADER: f32 = 44.0;

/// The space around a dialog's content.
pub const DIALOG_MARGIN: i8 = 16;

/// A dialog's OK and Cancel buttons at their least.
pub const DIALOG_BUTTON: egui::Vec2 = egui::vec2(76.0, 28.0);

/// A dialog's frame in the Pure edition: white (the popup colour in dark), one line, 3px corners,
/// its margin, and room at the top for the header [`dialog_header`] paints after it is shown.
pub fn dialog_frame(style: &egui::Style) -> egui::Frame {
    let m = DIALOG_MARGIN;
    egui::Frame::window(style).fill(paper(style.visuals.dark_mode).popup).inner_margin(egui::Margin { left: m, right: m, top: DIALOG_HEADER as i8 + 12, bottom: m })
}

/// Inside a dialog's body: 24px control rows on the 4px step, and room between them.
pub fn dialog_spacing(ui: &mut egui::Ui) {
    let s = ui.spacing_mut();
    s.interact_size.y = CONTROL_H;
    s.item_spacing = egui::vec2(8.0, 6.0);
    s.button_padding = egui::vec2(10.0, 4.0);
}

/// Paints the header of the dialog `window` (the response its `Window::show` returned) on its
/// layer and handles its close button. True when the close button was pressed.
pub fn dialog_header(ctx: &egui::Context, window: &egui::Response, title: &str) -> bool {
    let p = paper(ctx.global_style().visuals.dark_mode);
    let rtl = ctx.global_style().right_to_left;
    let stroke = ctx.global_style().visuals.window_stroke.width;
    let rect = egui::Rect::from_min_size(window.rect.min + egui::vec2(stroke, stroke), egui::vec2(window.rect.width() - 2.0 * stroke, DIALOG_HEADER));
    let id = window.layer_id.id.with("pure_dialog_header");
    let ui = egui::Ui::new(ctx.clone(), id, egui::UiBuilder::new().layer_id(window.layer_id).max_rect(rect));
    let painter = ctx.layer_painter(window.layer_id);
    let galley = painter.layout_no_wrap(title.to_string(), crate::theme::ui_bold(TYPE_TITLE), p.text);
    let inset = f32::from(DIALOG_MARGIN);
    let x = if rtl { rect.max.x - inset - galley.size().x } else { rect.min.x + inset };
    painter.galley(egui::pos2(x, rect.center().y - galley.size().y / 2.0), galley, p.text);
    let side = 28.0;
    let cx = if rtl { rect.min.x + 8.0 + side / 2.0 } else { rect.max.x - 8.0 - side / 2.0 };
    let close = egui::Rect::from_center_size(egui::pos2(cx, rect.center().y), egui::vec2(side, side));
    let r = ui.interact(close, id.with("close"), egui::Sense::click());
    r.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, "Close"));
    let r = r.on_hover_text("Close");
    let corner = egui::CornerRadius::same(RADIUS as u8);
    if r.is_pointer_button_down_on() {
        painter.rect_filled(close, corner, p.pressed);
    } else if r.hovered() {
        painter.rect_filled(close, corner, p.hover);
    }
    let c = egui::pos2(painter.round_to_pixel_center(close.center().x), painter.round_to_pixel_center(close.center().y));
    let s = Stroke::new(1.25, if r.hovered() { p.text } else { p.text_2 });
    let g = 5.0;
    painter.line_segment([c + egui::vec2(-g, -g), c + egui::vec2(g, g)], s);
    painter.line_segment([c + egui::vec2(-g, g), c + egui::vec2(g, -g)], s);
    let y = painter.round_to_pixel_center(rect.max.y - 0.5);
    painter.line_segment([egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)], Stroke::new(1.0, p.rule));
    r.clicked()
}

/// Before a dialog's OK and Cancel: room, and one rule the full width of the dialog.
pub fn dialog_footer_rule(ui: &mut egui::Ui) {
    let p = paper(ui.visuals().dark_mode);
    ui.add_space(12.0);
    let inset = f32::from(DIALOG_MARGIN);
    let r = ui.max_rect();
    let y = ui.painter().round_to_pixel_center(ui.cursor().min.y);
    ui.painter().line_segment([egui::pos2(r.min.x - inset, y), egui::pos2(r.max.x + inset, y)], Stroke::new(1.0, p.rule));
    ui.add_space(12.0);
}

/// A row of tabs in a dialog: 13px labels, the chosen one in the text colour, bold, on a 2px accent
/// rule; the others in the secondary colour with a quiet fill under the pointer; one rule under the
/// row. Upstream shows them as selectable chips. Returns the tab clicked.
pub fn dialog_tabs(ui: &mut egui::Ui, labels: &[&str], selected: usize) -> Option<usize> {
    const H: f32 = 30.0;
    const PAD: f32 = 12.0;
    let p = paper(ui.visuals().dark_mode);
    let font = |on: bool| if on { crate::theme::ui_bold(TYPE_TAB) } else { crate::theme::ui_font(TYPE_TAB) };
    let need: f32 = labels.iter().map(|l| ui.painter().layout_no_wrap(l.to_string(), font(true), p.text).size().x + 2.0 * PAD).sum();
    ui.set_min_width(need);
    let origin = ui.cursor().min;
    let width = ui.available_width();
    let rtl = ui.style().right_to_left;
    let mut x = if rtl { origin.x + width } else { origin.x };
    let mut clicked = None;
    for (i, label) in labels.iter().enumerate() {
        let on = i == selected;
        let w = ui.painter().layout_no_wrap(label.to_string(), font(true), p.text).size().x + 2.0 * PAD;
        let left = if rtl { x - w } else { x };
        let r = egui::Rect::from_min_size(egui::pos2(left, origin.y), egui::vec2(w, H));
        let resp = ui.interact(r, ui.id().with(("pure_dialog_tab", i)), egui::Sense::click());
        resp.widget_info(|| egui::WidgetInfo::selected(egui::Role::Button, true, on, *label));
        let painter = ui.painter();
        if !on && resp.hovered() {
            painter.rect_filled(r.shrink2(egui::vec2(0.0, 3.0)), egui::CornerRadius::same(RADIUS as u8), p.hover);
        }
        let ink = if on || resp.hovered() { p.text } else { p.text_2 };
        let galley = painter.layout_no_wrap(label.to_string(), font(on), ink);
        painter.galley(egui::pos2(r.center().x - galley.size().x / 2.0, r.center().y - galley.size().y / 2.0 - 1.0), galley, ink);
        if on {
            painter.rect_filled(egui::Rect::from_min_max(egui::pos2(r.min.x + 4.0, r.max.y - 2.0), egui::pos2(r.max.x - 4.0, r.max.y)), 0.0, p.accent);
        }
        if resp.clicked() && !on {
            clicked = Some(i);
        }
        x += if rtl { -w } else { w };
    }
    ui.allocate_space(egui::vec2(width, H));
    let y = ui.painter().round_to_pixel_center(origin.y + H - 0.5);
    ui.painter().line_segment([egui::pos2(origin.x, y), egui::pos2(origin.x + width, y)], Stroke::new(1.0, p.rule));
    ui.add_space(4.0);
    clicked
}
