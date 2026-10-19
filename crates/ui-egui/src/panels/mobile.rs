//! Dedicated mobile and touch-screen editing workspace.
//!
//! Compact, stacked responsive layout:
//! - Top: Program Monitor preview with sequence aspect ratio preservation.
//! - Middle: Touch Control Bar (Playhead timecode, large Play/Pause, Step ±1 frame, Split/Add Edit,
//!   Ripple Delete, Mark In/Out) and Jog Shuttle thumb scrubber with tactile graduations.
//! - Lower Pane: Tabbed active view (Timeline, Media, Inspector, Effects, Export).
//! - Bottom: Finger-friendly navigation bar with 44+ pt tap targets.

use egui::{Align2, Color32, Rect, Sense, Stroke, pos2, vec2};
use serde_json::json;

use crate::FilmcraftApp;
use crate::dock::PanelKind;
use crate::icons::{self, Icon};
use crate::state::MobileTab;
use crate::theme::Tokens;

/// Render the mobile stacked editing layout.
pub fn show(app: &mut FilmcraftApp, ui: &mut egui::Ui, body: Rect) {
    let t = app.tokens;
    let painter = ui.painter().clone();

    // Background fill
    painter.rect_filled(body, 0.0, t.app_bg);

    let nav_h = 48.0;
    let control_h = 76.0;
    let available = (body.height() - nav_h - control_h).max(180.0);

    // Give the Program Monitor ~40-44% of available vertical space, leaving plenty for the lower pane
    let monitor_h = (available * 0.42).clamp(140.0, (available - 140.0).max(140.0));
    let lower_h = (available - monitor_h).max(140.0);

    // 1. Top Section: Program Monitor
    let monitor_rect = Rect::from_min_size(body.min, vec2(body.width(), monitor_h));
    app.panel_body(ui, PanelKind::Program, monitor_rect);

    // 2. Middle Section: Mobile Touch Action Bar & Jog Shuttle Scrubber
    let control_rect = Rect::from_min_size(pos2(body.min.x, monitor_rect.max.y), vec2(body.width(), control_h));
    painter.rect_filled(control_rect, 0.0, t.header_bg);
    painter.line_segment([control_rect.left_top(), control_rect.right_top()], Stroke::new(1.0, t.separator));
    painter.line_segment([control_rect.left_bottom(), control_rect.right_bottom()], Stroke::new(1.0, t.separator));

    show_touch_controls(app, ui, control_rect, &t);

    // 3. Lower Section: Active Tabbed Panel
    let lower_rect = Rect::from_min_size(pos2(body.min.x, control_rect.max.y), vec2(body.width(), lower_h));
    match app.ui.mobile_tab {
        MobileTab::Timeline => {
            app.panel_body(ui, PanelKind::Timeline, lower_rect);
        }
        MobileTab::Media => {
            app.panel_body(ui, PanelKind::Project, lower_rect);
        }
        MobileTab::Inspector => {
            app.panel_body(ui, PanelKind::EffectControls, lower_rect);
        }
        MobileTab::Effects => {
            app.panel_body(ui, PanelKind::Effects, lower_rect);
        }
        MobileTab::Export => {
            app.panel_body(ui, PanelKind::Properties, lower_rect);
        }
    }

    // 4. Bottom Section: Mobile Navigation Bar
    let nav_rect = Rect::from_min_size(pos2(body.min.x, body.max.y - nav_h), vec2(body.width(), nav_h));
    painter.rect_filled(nav_rect, 0.0, t.panel_bg);
    painter.line_segment([nav_rect.left_top(), nav_rect.right_top()], Stroke::new(1.0, t.separator));

    show_bottom_nav(app, ui, nav_rect, &t);
}

