mod common;

use common::fixture;
use microui::{
    color, rect, vec2, Color, Context, Rect, Vec2, MU_CLIP_ALL, MU_CLIP_PART, MU_COLOR_MAX,
    MU_COMMAND_CLIP, MU_COMMAND_ICON, MU_COMMAND_JUMP, MU_COMMAND_MAX, MU_COMMAND_RECT,
    MU_COMMAND_TEXT, MU_CONTAINERPOOL_SIZE, MU_IDSTACK_SIZE, MU_KEY_BACKSPACE,
    MU_KEY_RETURN, MU_KEY_SHIFT, MU_MAX_FMT, MU_MAX_WIDTHS, MU_MOUSE_LEFT, MU_OPT_AUTOSIZE,
    MU_OPT_POPUP, MU_REAL_FMT, MU_ROOTLIST_SIZE, MU_SLIDER_FMT, MU_TREENODEPOOL_SIZE, MU_VERSION,
};

#[test]
fn constructors_and_version_exist() {
    assert_eq!(MU_VERSION, "2.02");
    assert_eq!(vec2(1, 2), Vec2 { x: 1, y: 2 });
    assert_eq!(rect(3, 4, 5, 6), Rect { x: 3, y: 4, w: 5, h: 6 });
    assert_eq!(color(7, 8, 9, 10), Color { r: 7, g: 8, b: 9, a: 10 });
}

#[test]
fn primitive_fixture_matches_c_oracle() {
    let fixture_text = fixture("tests/fixtures/primitives.txt");
    let actual = format!(
        "version={}\nvec2={:?}\nrect={:?}\ncolor={:?}\n",
        MU_VERSION,
        vec2(1, 2),
        rect(3, 4, 5, 6),
        color(7, 8, 9, 10),
    );
    assert_eq!(actual, fixture_text);
}

#[test]
fn exported_constants_match_c_header() {
    assert_eq!(MU_CLIP_PART, 1);
    assert_eq!(MU_CLIP_ALL, 2);
    assert_eq!(MU_COMMAND_JUMP, 1);
    assert_eq!(MU_COMMAND_CLIP, 2);
    assert_eq!(MU_COMMAND_RECT, 3);
    assert_eq!(MU_COMMAND_TEXT, 4);
    assert_eq!(MU_COMMAND_ICON, 5);
    assert_eq!(MU_COMMAND_MAX, 6);
    assert_eq!(MU_COLOR_MAX, 14);
    assert_eq!(MU_MOUSE_LEFT, 1);
    assert_eq!(MU_KEY_SHIFT, 1);
    assert_eq!(MU_KEY_BACKSPACE, 8);
    assert_eq!(MU_KEY_RETURN, 16);
    assert_eq!(MU_OPT_AUTOSIZE, 1 << 9);
    assert_eq!(MU_OPT_POPUP, 1 << 10);
    assert_eq!(MU_ROOTLIST_SIZE, 32);
    assert_eq!(MU_IDSTACK_SIZE, 32);
    assert_eq!(MU_CONTAINERPOOL_SIZE, 48);
    assert_eq!(MU_TREENODEPOOL_SIZE, 48);
    assert_eq!(MU_MAX_WIDTHS, 16);
    assert_eq!(MU_MAX_FMT, 127);
    assert_eq!(MU_REAL_FMT, "%.3g");
    assert_eq!(MU_SLIDER_FMT, "%.2f");
}

#[test]
fn context_init_sets_default_versioned_state() {
    let ctx = Context::new();
    assert_eq!(ctx.frame, 0);
    assert_eq!(ctx.hover, 0);
    assert_eq!(ctx.focus, 0);
    assert_eq!(ctx.style.title_height, 24);
    assert_eq!(MU_VERSION, "2.02");
}
