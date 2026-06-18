use crate::{color, vec2, Color, PoolItem, Vec2, MU_COLOR_MAX, MU_CONTAINERPOOL_SIZE, MU_MAX_FMT,
    MU_TREENODEPOOL_SIZE};

pub type Font = usize;
pub type Real = f32;
pub type Id = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Style {
    pub font: Font,
    pub size: Vec2,
    pub padding: i32,
    pub spacing: i32,
    pub indent: i32,
    pub title_height: i32,
    pub scrollbar_size: i32,
    pub thumb_size: i32,
    pub colors: [Color; MU_COLOR_MAX],
}

impl Default for Style {
    fn default() -> Self {
        Self {
            font: 0,
            size: vec2(68, 10),
            padding: 5,
            spacing: 4,
            indent: 24,
            title_height: 24,
            scrollbar_size: 12,
            thumb_size: 8,
            colors: [
                color(230, 230, 230, 255),
                color(25, 25, 25, 255),
                color(50, 50, 50, 255),
                color(25, 25, 25, 255),
                color(240, 240, 240, 255),
                color(0, 0, 0, 0),
                color(75, 75, 75, 255),
                color(95, 95, 95, 255),
                color(115, 115, 115, 255),
                color(30, 30, 30, 255),
                color(35, 35, 35, 255),
                color(40, 40, 40, 255),
                color(43, 43, 43, 255),
                color(30, 30, 30, 255),
            ],
        }
    }
}

pub struct Context {
    pub style_storage: Style,
    pub style: Option<Style>,
    pub hover: Id,
    pub focus: Id,
    pub frame: i32,
    pub number_edit: Id,
    pub number_edit_buf: [u8; MU_MAX_FMT],
    pub mouse_pos: Vec2,
    pub last_mouse_pos: Vec2,
    pub mouse_delta: Vec2,
    pub scroll_delta: Vec2,
    pub mouse_down: i32,
    pub mouse_pressed: i32,
    pub key_down: i32,
    pub key_pressed: i32,
    pub input_text: [u8; 32],
    pub container_pool: [PoolItem; MU_CONTAINERPOOL_SIZE],
    pub treenode_pool: [PoolItem; MU_TREENODEPOOL_SIZE],
}

impl Default for Context {
    fn default() -> Self {
        Self {
            style_storage: Style::default(),
            style: None,
            hover: 0,
            focus: 0,
            frame: 0,
            number_edit: 0,
            number_edit_buf: [0; MU_MAX_FMT],
            mouse_pos: Vec2::default(),
            last_mouse_pos: Vec2::default(),
            mouse_delta: Vec2::default(),
            scroll_delta: Vec2::default(),
            mouse_down: 0,
            mouse_pressed: 0,
            key_down: 0,
            key_pressed: 0,
            input_text: [0; 32],
            container_pool: [PoolItem::default(); MU_CONTAINERPOOL_SIZE],
            treenode_pool: [PoolItem::default(); MU_TREENODEPOOL_SIZE],
        }
    }
}

impl Context {
    pub fn new() -> Self {
        let style = Style::default();
        Self {
            style_storage: style,
            style: Some(style),
            ..Self::default()
        }
    }
}
