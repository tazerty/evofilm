//! Evora fork (patch EVORA-COR-59, exception E2-L5-ZOOM): the Timeline's vertical zoom.
//!
//! A small vertical slider at the right end of the Timeline's top-left block, above the track
//! headers, beside a magnifier between two vertical arrows. Down: short tracks, whose headers show
//! the track's name and its output toggle only (the eye, or Mute on an audio track). Up: taller
//! tracks, whose headers show every control, with taller thumbnails and waveforms in the clips.
//!
//! The slider moves the video and the audio track heights together, by the same ratio, on a
//! logarithmic scale between [`MIN_H`] and [`MAX_H`] (the bounds of a drag on a header's edge): each
//! step makes the tracks taller by the same factor. Drag the knob or press on the rail; wheel or
//! pinch over it; once it has focus, the arrows, Page Up / Page Down, Home (shortest) and End
//! (tallest); a double-click restores the original heights. The heights are the sequence's view,
//! saved with the project like the horizontal zoom (`EvoFilmApp::sync_timeline_view`).
//!
//! Host mode only ([`crate::theme::evora_enabled`]): in the upstream mode nothing here is drawn and
//! the headers keep their controls at every height.

use egui::{Color32, CursorIcon, Key, Modifiers, Rect, Sense, Shape, Stroke, pos2, vec2};

use crate::EvoFilmApp;
use crate::state::TimelineView;
use crate::theme::Tokens;

/// The slider's bounds: those of a drag on a header's bottom edge (`timeline::draw_headers`).
pub const MIN_H: f32 = 22.0;
pub const MAX_H: f32 = 220.0;
/// A header lower than this is compact: the track's name and its output toggle, nothing else.
pub const COMPACT_BELOW: f32 = 30.0;
/// Width the slider and its glyph take at the right end of the top-left block.
pub const WIDTH: f32 = 30.0;
/// Automation id. While the slider has focus the shortcuts leave its keys to it (`lib.rs`).
pub const AUTO_ID: &str = "timeline.trackHeight";
/// The interface strings of this module. No upstream catalog has them: the display override
/// catalog (`i18n/overrides.tsv`) translates them, and its test accepts them from this list.
pub const STRINGS: [&str; 2] =
    ["Track Height", "Drag, scroll or use the arrow keys to change the height of all tracks. Double-click: original height."];

/// Keyboard steps, as parts of the slider's course.
const KEY_STEP: f32 = 0.05;
const PAGE_STEP: f32 = 0.25;
/// Wheel over the slider: points of scroll for the whole course.
const WHEEL_RANGE_PX: f32 = 600.0;
/// A target closer than this to the current position changes nothing (no churn while held still).
const EPSILON: f32 = 1e-4;
const RAIL_W: f32 = 16.0;
const TRACK_W: f32 = 4.0;
const KNOB_D: f32 = 12.0;
const GLYPH_W: f32 = 10.0;
const GLYPH_H: f32 = 26.0;
const GLYPH_GAP: f32 = 2.0;
const GLYPH_STROKE: f32 = 1.3;
const FOCUS_STROKE: f32 = 1.5;
const KNOB_SHADOW_ALPHA: u8 = 90;

/// Whether a header `h` points high is compact (host mode only).
pub fn compact(h: f32) -> bool {
    crate::theme::evora_enabled() && h < COMPACT_BELOW
}

/// Where height `h` sits on the slider: 0 at [`MIN_H`], 1 at [`MAX_H`]. A height outside the bounds
/// (a project file, a keyboard command) shows at the nearest end.
pub fn position_of(h: f32) -> f32 {
    if h.is_nan() {
        return 0.0;
    }
    ((h.clamp(MIN_H, MAX_H) / MIN_H).ln() / (MAX_H / MIN_H).ln()).clamp(0.0, 1.0)
}

/// The height at slider position `p` (never past a bound, whatever the rounding).
pub fn height_at(p: f32) -> f32 {
    (MIN_H * (MAX_H / MIN_H).powf(p.clamp(0.0, 1.0))).clamp(MIN_H, MAX_H)
}

/// The slider's position for a view: the mean of the video and the audio positions.
pub fn position(v: &TimelineView) -> f32 {
    (position_of(v.video_track_h) + position_of(v.audio_track_h)) / 2.0
}

