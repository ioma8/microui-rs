mod common;

use common::fixture;
use microui::{color, rect, vec2, Color, Rect, Vec2, MU_VERSION};

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
