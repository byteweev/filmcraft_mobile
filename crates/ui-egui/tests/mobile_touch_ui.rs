use filmcraft_ui_egui::dock;
use filmcraft_ui_egui::state::{MobileTab, UiState};
use filmcraft_ui_egui::theme::apply_touch_style;

#[test]
fn mobile_workspace_shape_validates() {
    assert!(dock::WORKSPACES.contains(&"Mobile"), "Mobile must be a built-in workspace");
    let ws = dock::workspace("Mobile");
    assert!(ws.validate().is_ok(), "Mobile workspace layout must be valid");
    let mut panels = Vec::new();
    ws.panels(&mut panels);
    assert!(panels.contains(&dock::PanelKind::Program), "Mobile workspace must contain Program monitor");
    assert!(panels.contains(&dock::PanelKind::Timeline), "Mobile workspace must contain Timeline");
}

#[test]
fn touch_style_configuration() {
    let ctx = egui::Context::default();
    apply_touch_style(&ctx, true);
    let style = ctx.global_style();
    assert_eq!(style.spacing.interact_size.y, 42.0, "Touch mode must have large 42pt touch targets");
    assert_eq!(style.spacing.button_padding, egui::vec2(12.0, 8.0), "Touch mode must have padded buttons");

    apply_touch_style(&ctx, false);
    let desktop_style = ctx.global_style();
    assert_eq!(desktop_style.spacing.interact_size.y, 24.0, "Desktop mode must restore 24pt targets");
    assert_eq!(desktop_style.spacing.button_padding, egui::vec2(8.0, 3.0), "Desktop mode must restore standard button padding");
}

#[test]
fn mobile_tab_enum_and_labels() {
    for tab in MobileTab::ALL {
        let label = tab.label();
        assert!(!label.is_empty(), "Tab {tab:?} must have a non-empty label");
        let serialized = serde_json::to_string(&tab).expect("serialize tab");
        let deserialized: MobileTab = serde_json::from_str(&serialized).expect("deserialize tab");
        assert_eq!(tab, deserialized, "MobileTab must roundtrip through serde");
    }
}

#[test]
fn ui_state_mobile_and_touch_defaults() {
    let state = UiState::default();
    assert!(!state.touch_mode, "Touch mode should default to false");
    assert_eq!(state.mobile_tab, MobileTab::Timeline, "Mobile tab should default to Timeline");

    let json = serde_json::to_value(&state).expect("serialize UiState");
    assert_eq!(json["touch_mode"], false);
    assert_eq!(json["mobile_tab"], "Timeline");
}