/// Moves the slider to `p`: both heights change by the same ratio. When one of them meets an end of
/// the course the other takes the rest, so both ends are reached exactly.
pub fn set_position(v: &mut TimelineView, p: f32) {
    if !p.is_finite() {
        return;
    }
    let (video, audio) = shifted(position_of(v.video_track_h), position_of(v.audio_track_h), p.clamp(0.0, 1.0));
    v.video_track_h = height_at(video);
    v.audio_track_h = height_at(audio);
}

/// The original heights (a new sequence's).
pub fn reset(v: &mut TimelineView) {
    let d = TimelineView::default();
    (v.video_track_h, v.audio_track_h) = (d.video_track_h, d.audio_track_h);
}

/// Keys the slider takes while it has focus; `EvoFilmApp::handle_shortcuts` leaves them to it.
pub fn takes_key(key: Key) -> bool {
    matches!(key, Key::ArrowUp | Key::ArrowDown | Key::ArrowLeft | Key::ArrowRight | Key::PageUp | Key::PageDown | Key::Home | Key::End)
}

/// Two positions moved by the same amount so that their mean is `p` (in 0..=1).
fn shifted(a: f32, b: f32, p: f32) -> (f32, f32) {
    let sum = 2.0 * p;
    let d = p - (a + b) / 2.0;
    let (lo, hi) = (a.min(b), a.max(b));
    let (lo, hi) = if hi + d > 1.0 {
        (sum - 1.0, 1.0)
    } else if lo + d < 0.0 {
        (0.0, sum)
    } else {
        (lo + d, hi + d)
    };
    let (lo, hi) = (lo.clamp(0.0, 1.0), hi.clamp(0.0, 1.0));
    if a <= b { (lo, hi) } else { (hi, lo) }
}

/// The rail (left) and the glyph (right) in `area`.
fn parts(area: Rect) -> (Rect, Rect) {
    let rail = Rect::from_min_max(area.min, pos2(area.min.x + RAIL_W, area.max.y));
    let glyph = Rect::from_center_size(pos2(rail.max.x + GLYPH_GAP + GLYPH_W / 2.0, area.center().y), vec2(GLYPH_W, GLYPH_H));
    (rail, glyph)
}

/// The position under the pointer at `y`: the knob's centre follows the pointer along the rail.
fn position_at(rail: Rect, y: f32) -> f32 {
    let travel = rail.height() - KNOB_D;
    if travel <= 0.0 {
        return 0.0;
    }
    (1.0 - (y - rail.min.y - KNOB_D / 2.0) / travel).clamp(0.0, 1.0)
}

/// Where the keys pressed this frame move the slider from `now`, if anywhere. Only unmodified keys:
/// a chord (Shift, Option, Cmd or Ctrl with an arrow, Home or End) stays a shortcut (`lib.rs`).
fn key_target(i: &mut egui::InputState, now: f32) -> Option<f32> {
    if !i.modifiers.is_none() {
        return None;
    }
    let mut p = None;
    let steps = [
        (Key::ArrowUp, KEY_STEP),
        (Key::ArrowRight, KEY_STEP),
        (Key::ArrowDown, -KEY_STEP),
        (Key::ArrowLeft, -KEY_STEP),
        (Key::PageUp, PAGE_STEP),
        (Key::PageDown, -PAGE_STEP),
    ];
    for (key, step) in steps {
        if i.consume_key(Modifiers::NONE, key) {
            p = Some(p.unwrap_or(now) + step);
        }
    }
    if i.consume_key(Modifiers::NONE, Key::Home) {
        p = Some(0.0);
    }
    if i.consume_key(Modifiers::NONE, Key::End) {
        p = Some(1.0);
    }
    p
}

