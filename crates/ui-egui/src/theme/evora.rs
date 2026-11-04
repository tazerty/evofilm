//! Evora fork (patch COR-FK-B2, exception E2-L5 on the theme module): Corte's look inside Evora's
//! shell (E2-CO-a and E2-CO-b of Evora's Corte design, `apps/corte/CONCEPTION-PRO.md` §8.3).
//!
//! - **Chrome**: neutral zinc greys of Evora's own design instead of the upstream theme's, in each
//!   of the three themes (darkest, dark, light); the darkest chrome meets the shell's bar without a
//!   seam (`#18181b` on both sides), the monitors are a neutral `#0d0d0d` in every theme.
//! - **Accent**: the highlight colour (the accent `evora:init` names) gives the accent, its hover,
//!   the focus outline, the timecodes, the playhead and track targeting. Their shades come from
//!   Evora's own palettes (`theme/d38.tsv`, built by isoluminance), so they keep the contrasts of the
//!   indigo ones; a colour outside them is mixed towards white or black. Each of them then has a
//!   contrast floor ([`FOCUS_MIN`], [`TEXT_MIN`]) on the surfaces it is drawn on, raised if needed.
//! - **Accessible contrast**: brighter secondary text and borders, as Settings ▸ Appearance does.
//! - **Geometry** (patch EVORA-COR-50): the panels are rounded cards floating on the backdrop, with
//!   even gutters ([`RADIUS`], [`GAP`]); tabs and modes are segmented controls ([`segmented`]); a
//!   hairline ([`panel_edge`]) draws each panel's edge; an empty monitor is a dark screen
//!   ([`MONITOR_LABEL`] on [`MONITOR`]).
//!
//! Off unless the host turns it on ([`set_evora`]), per thread: the upstream themes and their tests
//! are unchanged.

use std::cell::Cell;
use std::sync::OnceLock;

use egui::Color32;

use super::{ThemeKind, Tokens};

thread_local! {
    static ON: Cell<bool> = const { Cell::new(false) };
}

/// Turn Evora's palette on or off for this thread (the UI thread). Off by default.
pub fn set_evora(on: bool) {
    ON.with(|c| c.set(on));
}

/// Whether [`Tokens::for_kind`] gives Evora's palette on this thread.
pub fn evora_enabled() -> bool {
    ON.with(Cell::get)
}

/// Evora's own accent (indigo 600), the shades' starting point when no highlight colour is set.
pub const EVORA_ACCENT: [u8; 3] = [0x4f, 0x46, 0xe5];

/// Least contrast of a non-text element (focus outline, playhead, targeting) on its surface: WCAG
/// 2.1, success criterion 1.4.11.
pub const FOCUS_MIN: f32 = 3.0;
/// Least contrast of text (timecodes, secondary text) on its surface: WCAG 2.1, criterion 1.4.3.
pub const TEXT_MIN: f32 = 4.5;
/// Steps from a shade to white or black when its contrast is raised.
const FLOOR_STEPS: u16 = 20;

const fn rgb(v: u32) -> Color32 {
    Color32::from_rgb((v >> 16) as u8, (v >> 8) as u8, v as u8)
}

// Evora's neutral greys (zinc), the design's own values.
const Z50: Color32 = rgb(0xfafafa);
const Z100: Color32 = rgb(0xf4f4f5);
const Z200: Color32 = rgb(0xe4e4e7);
const Z300: Color32 = rgb(0xd4d4d8);
const Z400: Color32 = rgb(0xa1a1aa);
const Z500: Color32 = rgb(0x71717a);
const Z600: Color32 = rgb(0x52525b);
const Z700: Color32 = rgb(0x3f3f46);
const Z800: Color32 = rgb(0x27272a);
const Z900: Color32 = rgb(0x18181b);
const Z950: Color32 = rgb(0x09090b);
/// The monitors' surround, the same neutral in every theme.
pub const MONITOR: Color32 = rgb(0x0d0d0d);
/// Text drawn on an empty monitor (the monitor is dark in every theme).
pub const MONITOR_LABEL: Color32 = Z400;

/// Corner radius of a panel (a rounded card on the backdrop).
pub const RADIUS: f32 = 10.0;
/// Corner radius of controls: buttons, fields, segments, menus' rows.
pub const RADIUS_SM: f32 = 6.0;
/// Gutter between panels, and between the panels and the window's edges.
pub const GAP: f32 = 6.0;
/// Height of a panel's tab strip.
pub const TAB_H: f32 = 34.0;