/// Upper action bar with Play/Pause, step, split, delete, and jog shuttle thumb scrubber.
fn show_touch_controls(app: &mut FilmcraftApp, ui: &mut egui::Ui, rect: Rect, t: &Tokens) {
    let p = ui.painter().clone();
    let action_h = 40.0;
    let jog_h = rect.height() - action_h;

    let action_rect = Rect::from_min_size(rect.min, vec2(rect.width(), action_h));
    let jog_rect = Rect::from_min_size(pos2(rect.min.x, action_rect.max.y), vec2(rect.width(), jog_h));

    // --- Action Bar ---
    // Timecode readout on the far left
    let rate = app.session.sequence_rate();
    let head = app.session.playhead();
    let tc_str = filmcraft_time::format_timecode_frames(rate.frame_at(head), rate, false);

    let tc_rect = Rect::from_min_size(pos2(action_rect.min.x + 8.0, action_rect.min.y + 6.0), vec2(88.0, 28.0));
    p.rect_filled(tc_rect, 4.0, t.control_well);
    p.text(tc_rect.center(), Align2::CENTER_CENTER, tc_str, Tokens::timecode(), t.hot_text);
    app.auto.add("mobile.timecode", tc_rect, "Timecode");

    // Action buttons: MarkIn, StepBack, Play/Pause, StepFwd, MarkOut, Split, RippleDelete
    let mut bx = action_rect.max.x - 8.0;

    // Helper closure to render a touch button
    let mut touch_btn = |ui: &mut egui::Ui, icon: Icon, id: &str, tip: &str, accent: bool, app: &mut FilmcraftApp| -> egui::Response {
        bx -= 36.0;
        let r = Rect::from_center_size(pos2(bx + 16.0, action_rect.center().y), vec2(32.0, 32.0));
        let resp = ui.interact(r, egui::Id::new(("mobile", id)), Sense::click()).on_hover_text(tip);
        app.auto.add(&format!("mobile.{id}"), r, tip);
        if accent {
            p.rect_filled(r, 6.0, if resp.hovered() { t.accent_hover } else { t.accent });
            icons::paint(&p, r.shrink(7.0), icon, Color32::WHITE);
        } else {
            if resp.hovered() {
                p.rect_filled(r, 6.0, t.hover);
            }
            icons::paint(&p, r.shrink(7.0), icon, if resp.hovered() { t.text } else { t.text_dim });
        }
        resp
    };

    // Ripple Delete
    if touch_btn(ui, Icon::Trash, "delete", "Ripple Delete", false, app).clicked() {
        let _ = app.session.execute("edit.rippleDelete", json!({}));
    }

    // Split / Add Edit at playhead
    if touch_btn(ui, Icon::Razor, "split", "Split", false, app).clicked() {
        let _ = app.session.execute("sequence.addEdit", json!({}));
    }

    // Mark Out
    if touch_btn(ui, Icon::MarkOut, "markOut", "Mark Out", false, app).clicked() {
        let _ = app.session.execute("sequence.setMarkOut", json!({}));
    }

    // Step Forward 1 frame
    if touch_btn(ui, Icon::StepFwd, "stepFwd", "Step Forward", false, app).clicked() {
        let _ = app.session.execute("playback.stepForward", json!({}));
    }

    // Play / Pause (central prominent button)
    let playing = app.playback.playing;
    let play_icon = if playing { Icon::Pause } else { Icon::Play };
    if touch_btn(ui, play_icon, "play", "Play/Pause", true, app).clicked() {
        app.toggle_play(1.0);
    }

    // Step Back 1 frame
    if touch_btn(ui, Icon::StepBack, "stepBack", "Step Backward", false, app).clicked() {
        let _ = app.session.execute("playback.stepBackward", json!({}));
    }

    // Mark In
    if touch_btn(ui, Icon::MarkIn, "markIn", "Mark In", false, app).clicked() {
        let _ = app.session.execute("sequence.setMarkIn", json!({}));
    }

    // --- Jog Shuttle Ribbon ---
    p.rect_filled(jog_rect, 0.0, t.control_well);
    p.line_segment([jog_rect.left_top(), jog_rect.right_top()], Stroke::new(1.0, t.separator));

    // Draw tactile graduations on the jog ribbon
    let mid_x = jog_rect.center().x;
    let graduation_spacing = 16.0;
    let count = (jog_rect.width() / (2.0 * graduation_spacing)) as i32;

    for i in -count..=count {
        let x = mid_x + i as f32 * graduation_spacing;
        if x > jog_rect.min.x + 4.0 && x < jog_rect.max.x - 4.0 {
            let is_major = i % 5 == 0;
            let tick_h = if is_major { 16.0 } else { 8.0 };
            let y0 = jog_rect.center().y - tick_h / 2.0;
            let y1 = jog_rect.center().y + tick_h / 2.0;
            let col = if i == 0 { t.accent } else if is_major { t.text_dim } else { t.text_faint };
            p.line_segment([pos2(x, y0), pos2(x, y1)], Stroke::new(if i == 0 { 2.0 } else { 1.0 }, col));
        }
    }

    // Center indicator needle
    p.line_segment([pos2(mid_x, jog_rect.min.y + 2.0), pos2(mid_x, jog_rect.max.y - 2.0)], Stroke::new(2.0, t.accent));

    let jog_resp = ui.interact(jog_rect, egui::Id::new("mobile-jog-shuttle"), Sense::click_and_drag());
    app.auto.add("mobile.jog", jog_rect, "Jog Shuttle");

    if jog_resp.dragged() {
        let dx = jog_resp.drag_delta().x;
        if dx.abs() > 0.5 {
            // Sensitivity: ~12 pixels per frame
            let frames_delta = (dx / 12.0).round() as i64;
            if frames_delta != 0 {
                let frame_dur = rate.frame_duration();
                let delta_ticks = filmcraft_time::Tick(frame_dur.0.saturating_mul(frames_delta));
                let new_time = (head + delta_ticks).max(filmcraft_time::Tick::ZERO);
                app.session.set_playhead(new_time);
            }
        }
    }
}