/// Draws the slider in `area` and applies what the user does with it this frame.
pub fn show(app: &mut EvoFilmApp, ui: &mut egui::Ui, area: Rect, t: &Tokens) {
    let (rail, glyph) = parts(area);
    let label = tl!("Track Height");
    let hint = tl!("Drag, scroll or use the arrow keys to change the height of all tracks. Double-click: original height.");
    let id = egui::Id::new(AUTO_ID);
    let resp = ui.interact(rail, id, Sense::click_and_drag()).on_hover_ui(|ui| {
        ui.label(format!("{label}\n{hint}"));
    });
    app.auto.add_response(AUTO_ID, &resp, "Track Height");
    let v = &mut app.ui.timeline;
    let mut target = None;
    // a press of the primary button on the rail moves the knob under the pointer, a drag keeps it
    // there; the slider takes focus, as a dock divider does, so the arrows nudge it afterwards
    if resp.is_pointer_button_down_on() && ui.input(|i| i.pointer.primary_down()) {
        if !resp.has_focus() {
            resp.request_focus();
        }
        if let Some(pointer) = resp.interact_pointer_pos() {
            target = Some(position_at(rail, pointer.y));
        }
    }
    // a press anywhere else gives the keys back (egui keeps focus through a drag elsewhere: a scrub
    // on the ruler or a moved clip would leave the arrows, Home and End to the slider)
    if resp.has_focus() && !resp.hovered() && ui.input(|i| i.pointer.any_pressed()) {
        ui.memory_mut(|m| m.surrender_focus(id));
    }
    if resp.hovered() || resp.dragged() {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);
    }
    // wheel and pinch over the slider and its glyph (the Timeline leaves them alone there)
    if ui.rect_contains_pointer(area) {
        let w = super::timeline::wheel_input(ui.ctx());
        let mut d = w.delta.y / WHEEL_RANGE_PX;
        if (w.pinch - 1.0).abs() > EPSILON {
            d += w.pinch.ln() / (MAX_H / MIN_H).ln();
        }
        if d != 0.0 && d.is_finite() {
            target = Some(position(v) + d);
        }
    }
    if resp.has_focus() {
        ui.memory_mut(|m| m.set_focus_lock_filter(id, egui::EventFilter { horizontal_arrows: true, vertical_arrows: true, ..Default::default() }));
        let now = position(v);
        if let Some(p) = ui.input_mut(|i| key_target(i, now)) {
            target = Some(p);
        }
    }
    if let Some(p) = target
        && (p.clamp(0.0, 1.0) - position(v)).abs() > EPSILON
    {
        set_position(v, p);
        ui.ctx().request_repaint();
    }
    if resp.double_clicked() {
        reset(v);
        ui.ctx().request_repaint();
    }
    let pos = position(v);
    let value = f64::from((pos * 100.0).round());
    resp.widget_info(|| egui::WidgetInfo::slider(true, value, label));
    paint(ui, rail, glyph, pos, resp.has_focus(), t);
}

/// The rail (filled with the accent below the knob), the white knob and the glyph.
fn paint(ui: &egui::Ui, rail: Rect, glyph: Rect, pos: f32, focused: bool, t: &Tokens) {
    let p = ui.painter();
    let track = Rect::from_center_size(rail.center(), vec2(TRACK_W, rail.height()));
    p.rect_filled(track, TRACK_W / 2.0, t.pressed);
    let knob_y = rail.max.y - KNOB_D / 2.0 - pos * (rail.height() - KNOB_D);
    p.rect_filled(Rect::from_min_max(pos2(track.min.x, knob_y), track.max), TRACK_W / 2.0, t.accent);
    let knob = pos2(rail.center().x, knob_y);
    p.circle_filled(knob + vec2(0.0, 1.0), KNOB_D / 2.0 + 0.5, Color32::from_black_alpha(KNOB_SHADOW_ALPHA));
    p.circle_filled(knob, KNOB_D / 2.0, Color32::WHITE);
    if focused {
        p.circle_stroke(knob, KNOB_D / 2.0 + 2.0, Stroke::new(FOCUS_STROKE, t.focus));
    }
    paint_glyph(p, glyph, t.text_dim);
}

