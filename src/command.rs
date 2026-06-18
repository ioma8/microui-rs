#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandType {
    Jump = crate::MU_COMMAND_JUMP,
    Clip = crate::MU_COMMAND_CLIP,
    Rect = crate::MU_COMMAND_RECT,
    Text = crate::MU_COMMAND_TEXT,
    Icon = crate::MU_COMMAND_ICON,
}
