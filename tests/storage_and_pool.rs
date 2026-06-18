use microui::{
    mu_color, mu_rect, mu_vec2, Color, PoolItem, Rect, Vec2, MU_CLIP_ALL, MU_CLIP_PART,
    MU_CONTAINERPOOL_SIZE,
};

#[test]
fn pool_item_layout_matches_expectations() {
    let item = PoolItem {
        id: 7,
        last_update: 9,
    };
    assert_eq!(item.id, 7);
    assert_eq!(item.last_update, 9);
}

#[test]
fn clip_classifier_constants_exist() {
    assert_eq!(MU_CLIP_PART, 1);
    assert_eq!(MU_CLIP_ALL, 2);
    assert_eq!(MU_CONTAINERPOOL_SIZE, 48);
    assert_eq!(mu_vec2(1, 2), Vec2 { x: 1, y: 2 });
    assert_eq!(mu_rect(3, 4, 5, 6), Rect { x: 3, y: 4, w: 5, h: 6 });
    assert_eq!(
        mu_color(7, 8, 9, 10),
        Color {
            r: 7,
            g: 8,
            b: 9,
            a: 10,
        }
    );
}