/// A magnifier between an arrow up and an arrow down, drawn on a 10 × 26 grid.
fn paint_glyph(p: &egui::Painter, r: Rect, color: Color32) {
    let s = Stroke::new(GLYPH_STROKE, color);
    let at = |x: f32, y: f32| pos2(r.min.x + x, r.min.y + y);
    p.line_segment([at(5.0, 1.2), at(5.0, 5.8)], s);
    p.add(Shape::line(vec![at(2.8, 3.4), at(5.0, 1.2), at(7.2, 3.4)], s));
    p.circle_stroke(at(4.4, 12.4), 2.9, s);
    p.line_segment([at(6.5, 14.5), at(8.6, 16.6)], s);
    p.line_segment([at(5.0, 20.2), at(5.0, 24.8)], s);
    p.add(Shape::line(vec![at(2.8, 22.6), at(5.0, 24.8), at(7.2, 22.6)], s));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Language;

    fn view(video: f32, audio: f32) -> TimelineView {
        TimelineView { video_track_h: video, audio_track_h: audio, ..TimelineView::default() }
    }

    #[test]
    fn the_scale_is_logarithmic_between_the_bounds() {
        assert_eq!(position_of(MIN_H), 0.0);
        assert!((position_of(MAX_H) - 1.0).abs() < 1e-6);
        // the geometric mean of the bounds sits in the middle
        assert!((position_of((MIN_H * MAX_H).sqrt()) - 0.5).abs() < 1e-5);
        for p in [0.0, 0.1, 0.42, 0.9, 1.0] {
            assert!((position_of(height_at(p)) - p).abs() < 1e-5, "{p}");
        }
        // out of bounds: the nearest end; not a number: the bottom, never NaN
        assert_eq!(position_of(8.0), 0.0);
        assert_eq!(position_of(600.0), 1.0);
        assert_eq!(position_of(f32::INFINITY), 1.0);
        assert_eq!(position_of(f32::NAN), 0.0);
    }

    #[test]
    fn the_slider_keeps_the_ratio_and_reaches_both_ends() {
        let mut v = TimelineView::default();
        let ratio = v.video_track_h / v.audio_track_h;
        set_position(&mut v, position(&v) + 0.1);
        assert!(v.video_track_h > 60.0 && v.audio_track_h > 56.0);
        assert!((v.video_track_h / v.audio_track_h - ratio).abs() < 1e-3);
        set_position(&mut v, 0.0);
        assert_eq!((v.video_track_h, v.audio_track_h), (MIN_H, MIN_H));
        set_position(&mut v, 1.0);
        assert_eq!((v.video_track_h, v.audio_track_h), (MAX_H, MAX_H));
        // the mean follows the target exactly, even past one height's end
        let mut v = view(200.0, 60.0);
        set_position(&mut v, 0.9);
        assert!((position(&v) - 0.9).abs() < 1e-5);
        assert!(v.video_track_h <= MAX_H);
        // and past the other end: the shorter height stays at the bottom, the other takes the rest
        let mut v = view(MIN_H, 60.0);
        set_position(&mut v, 0.1);
        assert!((position(&v) - 0.1).abs() < 1e-5);
        assert_eq!(v.video_track_h, MIN_H);
        set_position(&mut v, f32::NAN);
        assert!(position(&v).is_finite());
    }

    #[test]
    fn a_double_click_restores_the_original_heights() {
        let mut v = view(MIN_H, MIN_H);
        reset(&mut v);
        let d = TimelineView::default();
        assert_eq!((v.video_track_h, v.audio_track_h), (d.video_track_h, d.audio_track_h));
    }

    #[test]
    fn headers_are_compact_low_and_in_host_mode_only() {
        crate::theme::set_evora(true);
        assert!(compact(MIN_H));
        assert!(compact(COMPACT_BELOW - 0.5));
        assert!(!compact(COMPACT_BELOW));
        assert!(!compact(TimelineView::default().audio_track_h));
        crate::theme::set_evora(false);
        assert!(!compact(MIN_H));
    }

    #[test]
    fn the_slider_keys_are_the_navigation_keys() {
        for key in [Key::ArrowUp, Key::ArrowDown, Key::ArrowLeft, Key::ArrowRight, Key::PageUp, Key::PageDown, Key::Home, Key::End] {
            assert!(takes_key(key), "{key:?}");
        }
        assert!(!takes_key(Key::Space));
        assert!(!takes_key(Key::S));
    }

    #[test]
    fn the_strings_are_translated_and_used() {
        let source = include_str!("timeline_zoom.rs");
        for s in STRINGS {
            assert!(source.contains(&format!("tl!(\"{s}\")")), "not drawn through tl!: {s:?}");
            // every language has its own text (a missing override would show the English source)
            for lang in Language::ALL {
                assert!(lang == Language::En || lang.tr(s) != s, "{}: {s:?}", lang.code());
            }
        }
        assert_eq!(Language::Fr.tr(STRINGS[0]), "Hauteur des pistes");
        assert_eq!(Language::En.tr(STRINGS[0]), "Track Height");
    }
}
