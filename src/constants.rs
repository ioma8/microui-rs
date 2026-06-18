pub const MU_VERSION: &str = "2.02";

pub const MU_COMMANDLIST_SIZE: usize = 256 * 1024;
pub const MU_ROOTLIST_SIZE: usize = 32;
pub const MU_CONTAINERSTACK_SIZE: usize = 32;
pub const MU_CLIPSTACK_SIZE: usize = 32;
pub const MU_IDSTACK_SIZE: usize = 32;
pub const MU_LAYOUTSTACK_SIZE: usize = 16;
pub const MU_CONTAINERPOOL_SIZE: usize = 48;
pub const MU_TREENODEPOOL_SIZE: usize = 48;
pub const MU_MAX_WIDTHS: usize = 16;
pub const MU_MAX_FMT: usize = 127;

pub const MU_REAL_FMT: &str = "%.3g";
pub const MU_SLIDER_FMT: &str = "%.2f";

pub const MU_CLIP_PART: i32 = 1;
pub const MU_CLIP_ALL: i32 = 2;

pub const MU_COMMAND_JUMP: i32 = 1;
pub const MU_COMMAND_CLIP: i32 = 2;
pub const MU_COMMAND_RECT: i32 = 3;
pub const MU_COMMAND_TEXT: i32 = 4;
pub const MU_COMMAND_ICON: i32 = 5;
pub const MU_COMMAND_MAX: i32 = 6;

pub const MU_COLOR_MAX: usize = 14;

pub const MU_MOUSE_LEFT: i32 = 1 << 0;
pub const MU_MOUSE_RIGHT: i32 = 1 << 1;
pub const MU_MOUSE_MIDDLE: i32 = 1 << 2;

pub const MU_KEY_SHIFT: i32 = 1 << 0;
pub const MU_KEY_CTRL: i32 = 1 << 1;
pub const MU_KEY_ALT: i32 = 1 << 2;
pub const MU_KEY_BACKSPACE: i32 = 1 << 3;
pub const MU_KEY_RETURN: i32 = 1 << 4;

pub const MU_OPT_AUTOSIZE: i32 = 1 << 9;
pub const MU_OPT_POPUP: i32 = 1 << 10;
