use microui::{
    CommandType, MU_COMMAND_CLIP, MU_COMMAND_ICON, MU_COMMAND_JUMP, MU_COMMAND_RECT,
    MU_COMMAND_TEXT,
};

#[test]
fn command_type_constants_are_mapped() {
    assert_eq!(CommandType::Jump as i32, MU_COMMAND_JUMP);
    assert_eq!(CommandType::Clip as i32, MU_COMMAND_CLIP);
    assert_eq!(CommandType::Rect as i32, MU_COMMAND_RECT);
    assert_eq!(CommandType::Text as i32, MU_COMMAND_TEXT);
    assert_eq!(CommandType::Icon as i32, MU_COMMAND_ICON);
}