/// Bottom navigation bar for thumb-switching tabs on phones and tablets.
fn show_bottom_nav(app: &mut FilmcraftApp, ui: &mut egui::Ui, rect: Rect, t: &Tokens) {
    let p = ui.painter().clone();
    let tabs = MobileTab::ALL;
    let tab_w = rect.width() / tabs.len() as f32;

    for (i, tab) in tabs.iter().enumerate() {
        let tx0 = rect.min.x + i as f32 * tab_w;
        let tab_rect = Rect::from_min_size(pos2(tx0, rect.min.y), vec2(tab_w, rect.height()));
        let active = app.ui.mobile_tab == *tab;

        let resp = ui.interact(tab_rect, egui::Id::new(("mobile-nav", tab.label())), Sense::click());
        app.auto.add(&format!("mobile.tab.{}", tab.label().to_ascii_lowercase()), tab_rect, tab.label());

        if active {
            p.rect_filled(tab_rect.shrink2(vec2(4.0, 4.0)), 6.0, t.hover);
            p.line_segment([pos2(tab_rect.min.x + 8.0, tab_rect.min.y + 2.0), pos2(tab_rect.max.x - 8.0, tab_rect.min.y + 2.0)], Stroke::new(2.0, t.accent));
        }

        let icon = match tab {
            MobileTab::Timeline => Icon::Film,
            MobileTab::Media => Icon::Folder,
            MobileTab::Inspector => Icon::Wrench,
            MobileTab::Effects => Icon::Fx,
            MobileTab::Export => Icon::Export,
        };

        let icon_rect = Rect::from_center_size(pos2(tab_rect.center().x, tab_rect.min.y + 16.0), vec2(18.0, 18.0));
        let col = if active { t.tab_text_active } else { t.tab_text };
        icons::paint(&p, icon_rect, icon, col);

        p.text(pos2(tab_rect.center().x, tab_rect.min.y + 34.0), Align2::CENTER_CENTER, tab.label(), Tokens::ui(11.0), col);

        if resp.clicked() {
            app.ui.mobile_tab = *tab;
        }
    }
}
