//! Wayland layer shell for the windows a compositor places over the
//! desktop: [`WindowKind::Overlay`] (a HUD, captions: transparent, never
//! the focus, above everything on every workspace) and any window the app
//! anchors with `screenAnchor`. Elsewhere — X11, or a compositor without
//! the protocol — the window stays a plain toplevel and the anchoring
//! attributes are ignored, like any attribute the platform cannot honor.

use gtk4 as gtk;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

use nsh_core::desktop::{WindowAnchor, WindowAttributes, WindowLevel};

/// Makes `window` a layer surface when `a` asks for one, and keeps its
/// layer, anchors and margins with the attributes. Safe to call again:
/// the surface is initialized once, before the window is mapped, and the
/// rest is updated in place.
pub(crate) fn sync(window: &gtk::Window, a: &WindowAttributes) {
    let anchor = a
        .screen_anchor
        .flatten()
        .filter(|a| *a != WindowAnchor::None);
    if anchor.is_none() && a.level != Some(WindowLevel::AboveDock) {
        return;
    }
    // Without the protocol (X11, a plain Wayland WM) nothing changes.
    if !gtk4_layer_shell::is_supported() {
        return;
    }
    if !window.is_layer_window() {
        // Must precede the window's realization; [`crate::window::apply`]
        // runs before the first `present` of an app's own window.
        window.init_layer_shell();
        window.set_namespace(Some("nsh"));
        // An overlay never takes the keyboard; an anchored window takes it
        // when the compositor has nothing else for it.
        window.set_keyboard_mode(if a.focusable == Some(false) {
            KeyboardMode::None
        } else {
            KeyboardMode::OnDemand
        });
    }
    // `AboveDock` is what an overlay kind's defaults ask for: above
    // everything, on every workspace. The other levels keep their order.
    window.set_layer(if a.level == Some(WindowLevel::AboveDock) {
        Layer::Overlay
    } else {
        Layer::Top
    });
    let Some(anchor) = anchor else {
        return;
    };
    let margin = a
        .screen_margin
        .flatten()
        .map(|s| (round(s.width.0), round(s.height.0)))
        .unwrap_or((0, 0));
    for (edge, on, m) in edges(anchor, margin) {
        window.set_anchor(edge, on);
        window.set_margin(edge, m);
    }
    // A widget's place is its anchor: it never asks the compositor to move
    // other windows (that is a panel's work, which a plain window does
    // better).
    window.set_exclusive_zone(0);
}

/// The anchors and margins the four edges get: opposite anchored edges
/// stretch the surface, perpendicular ones pin it to a corner.
fn edges(anchor: WindowAnchor, margin: (i32, i32)) -> [(Edge, bool, i32); 4] {
    let (h, v) = margin;
    let (top, bottom, left, right) = match anchor {
        WindowAnchor::All => (true, true, true, true),
        WindowAnchor::Top => (true, false, false, false),
        WindowAnchor::Bottom => (false, true, false, false),
        WindowAnchor::Left => (false, false, true, false),
        WindowAnchor::Right => (false, false, false, true),
        WindowAnchor::TopLeft => (true, false, true, false),
        WindowAnchor::TopRight => (true, false, false, true),
        WindowAnchor::BottomLeft => (false, true, true, false),
        WindowAnchor::BottomRight | WindowAnchor::None => (false, true, false, true),
    };
    [
        (Edge::Top, top, v),
        (Edge::Bottom, bottom, v),
        (Edge::Left, left, h),
        (Edge::Right, right, h),
    ]
}

fn round(v: f32) -> i32 {
    (v.round() as i32).max(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchors_follow_the_edges() {
        let all = edges(WindowAnchor::All, (3, 4));
        assert!(all.iter().all(|(_, on, _)| *on));
        let bottom = edges(WindowAnchor::Bottom, (0, 8));
        assert_eq!(bottom[1], (Edge::Bottom, true, 8));
        assert!(!bottom[0].1 && !bottom[2].1 && !bottom[3].1);
        let corner = edges(WindowAnchor::BottomRight, (0, 0));
        assert!(corner[1].1 && corner[3].1 && !corner[0].1 && !corner[2].1);
    }

    #[test]
    fn margins_round_and_stay_positive() {
        assert_eq!(round(-5.6), 0, "a negative margin is not an inset");
        assert_eq!(round(7.4), 7);
    }
}
