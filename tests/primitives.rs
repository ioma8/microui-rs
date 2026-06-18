use microui::{color, rect, vec2, Color, Rect, Vec2, MU_VERSION};

#[test]
fn constructors_and_version_exist() {
    assert_eq!(MU_VERSION, "2.02");
    assert_eq!(vec2(1, 2), Vec2 { x: 1, y: 2 });
    assert_eq!(rect(3, 4, 5, 6), Rect { x: 3, y: 4, w: 5, h: 6 });
    assert_eq!(color(7, 8, 9, 10), Color { r: 7, g: 8, b: 9, a: 10 });
}