/// Track, selected segment and the selected segment's edge of Evora's segmented controls (a
/// panel's tabs, the header's Import / Edit / Export) in theme `kind`.
pub fn segmented(kind: ThemeKind) -> (Color32, Color32, Color32) {
    match kind {
        ThemeKind::Dark => (Z800, Z600, Z600),
        ThemeKind::Medium => (Z900, Z600, Z600),
        ThemeKind::Light => (Z200, Color32::WHITE, Z300),
    }
}

/// The hairline drawn along a panel's edge in theme `kind`.
pub fn panel_edge(kind: ThemeKind) -> Color32 {
    match kind {
        ThemeKind::Dark => Z800,
        ThemeKind::Medium => Z700,
        ThemeKind::Light => Z300,
    }
}

/// The chrome of theme `t.kind` in Evora's greys; the other tokens (audio controls, plots, render
/// and caption colours, danger) stay the theme's.
pub(super) fn chrome(t: Tokens) -> Tokens {
    match t.kind {
        ThemeKind::Dark => Tokens {
            app_bg: Z950,
            header_bg: Z900,
            panel_bg: Z900,
            tab_bg: Z900,
            tab_text: Z400,
            tab_text_active: Z100,
            text: Z200,
            text_dim: Z400,
            text_faint: Z500,
            icon: Z400,
            icon_active: Z100,
            hover: Z800,
            pressed: Z700,
            field_bg: Z950,
            field_border: Z700,
            separator: Z800,
            row_alt: rgb(0x1f1f23),
            row_selected: Z700,
            pill_active_bg: Z700,
            button_hover: Z800,
            button_border: Z700,
            tl_bg: Z900,
            tl_track_bg: Z900,
            tl_track_bg_alt: Z900,
            tl_header_bg: Z900,
            tl_ruler_bg: Z900,
            tl_ruler_tick: Z500,
            tl_ruler_text: Z400,
            in_out_shade: Z700,
            clip_selected_border: Z50,
            monitor_bg: MONITOR,
            radius: RADIUS,
            radius_sm: RADIUS_SM,
            gap: GAP,
            tab_h: TAB_H,
            ..t
        },
        ThemeKind::Medium => Tokens {
            app_bg: Z900,
            header_bg: Z800,
            panel_bg: Z800,
            tab_bg: Z800,
            tab_text: Z400,
            tab_text_active: Z100,
            text: Z200,
            text_dim: Z400,
            text_faint: Z500,
            icon: Z400,
            icon_active: Z100,
            hover: Z700,
            pressed: Z600,
            field_bg: Z900,
            field_border: Z600,
            separator: Z700,
            row_alt: rgb(0x2c2c30),
            row_selected: Z600,
            pill_active_bg: Z600,
            button_hover: Z700,
            button_border: Z600,
            tl_bg: Z800,
            tl_track_bg: Z800,
            tl_track_bg_alt: Z800,
            tl_header_bg: Z800,
            tl_ruler_bg: Z800,
            tl_ruler_tick: Z500,
            tl_ruler_text: Z400,
            in_out_shade: Z600,
            clip_selected_border: Z50,
            monitor_bg: MONITOR,
            radius: RADIUS,
            radius_sm: RADIUS_SM,
            gap: GAP,
            tab_h: TAB_H,
            ..t
        },
        ThemeKind::Light => Tokens {
            app_bg: Z200,
            header_bg: Color32::WHITE,
            panel_bg: Color32::WHITE,
            tab_bg: Color32::WHITE,
            tab_text: Z600,
            tab_text_active: Z900,
            text: Z900,
            text_dim: Z600,
            text_faint: Z500,
            icon: Z600,
            icon_active: Z900,
            hover: Z100,
            pressed: Z200,
            field_bg: Z50,
            field_border: Z400,
            separator: Z200,
            row_alt: rgb(0xf7f7f8),
            row_selected: Z200,
            pill_active_bg: Z200,
            button_hover: Z100,
            button_border: Z300,
            tl_bg: Z100,
            tl_track_bg: Color32::WHITE,
            tl_track_bg_alt: Z50,
            tl_header_bg: Z50,
            tl_ruler_bg: Color32::WHITE,
            tl_ruler_tick: Z400,
            tl_ruler_text: Z600,
            in_out_shade: Z200,
            clip_selected_border: Z900,
            monitor_bg: MONITOR,
            radius: RADIUS,
            radius_sm: RADIUS_SM,
            gap: GAP,
            tab_h: TAB_H,
            ..t
        },
    }
}

