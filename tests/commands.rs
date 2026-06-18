use microui::{MU_COMMAND_CLIP, MU_COMMAND_ICON, MU_COMMAND_JUMP, MU_COMMAND_MAX, MU_COMMAND_RECT, MU_COMMAND_TEXT};

#[test]
fn command_constants_are_sequential() {
    assert_eq!(MU_COMMAND_JUMP, 1);
    assert_eq!(MU_COMMAND_CLIP, 2);
    assert_eq!(MU_COMMAND_RECT, 3);
    assert_eq!(MU_COMMAND_TEXT, 4);
    assert_eq!(MU_COMMAND_ICON, 5);
    assert_eq!(MU_COMMAND_MAX, 6);
}