/// Settings ▸ Appearance on top of Evora's chrome: accessible contrast, then the highlight's shades
/// with their contrast floors (see the module's documentation).
pub(super) fn with_appearance(mut t: Tokens, highlight: Option<[u8; 3]>, accessible_contrast: bool) -> Tokens {
    if accessible_contrast {
        match t.kind {
            // inactive tabs keep a secondary colour (EVORA-COR-50): the selected segment marks the
            // shown tab, and secondary text already meets 4.5:1 on the tab strip
            ThemeKind::Dark => {
                t.text_dim = Z300;
                t.text_faint = Z400;
                t.tab_text = Z400;
                t.field_border = Z500;
                t.separator = Z700;
            }
            ThemeKind::Medium => {
                t.text_dim = Z300;
                t.text_faint = Z400;
                t.tab_text = Z400;
                t.field_border = Z400;
                t.separator = Z600;
            }
            ThemeKind::Light => {
                t.text_dim = Z700;
                t.text_faint = Z600;
                t.tab_text = Z600;
                t.field_border = Z500;
                t.separator = Z300;
            }
        }
    }
    let [s300, s400, s500, s600, s700] = shades(highlight.unwrap_or(EVORA_ACCENT));
    let light = t.kind == ThemeKind::Light;
    t.accent = s600;
    t.accent_hover = s700;
    // light shades on the dark themes, deeper ones on the light theme
    let (focus, text, mark) = if light { (s600, s700, s600) } else { (s400, s300, s400) };
    t.focus = floor(focus, &[t.panel_bg, t.tab_bg], FOCUS_MIN, light);
    t.timecode = floor(text, &[t.panel_bg, t.tl_track_bg, t.tl_track_bg_alt], TEXT_MIN, light);
    t.hot_text = t.timecode;
    t.playhead = floor(mark, &[t.tl_track_bg, t.tl_track_bg_alt, t.tl_ruler_bg], FOCUS_MIN, light);
    t.track_target = floor(s500, &[t.tl_header_bg, t.panel_bg], FOCUS_MIN, light);
    t
}

/// Evora's palettes: per hue, the 300, 400, 500, 600 and 700 shades of the accent.
fn palettes() -> &'static [[Color32; 5]] {
    static TABLE: OnceLock<Vec<[Color32; 5]>> = OnceLock::new();
    TABLE.get_or_init(|| include_str!("d38.tsv").lines().filter_map(parse_row).collect())
}

fn parse_row(line: &str) -> Option<[Color32; 5]> {
    if line.starts_with('#') {
        return None;
    }
    let mut cols = line.split_whitespace().skip(1).map(parse_color);
    let mut row = [Color32::BLACK; 5];
    for c in &mut row {
        *c = cols.next()??;
    }
    Some(row)
}

fn parse_color(s: &str) -> Option<Color32> {
    if s.len() != 6 {
        return None;
    }
    u32::from_str_radix(s, 16).ok().map(rgb)
}

/// Whether `accent` is the 600 shade of one of Evora's palettes (what Evora's shell sends).
pub fn in_palette(accent: [u8; 3]) -> bool {
    let a = Color32::from_rgb(accent[0], accent[1], accent[2]);
    palettes().iter().any(|r| r[3] == a)
}

/// The 300, 400, 500, 600 and 700 shades of `accent`: its palette's, else mixed towards white and
/// black (a highlight colour of the user's own).
fn shades(accent: [u8; 3]) -> [Color32; 5] {
    let a = Color32::from_rgb(accent[0], accent[1], accent[2]);
    if let Some(row) = palettes().iter().find(|r| r[3] == a) {
        return *row;
    }
    [mix(a, Color32::WHITE, 0.55), mix(a, Color32::WHITE, 0.32), mix(a, Color32::WHITE, 0.12), a, mix(a, Color32::BLACK, 0.16)]
}

/// Relative luminance (WCAG 2.1).
pub fn luminance(c: Color32) -> f32 {
    let lin = |v: u8| {
        let s = f32::from(v) / 255.0;
        if s <= 0.040_45 { s / 12.92 } else { ((s + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * lin(c.r()) + 0.7152 * lin(c.g()) + 0.0722 * lin(c.b())
}

/// Contrast ratio of two colours (WCAG 2.1), from 1 to 21.
pub fn contrast(a: Color32, b: Color32) -> f32 {
    let (x, y) = (luminance(a), luminance(b));
    (x.max(y) + 0.05) / (x.min(y) + 0.05)
}

/// `a` moved a fraction `f` of the way to `b`.
fn mix(a: Color32, b: Color32, f: f32) -> Color32 {
    let m = |x: u8, y: u8| (f32::from(x) + (f32::from(y) - f32::from(x)) * f).round().clamp(0.0, 255.0) as u8;
    Color32::from_rgb(m(a.r(), b.r()), m(a.g(), b.g()), m(a.b(), b.b()))
}

/// `c`, or the first step from it towards white (dark themes) or black (light theme) that has at
/// least `min` contrast on every surface of `on`.
fn floor(c: Color32, on: &[Color32], min: f32, light: bool) -> Color32 {
    let target = if light { Color32::BLACK } else { Color32::WHITE };
    (0..=FLOOR_STEPS)
        .map(|i| mix(c, target, f32::from(i) / f32::from(FLOOR_STEPS)))
        .find(|x| on.iter().all(|b| contrast(*x, *b) >= min))
        .unwrap_or(target)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Evora's palette on this test's thread, off again when the test ends (failed or not).
    struct On;

    impl On {
        fn new() -> Self {
            set_evora(true);
            On
        }
    }

    impl Drop for On {
        fn drop(&mut self) {
            set_evora(false);
        }
    }

    const KINDS: [ThemeKind; 3] = [ThemeKind::Dark, ThemeKind::Medium, ThemeKind::Light];

    fn bytes(c: Color32) -> [u8; 3] {
        [c.r(), c.g(), c.b()]
    }

    /// Every pair the design measures, with its floor.
    fn check(t: &Tokens, what: &str) {
        let pairs = [
            ("text", t.text, t.panel_bg, TEXT_MIN),
            ("secondary text", t.text_dim, t.panel_bg, TEXT_MIN),
            ("faint text", t.text_faint, t.panel_bg, TEXT_MIN),
            ("tab text", t.tab_text, t.tab_bg, TEXT_MIN),
            ("field border on field", t.field_border, t.field_bg, FOCUS_MIN),
            ("field border on panel", t.field_border, t.panel_bg, FOCUS_MIN),
            ("focus", t.focus, t.panel_bg, FOCUS_MIN),
            ("timecode on panel", t.timecode, t.panel_bg, TEXT_MIN),
            ("timecode on track", t.timecode, t.tl_track_bg, TEXT_MIN),
            ("drag value", t.hot_text, t.panel_bg, TEXT_MIN),
            ("playhead on track", t.playhead, t.tl_track_bg, FOCUS_MIN),
            ("playhead on alternate track", t.playhead, t.tl_track_bg_alt, FOCUS_MIN),
            ("track targeting", t.track_target, t.panel_bg, FOCUS_MIN),
        ];
        for (name, fg, bg, min) in pairs {
            let r = contrast(fg, bg);
            assert!(r >= min, "{what}: {name} {r:.2}:1 < {min}:1 ({fg:?} on {bg:?})");
        }
    }

    #[test]
    fn evoras_palettes_are_all_there() {
        let rows = palettes();
        assert_eq!(rows.len(), 360, "one row per hue");
        assert!(in_palette(EVORA_ACCENT), "indigo is Evora's own");
        assert_eq!(bytes(rows[277][3]), EVORA_ACCENT);
        assert!(!in_palette([0x2f, 0x6b, 0xdf]), "upstream's highlight is not");
    }

    #[test]
    fn every_palette_keeps_its_contrasts_in_every_theme() {
        let _on = On::new();
        for (hue, row) in palettes().iter().enumerate() {
            for kind in KINDS {
                let t = Tokens::for_kind(kind).with_appearance(Some(bytes(row[3])), true);
                check(&t, &format!("hue {hue}, {kind:?}"));
                assert_eq!(t.accent, row[3], "hue {hue}, {kind:?}: the accent is the one sent");
                assert_eq!(t.kind, kind);
            }
        }
    }

    #[test]
    fn a_colour_of_ones_own_keeps_the_contrasts_too() {
        let _on = On::new();
        for c in [[0xff, 0, 0], [0, 0xff, 0], [0xff, 0xff, 0], [0, 0, 0x80], [0x80, 0x80, 0x80], [0, 0, 0], [0xff, 0xff, 0xff], [0x2f, 0x6b, 0xdf]] {
            for kind in KINDS {
                check(&Tokens::for_kind(kind).with_appearance(Some(c), true), &format!("{c:?}, {kind:?}"));
            }
        }
        // no highlight colour: Evora's indigo
        assert_eq!(bytes(Tokens::for_kind(ThemeKind::Dark).with_appearance(None, true).accent), EVORA_ACCENT);
    }

    #[test]
    fn the_accent_sent_drives_focus_timecodes_playhead_and_targeting() {
        let _on = On::new();
        let rows = palettes();
        let (indigo, teal) = (rows[277][3], rows[195][3]);
        for kind in KINDS {
            let a = Tokens::for_kind(kind).with_appearance(Some(bytes(indigo)), true);
            let b = Tokens::for_kind(kind).with_appearance(Some(bytes(teal)), true);
            assert_eq!(b.accent, teal);
            for (name, x, y) in [("focus", a.focus, b.focus), ("timecode", a.timecode, b.timecode), ("playhead", a.playhead, b.playhead), ("targeting", a.track_target, b.track_target)] {
                assert_ne!(x, y, "{kind:?}: {name} stays indigo under another accent");
            }
        }
    }

    #[test]
    fn the_chrome_meets_the_shell_and_the_monitor_is_neutral() {
        let _on = On::new();
        let dark = Tokens::for_kind(ThemeKind::Dark);
        assert_eq!((dark.header_bg, dark.panel_bg), (Z900, Z900), "the shell's bar is #18181b");
        for kind in KINDS {
            assert_eq!(Tokens::for_kind(kind).monitor_bg, MONITOR, "{kind:?}");
        }
        let light = Tokens::for_kind(ThemeKind::Light).with_appearance(None, true);
        assert!(luminance(light.panel_bg) > luminance(dark.panel_bg));
    }

    #[test]
    fn evoras_panels_are_rounded_cards_and_upstreams_are_not() {
        set_evora(false);
        let up = Tokens::for_kind(ThemeKind::Dark);
        assert_eq!((up.radius, up.radius_sm, up.gap, up.tab_h), (0.0, 4.0, 4.0, 32.0), "upstream geometry unchanged");
        let _on = On::new();
        for kind in KINDS {
            let t = Tokens::for_kind(kind).with_appearance(None, true);
            assert_eq!((t.radius, t.radius_sm, t.gap, t.tab_h), (RADIUS, RADIUS_SM, GAP, TAB_H), "{kind:?}");
            assert_ne!(t.app_bg, t.panel_bg, "{kind:?}: the panels stand out from the backdrop");
            assert_ne!(panel_edge(kind), t.panel_bg, "{kind:?}: the edge is drawn");
        }
        // the band of backdrop that trims the bodies' square corners (dock::paint_group_frame), as wide
        // as the radius and clipped to the frame, reaches past each corner (r·√2 < 2r from the arc's centre)
        assert!(RADIUS * std::f32::consts::SQRT_2 < 2.0 * RADIUS);
        assert!(GAP > 0.0 && RADIUS_SM < RADIUS);
    }

    #[test]
    fn segmented_controls_and_the_empty_monitor_keep_their_contrasts() {
        let _on = On::new();
        for kind in KINDS {
            let t = Tokens::for_kind(kind).with_appearance(None, true);
            let (track, thumb, _) = segmented(kind);
            assert!(contrast(t.tab_text_active, thumb) >= TEXT_MIN, "{kind:?}: shown tab on its segment");
            assert!(contrast(t.tab_text, track) >= TEXT_MIN, "{kind:?}: other tabs on the track");
            assert!(contrast(t.tab_text, t.tab_bg) >= TEXT_MIN, "{kind:?}: other tabs on the strip");
            assert_ne!(t.tab_text, t.tab_text_active, "{kind:?}: the other tabs read as secondary");
        }
        assert!(contrast(MONITOR_LABEL, MONITOR) >= TEXT_MIN);
    }

    #[test]
    fn off_the_upstream_themes_are_unchanged() {
        set_evora(false);
        let t = Tokens::for_kind(ThemeKind::Dark);
        assert_eq!(t.panel_bg, Color32::from_rgb(0x1d, 0x1d, 0x1d));
        assert_eq!(t.monitor_bg, Color32::from_rgb(0x1d, 0x1d, 0x1d));
        {
            let _on = On::new();
            assert_eq!(Tokens::for_kind(ThemeKind::Dark).panel_bg, Z900);
        }
        assert!(!evora_enabled());
        assert_eq!(Tokens::for_kind(ThemeKind::Dark), t);
    }
}
