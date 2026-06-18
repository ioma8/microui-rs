use std::cmp::{max, min};
use std::ffi::{c_char, c_double, c_int, CStr, CString};
use std::mem::size_of;

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

pub const MU_COLOR_TEXT: usize = 0;
pub const MU_COLOR_BORDER: usize = 1;
pub const MU_COLOR_WINDOWBG: usize = 2;
pub const MU_COLOR_TITLEBG: usize = 3;
pub const MU_COLOR_TITLETEXT: usize = 4;
pub const MU_COLOR_PANELBG: usize = 5;
pub const MU_COLOR_BUTTON: usize = 6;
pub const MU_COLOR_BUTTONHOVER: usize = 7;
pub const MU_COLOR_BUTTONFOCUS: usize = 8;
pub const MU_COLOR_BASE: usize = 9;
pub const MU_COLOR_BASEHOVER: usize = 10;
pub const MU_COLOR_BASEFOCUS: usize = 11;
pub const MU_COLOR_SCROLLBASE: usize = 12;
pub const MU_COLOR_SCROLLTHUMB: usize = 13;
pub const MU_COLOR_MAX: usize = 14;

pub const MU_ICON_CLOSE: i32 = 1;
pub const MU_ICON_CHECK: i32 = 2;
pub const MU_ICON_COLLAPSED: i32 = 3;
pub const MU_ICON_EXPANDED: i32 = 4;
pub const MU_ICON_MAX: i32 = 5;

pub const MU_RES_ACTIVE: i32 = 1 << 0;
pub const MU_RES_SUBMIT: i32 = 1 << 1;
pub const MU_RES_CHANGE: i32 = 1 << 2;

pub const MU_OPT_ALIGNCENTER: i32 = 1 << 0;
pub const MU_OPT_ALIGNRIGHT: i32 = 1 << 1;
pub const MU_OPT_NOINTERACT: i32 = 1 << 2;
pub const MU_OPT_NOFRAME: i32 = 1 << 3;
pub const MU_OPT_NORESIZE: i32 = 1 << 4;
pub const MU_OPT_NOSCROLL: i32 = 1 << 5;
pub const MU_OPT_NOCLOSE: i32 = 1 << 6;
pub const MU_OPT_NOTITLE: i32 = 1 << 7;
pub const MU_OPT_HOLDFOCUS: i32 = 1 << 8;
pub const MU_OPT_AUTOSIZE: i32 = 1 << 9;
pub const MU_OPT_POPUP: i32 = 1 << 10;
pub const MU_OPT_CLOSED: i32 = 1 << 11;
pub const MU_OPT_EXPANDED: i32 = 1 << 12;

pub const MU_MOUSE_LEFT: i32 = 1 << 0;
pub const MU_MOUSE_RIGHT: i32 = 1 << 1;
pub const MU_MOUSE_MIDDLE: i32 = 1 << 2;

pub const MU_KEY_SHIFT: i32 = 1 << 0;
pub const MU_KEY_CTRL: i32 = 1 << 1;
pub const MU_KEY_ALT: i32 = 1 << 2;
pub const MU_KEY_BACKSPACE: i32 = 1 << 3;
pub const MU_KEY_RETURN: i32 = 1 << 4;

pub type Id = u32;
pub type Real = f32;
pub type Font = usize;
pub type TextWidthFn = fn(Font, &str, i32) -> i32;
pub type TextHeightFn = fn(Font) -> i32;
pub type DrawFrameFn = fn(&mut Context, Rect, i32);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PoolItem {
    pub id: Id,
    pub last_update: i32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BaseCommand {
    pub type_: i32,
    pub size: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct JumpCommand {
    pub base: BaseCommand,
    pub dst: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClipCommand {
    pub base: BaseCommand,
    pub rect: Rect,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RectCommand {
    pub base: BaseCommand,
    pub rect: Rect,
    pub color: Color,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct IconCommand {
    pub base: BaseCommand,
    pub rect: Rect,
    pub id: i32,
    pub color: Color,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextCommand<'a> {
    pub base: BaseCommand,
    pub font: Font,
    pub pos: Vec2,
    pub color: Color,
    pub text: &'a str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command<'a> {
    Jump(JumpCommand),
    Clip(ClipCommand),
    Rect(RectCommand),
    Text(TextCommand<'a>),
    Icon(IconCommand),
}

impl<'a> Command<'a> {
    pub fn type_(&self) -> i32 {
        match self {
            Command::Jump(_) => MU_COMMAND_JUMP,
            Command::Clip(_) => MU_COMMAND_CLIP,
            Command::Rect(_) => MU_COMMAND_RECT,
            Command::Text(_) => MU_COMMAND_TEXT,
            Command::Icon(_) => MU_COMMAND_ICON,
        }
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandType {
    Jump = MU_COMMAND_JUMP,
    Clip = MU_COMMAND_CLIP,
    Rect = MU_COMMAND_RECT,
    Text = MU_COMMAND_TEXT,
    Icon = MU_COMMAND_ICON,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Layout {
    pub body: Rect,
    pub next: Rect,
    pub position: Vec2,
    pub size: Vec2,
    pub max: Vec2,
    pub widths: [i32; MU_MAX_WIDTHS],
    pub items: i32,
    pub item_index: i32,
    pub next_row: i32,
    pub next_type: i32,
    pub indent: i32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Container {
    pub head: Option<usize>,
    pub tail: Option<usize>,
    pub rect: Rect,
    pub body: Rect,
    pub content_size: Vec2,
    pub scroll: Vec2,
    pub zindex: i32,
    pub open: bool,
}

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
        default_style()
    }
}

#[derive(Clone, Debug)]
pub struct FixedStack<T: Copy + Default, const N: usize> {
    pub idx: usize,
    pub items: [T; N],
}

impl<T: Copy + Default, const N: usize> Default for FixedStack<T, N> {
    fn default() -> Self {
        Self {
            idx: 0,
            items: [T::default(); N],
        }
    }
}

impl<T: Copy + Default, const N: usize> FixedStack<T, N> {
    pub fn push(&mut self, value: T) {
        assert!(self.idx < N, "stack overflow");
        self.items[self.idx] = value;
        self.idx += 1;
    }

    pub fn pop(&mut self) -> T {
        assert!(self.idx > 0, "stack underflow");
        self.idx -= 1;
        self.items[self.idx]
    }

    pub fn last(&self) -> T {
        assert!(self.idx > 0, "stack empty");
        self.items[self.idx - 1]
    }
}

#[derive(Clone)]
pub struct Context {
    pub text_width: Option<TextWidthFn>,
    pub text_height: Option<TextHeightFn>,
    pub draw_frame: Option<DrawFrameFn>,
    pub style_storage: Style,
    pub style: Style,
    pub hover: Id,
    pub focus: Id,
    pub last_id: Id,
    pub last_rect: Rect,
    pub last_zindex: i32,
    pub updated_focus: i32,
    pub frame: i32,
    pub hover_root: Option<usize>,
    pub next_hover_root: Option<usize>,
    pub scroll_target: Option<usize>,
    pub number_edit_buf: [u8; MU_MAX_FMT],
    pub number_edit: Id,
    pub command_list: [u8; MU_COMMANDLIST_SIZE],
    pub command_list_idx: usize,
    pub root_list: FixedStack<usize, MU_ROOTLIST_SIZE>,
    pub container_stack: FixedStack<usize, MU_CONTAINERSTACK_SIZE>,
    pub clip_stack: FixedStack<Rect, MU_CLIPSTACK_SIZE>,
    pub id_stack: FixedStack<Id, MU_IDSTACK_SIZE>,
    pub layout_stack: FixedStack<Layout, MU_LAYOUTSTACK_SIZE>,
    pub container_pool: [PoolItem; MU_CONTAINERPOOL_SIZE],
    pub containers: [Container; MU_CONTAINERPOOL_SIZE],
    pub treenode_pool: [PoolItem; MU_TREENODEPOOL_SIZE],
    pub mouse_pos: Vec2,
    pub last_mouse_pos: Vec2,
    pub mouse_delta: Vec2,
    pub scroll_delta: Vec2,
    pub mouse_down: i32,
    pub mouse_pressed: i32,
    pub key_down: i32,
    pub key_pressed: i32,
    pub input_text: [u8; 32],
}

impl Default for Context {
    fn default() -> Self {
        Self {
            text_width: None,
            text_height: None,
            draw_frame: Some(default_draw_frame),
            style_storage: default_style(),
            style: default_style(),
            hover: 0,
            focus: 0,
            last_id: 0,
            last_rect: Rect::default(),
            last_zindex: 0,
            updated_focus: 0,
            frame: 0,
            hover_root: None,
            next_hover_root: None,
            scroll_target: None,
            number_edit_buf: [0; MU_MAX_FMT],
            number_edit: 0,
            command_list: [0; MU_COMMANDLIST_SIZE],
            command_list_idx: 0,
            root_list: FixedStack::default(),
            container_stack: FixedStack::default(),
            clip_stack: FixedStack::default(),
            id_stack: FixedStack::default(),
            layout_stack: FixedStack::default(),
            container_pool: [PoolItem::default(); MU_CONTAINERPOOL_SIZE],
            containers: [Container::default(); MU_CONTAINERPOOL_SIZE],
            treenode_pool: [PoolItem::default(); MU_TREENODEPOOL_SIZE],
            mouse_pos: Vec2::default(),
            last_mouse_pos: Vec2::default(),
            mouse_delta: Vec2::default(),
            scroll_delta: Vec2::default(),
            mouse_down: 0,
            mouse_pressed: 0,
            key_down: 0,
            key_pressed: 0,
            input_text: [0; 32],
        }
    }
}

impl Context {
    pub fn new() -> Self {
        let mut ctx = Self::default();
        init(&mut ctx);
        ctx
    }

    pub fn begin(&mut self) {
        begin(self);
    }

    pub fn end(&mut self) {
        end(self);
    }
}

pub trait CheckboxState {
    fn is_checked(&self) -> bool;
    fn toggle(&mut self);
}

impl CheckboxState for bool {
    fn is_checked(&self) -> bool {
        *self
    }

    fn toggle(&mut self) {
        *self = !*self;
    }
}

impl CheckboxState for i32 {
    fn is_checked(&self) -> bool {
        *self != 0
    }

    fn toggle(&mut self) {
        *self = if *self == 0 { 1 } else { 0 };
    }
}

const HASH_INITIAL: Id = 2_166_136_261;
const RELATIVE: i32 = 1;
const ABSOLUTE: i32 = 2;
const UNCLIPPED_RECT: Rect = Rect {
    x: 0,
    y: 0,
    w: 0x1000000,
    h: 0x1000000,
};
const COMMAND_BASE_SIZE: usize = 8;
const JUMP_COMMAND_SIZE: usize = COMMAND_BASE_SIZE + size_of::<usize>();
const CLIP_COMMAND_SIZE: usize = COMMAND_BASE_SIZE + 16;
const RECT_COMMAND_SIZE: usize = COMMAND_BASE_SIZE + 16 + 4;
const ICON_COMMAND_SIZE: usize = COMMAND_BASE_SIZE + 16 + 4 + 4;
const TEXT_COMMAND_FIXED_SIZE: usize = COMMAND_BASE_SIZE + size_of::<usize>() + 8 + 4;

unsafe extern "C" {
    fn snprintf(dst: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn strtod(src: *const c_char, endptr: *mut *mut c_char) -> c_double;
}

pub const fn vec2(x: i32, y: i32) -> Vec2 {
    Vec2 { x, y }
}

pub const fn rect(x: i32, y: i32, w: i32, h: i32) -> Rect {
    Rect { x, y, w, h }
}

pub const fn color(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color { r, g, b, a }
}

pub fn init(ctx: &mut Context) {
    *ctx = Context::default();
}

pub fn begin(ctx: &mut Context) {
    assert!(ctx.text_width.is_some() && ctx.text_height.is_some(), "text callbacks must be set before begin");
    ctx.command_list_idx = 0;
    ctx.root_list.idx = 0;
    ctx.scroll_target = None;
    ctx.hover_root = ctx.next_hover_root;
    ctx.next_hover_root = None;
    ctx.mouse_delta.x = ctx.mouse_pos.x - ctx.last_mouse_pos.x;
    ctx.mouse_delta.y = ctx.mouse_pos.y - ctx.last_mouse_pos.y;
    ctx.frame += 1;
}

pub fn end(ctx: &mut Context) {
    assert!(ctx.container_stack.idx == 0, "container stack not empty");
    assert!(ctx.clip_stack.idx == 0, "clip stack not empty");
    assert!(ctx.id_stack.idx == 0, "id stack not empty");
    assert!(ctx.layout_stack.idx == 0, "layout stack not empty");

    if let Some(idx) = ctx.scroll_target {
        ctx.containers[idx].scroll.x += ctx.scroll_delta.x;
        ctx.containers[idx].scroll.y += ctx.scroll_delta.y;
    }

    if ctx.updated_focus == 0 {
        ctx.focus = 0;
    }
    ctx.updated_focus = 0;

    if ctx.mouse_pressed != 0 {
        if let Some(next_hover_root) = ctx.next_hover_root {
            let z = ctx.containers[next_hover_root].zindex;
            if z < ctx.last_zindex && z >= 0 {
                bring_to_front(ctx, next_hover_root);
            }
        }
    }

    ctx.key_pressed = 0;
    ctx.input_text[0] = 0;
    ctx.mouse_pressed = 0;
    ctx.scroll_delta = vec2(0, 0);
    ctx.last_mouse_pos = ctx.mouse_pos;

    let n = ctx.root_list.idx;
    ctx.root_list.items[..n].sort_by_key(|idx| ctx.containers[*idx].zindex);
    for i in 0..n {
        let cnt_idx = ctx.root_list.items[i];
        let head = ctx.containers[cnt_idx].head.expect("root container missing head");
        let tail = ctx.containers[cnt_idx].tail.expect("root container missing tail");
        if i == 0 {
            write_jump_dst(ctx, 0, head + JUMP_COMMAND_SIZE);
        } else {
            let prev = ctx.root_list.items[i - 1];
            let prev_tail = ctx.containers[prev].tail.expect("previous root missing tail");
            write_jump_dst(ctx, prev_tail, head + JUMP_COMMAND_SIZE);
        }
        if i == n - 1 {
            write_jump_dst(ctx, tail, ctx.command_list_idx);
        }
    }
}

pub fn set_focus(ctx: &mut Context, id: Id) {
    ctx.focus = id;
    ctx.updated_focus = 1;
}

pub fn get_id(ctx: &mut Context, data: &[u8]) -> Id {
    let mut res = if ctx.id_stack.idx > 0 {
        ctx.id_stack.last()
    } else {
        HASH_INITIAL
    };
    hash(&mut res, data);
    ctx.last_id = res;
    res
}

pub fn push_id(ctx: &mut Context, data: &[u8]) {
    let id = get_id(ctx, data);
    ctx.id_stack.push(id);
}

pub fn pop_id(ctx: &mut Context) {
    ctx.id_stack.pop();
}

pub fn push_clip_rect(ctx: &mut Context, r: Rect) {
    let last = get_clip_rect(ctx);
    ctx.clip_stack.push(intersect_rects(r, last));
}

pub fn pop_clip_rect(ctx: &mut Context) {
    ctx.clip_stack.pop();
}

pub fn get_clip_rect(ctx: &Context) -> Rect {
    assert!(ctx.clip_stack.idx > 0, "clip stack empty");
    ctx.clip_stack.last()
}

pub fn check_clip(ctx: &Context, r: Rect) -> i32 {
    let cr = get_clip_rect(ctx);
    if r.x > cr.x + cr.w || r.x + r.w < cr.x || r.y > cr.y + cr.h || r.y + r.h < cr.y {
        return MU_CLIP_ALL;
    }
    if r.x >= cr.x && r.x + r.w <= cr.x + cr.w && r.y >= cr.y && r.y + r.h <= cr.y + cr.h {
        return 0;
    }
    MU_CLIP_PART
}

pub fn get_current_container(ctx: &Context) -> &Container {
    let idx = current_container_index(ctx);
    &ctx.containers[idx]
}

pub fn get_current_container_mut(ctx: &mut Context) -> &mut Container {
    let idx = current_container_index(ctx);
    &mut ctx.containers[idx]
}

pub fn get_container<'a>(ctx: &'a mut Context, name: &str) -> &'a mut Container {
    let id = get_id(ctx, name.as_bytes());
    let idx = get_container_index_internal(ctx, id, 0).expect("container should exist");
    &mut ctx.containers[idx]
}

pub fn bring_to_front(ctx: &mut Context, cnt_idx: usize) {
    ctx.last_zindex += 1;
    ctx.containers[cnt_idx].zindex = ctx.last_zindex;
}

pub fn pool_init(ctx: &mut Context, items: &mut [PoolItem], id: Id) -> usize {
    let mut f = ctx.frame;
    let mut n = None;
    for (i, item) in items.iter().enumerate() {
        if item.last_update < f {
            f = item.last_update;
            n = Some(i);
        }
    }
    let idx = n.expect("pool init failed");
    items[idx].id = id;
    pool_update(ctx, items, idx);
    idx
}

pub fn pool_get(_ctx: &Context, items: &[PoolItem], id: Id) -> Option<usize> {
    items.iter().position(|item| item.id == id)
}

pub fn pool_update(ctx: &Context, items: &mut [PoolItem], idx: usize) {
    items[idx].last_update = ctx.frame;
}

pub fn input_mousemove(ctx: &mut Context, x: i32, y: i32) {
    ctx.mouse_pos = vec2(x, y);
}

pub fn input_mousedown(ctx: &mut Context, x: i32, y: i32, btn: i32) {
    input_mousemove(ctx, x, y);
    ctx.mouse_down |= btn;
    ctx.mouse_pressed |= btn;
}

pub fn input_mouseup(ctx: &mut Context, x: i32, y: i32, btn: i32) {
    input_mousemove(ctx, x, y);
    ctx.mouse_down &= !btn;
}

pub fn input_scroll(ctx: &mut Context, x: i32, y: i32) {
    ctx.scroll_delta.x += x;
    ctx.scroll_delta.y += y;
}

pub fn input_keydown(ctx: &mut Context, key: i32) {
    ctx.key_pressed |= key;
    ctx.key_down |= key;
}

pub fn input_keyup(ctx: &mut Context, key: i32) {
    ctx.key_down &= !key;
}

pub fn input_text(ctx: &mut Context, text: &str) {
    let mut current = c_buf_len(&ctx.input_text);
    let bytes = text.as_bytes();
    assert!(current + bytes.len() + 1 <= ctx.input_text.len(), "input text overflow");
    for byte in bytes {
        ctx.input_text[current] = *byte;
        current += 1;
    }
    ctx.input_text[current] = 0;
}

pub fn push_command(ctx: &mut Context, type_: i32, size: usize) -> usize {
    assert!(ctx.command_list_idx + size < MU_COMMANDLIST_SIZE, "command list overflow");
    let at = ctx.command_list_idx;
    write_i32(&mut ctx.command_list, at, type_);
    write_i32(&mut ctx.command_list, at + 4, size as i32);
    ctx.command_list_idx += size;
    at
}

pub fn next_command<'a>(ctx: &'a Context, cmd: &mut Option<usize>) -> Option<Command<'a>> {
    let mut pos = if let Some(current) = *cmd {
        current + read_size(&ctx.command_list, current)
    } else {
        0
    };
    while pos != ctx.command_list_idx {
        let type_ = read_i32(&ctx.command_list, pos);
        if type_ != MU_COMMAND_JUMP {
            *cmd = Some(pos);
            return Some(parse_command(ctx, pos));
        }
        pos = read_usize(&ctx.command_list, pos + 8);
    }
    *cmd = None;
    None
}

pub fn set_clip(ctx: &mut Context, r: Rect) {
    let at = push_command(ctx, MU_COMMAND_CLIP, CLIP_COMMAND_SIZE);
    write_rect(&mut ctx.command_list, at + 8, r);
}

pub fn draw_rect(ctx: &mut Context, mut r: Rect, c: Color) {
    r = intersect_rects(r, get_clip_rect(ctx));
    if r.w > 0 && r.h > 0 {
        let at = push_command(ctx, MU_COMMAND_RECT, RECT_COMMAND_SIZE);
        write_rect(&mut ctx.command_list, at + 8, r);
        write_color(&mut ctx.command_list, at + 24, c);
    }
}

pub fn draw_box(ctx: &mut Context, r: Rect, c: Color) {
    draw_rect(ctx, rect(r.x + 1, r.y, r.w - 2, 1), c);
    draw_rect(ctx, rect(r.x + 1, r.y + r.h - 1, r.w - 2, 1), c);
    draw_rect(ctx, rect(r.x, r.y, 1, r.h), c);
    draw_rect(ctx, rect(r.x + r.w - 1, r.y, 1, r.h), c);
}

pub fn draw_text(ctx: &mut Context, font: Font, text: &str, len: i32, pos: Vec2, c: Color) {
    let rect = rect(
        pos.x,
        pos.y,
        call_text_width(ctx, font, text, len),
        call_text_height(ctx, font),
    );
    let clipped = check_clip(ctx, rect);
    if clipped == MU_CLIP_ALL {
        return;
    }
    if clipped == MU_CLIP_PART {
        set_clip(ctx, get_clip_rect(ctx));
    }
    let actual_len = if len < 0 { text.len() } else { len as usize };
    let at = push_command(ctx, MU_COMMAND_TEXT, TEXT_COMMAND_FIXED_SIZE + actual_len + 1);
    write_usize(&mut ctx.command_list, at + 8, font);
    write_vec2(&mut ctx.command_list, at + 8 + size_of::<usize>(), pos);
    write_color(&mut ctx.command_list, at + 8 + size_of::<usize>() + 8, c);
    let start = at + TEXT_COMMAND_FIXED_SIZE;
    ctx.command_list[start..start + actual_len].copy_from_slice(&text.as_bytes()[..actual_len]);
    ctx.command_list[start + actual_len] = 0;
    if clipped != 0 {
        set_clip(ctx, UNCLIPPED_RECT);
    }
}

pub fn draw_icon(ctx: &mut Context, id: i32, rect_: Rect, c: Color) {
    let clipped = check_clip(ctx, rect_);
    if clipped == MU_CLIP_ALL {
        return;
    }
    if clipped == MU_CLIP_PART {
        set_clip(ctx, get_clip_rect(ctx));
    }
    let at = push_command(ctx, MU_COMMAND_ICON, ICON_COMMAND_SIZE);
    write_rect(&mut ctx.command_list, at + 8, rect_);
    write_i32(&mut ctx.command_list, at + 24, id);
    write_color(&mut ctx.command_list, at + 28, c);
    if clipped != 0 {
        set_clip(ctx, UNCLIPPED_RECT);
    }
}

pub fn layout_begin_column(ctx: &mut Context) {
    let next = layout_next(ctx);
    push_layout(ctx, next, vec2(0, 0));
}

pub fn layout_end_column(ctx: &mut Context) {
    let b = ctx.layout_stack.pop();
    let a = get_layout_mut(ctx);
    a.position.x = max(a.position.x, b.position.x + b.body.x - a.body.x);
    a.next_row = max(a.next_row, b.next_row + b.body.y - a.body.y);
    a.max.x = max(a.max.x, b.max.x);
    a.max.y = max(a.max.y, b.max.y);
}

pub fn layout_row(ctx: &mut Context, items: i32, widths: Option<&[i32]>, height: i32) {
    let layout = get_layout_mut(ctx);
    if let Some(widths) = widths {
        assert!(items as usize <= MU_MAX_WIDTHS, "too many widths");
        layout.widths[..items as usize].copy_from_slice(&widths[..items as usize]);
    }
    layout.items = items;
    layout.position = vec2(layout.indent, layout.next_row);
    layout.size.y = height;
    layout.item_index = 0;
}

pub fn layout_width(ctx: &mut Context, width: i32) {
    get_layout_mut(ctx).size.x = width;
}

pub fn layout_height(ctx: &mut Context, height: i32) {
    get_layout_mut(ctx).size.y = height;
}

pub fn layout_set_next(ctx: &mut Context, r: Rect, relative: bool) {
    let layout = get_layout_mut(ctx);
    layout.next = r;
    layout.next_type = if relative { RELATIVE } else { ABSOLUTE };
}

pub fn layout_next(ctx: &mut Context) -> Rect {
    let style = ctx.style;
    let layout = get_layout_mut(ctx);
    let mut res;
    if layout.next_type != 0 {
        let typ = layout.next_type;
        layout.next_type = 0;
        res = layout.next;
        if typ == ABSOLUTE {
            ctx.last_rect = res;
            return res;
        }
    } else {
        if layout.item_index == layout.items {
            let height = layout.size.y;
            let items = layout.items;
            layout_row(ctx, items, None, height);
        }
        let layout = get_layout_mut(ctx);
        res = rect(layout.position.x, layout.position.y, 0, 0);
        res.w = if layout.items > 0 {
            layout.widths[layout.item_index as usize]
        } else {
            layout.size.x
        };
        res.h = layout.size.y;
        if res.w == 0 {
            res.w = style.size.x + style.padding * 2;
        }
        if res.h == 0 {
            res.h = style.size.y + style.padding * 2;
        }
        if res.w < 0 {
            res.w += layout.body.w - res.x + 1;
        }
        if res.h < 0 {
            res.h += layout.body.h - res.y + 1;
        }
        layout.item_index += 1;
    }

    let layout = get_layout_mut(ctx);
    layout.position.x += res.w + style.spacing;
    layout.next_row = max(layout.next_row, res.y + res.h + style.spacing);
    res.x += layout.body.x;
    res.y += layout.body.y;
    layout.max.x = max(layout.max.x, res.x + res.w);
    layout.max.y = max(layout.max.y, res.y + res.h);
    ctx.last_rect = res;
    res
}

pub fn draw_control_frame(ctx: &mut Context, id: Id, rect_: Rect, colorid: usize, opt: i32) {
    if opt & MU_OPT_NOFRAME != 0 {
        return;
    }
    let id_color = colorid + if ctx.focus == id { 2 } else if ctx.hover == id { 1 } else { 0 };
    call_draw_frame(ctx, rect_, id_color as i32);
}

pub fn draw_control_text(ctx: &mut Context, text: &str, rect_: Rect, colorid: usize, opt: i32) {
    let font = ctx.style.font;
    let tw = call_text_width(ctx, font, text, -1);
    push_clip_rect(ctx, rect_);
    let mut pos = vec2(0, rect_.y + (rect_.h - call_text_height(ctx, font)) / 2);
    if opt & MU_OPT_ALIGNCENTER != 0 {
        pos.x = rect_.x + (rect_.w - tw) / 2;
    } else if opt & MU_OPT_ALIGNRIGHT != 0 {
        pos.x = rect_.x + rect_.w - tw - ctx.style.padding;
    } else {
        pos.x = rect_.x + ctx.style.padding;
    }
    draw_text(ctx, font, text, -1, pos, ctx.style.colors[colorid]);
    pop_clip_rect(ctx);
}

pub fn mouse_over(ctx: &Context, rect_: Rect) -> bool {
    rect_overlaps_vec2(rect_, ctx.mouse_pos)
        && rect_overlaps_vec2(get_clip_rect(ctx), ctx.mouse_pos)
        && in_hover_root(ctx)
}

pub fn update_control(ctx: &mut Context, id: Id, rect_: Rect, opt: i32) {
    let mouseover = mouse_over(ctx, rect_);
    if ctx.focus == id {
        ctx.updated_focus = 1;
    }
    if opt & MU_OPT_NOINTERACT != 0 {
        return;
    }
    if mouseover && ctx.mouse_down == 0 {
        ctx.hover = id;
    }
    if ctx.focus == id {
        if ctx.mouse_pressed != 0 && !mouseover {
            set_focus(ctx, 0);
        }
        if ctx.mouse_down == 0 && opt & MU_OPT_HOLDFOCUS == 0 {
            set_focus(ctx, 0);
        }
    }
    if ctx.hover == id {
        if ctx.mouse_pressed != 0 {
            set_focus(ctx, id);
        } else if !mouseover {
            ctx.hover = 0;
        }
    }
}

pub fn text(ctx: &mut Context, text: &str) {
    let font = ctx.style.font;
    let color_ = ctx.style.colors[MU_COLOR_TEXT];
    let mut p = 0usize;
    let width = -1;
    layout_begin_column(ctx);
    layout_row(ctx, 1, Some(&[width]), call_text_height(ctx, font));
    let bytes = text.as_bytes();
    loop {
        let r = layout_next(ctx);
        let mut w = 0;
        let start = p;
        let mut end = p;
        loop {
            let word = p;
            while p < bytes.len() && bytes[p] != b' ' && bytes[p] != b'\n' {
                p += 1;
            }
            let word_text = std::str::from_utf8(&bytes[word..p]).unwrap_or("");
            w += call_text_width(ctx, font, word_text, (p - word) as i32);
            if w > r.w && end != start {
                break;
            }
            let ch = if p < bytes.len() { &text[p..p + 1] } else { "" };
            w += call_text_width(ctx, font, ch, 1);
            end = p;
            if p < bytes.len() {
                p += 1;
            }
            if end >= bytes.len() || bytes[end] == b'\n' {
                break;
            }
        }
        let line = std::str::from_utf8(&bytes[start..end]).unwrap_or("");
        draw_text(ctx, font, line, line.len() as i32, vec2(r.x, r.y), color_);
        if end >= bytes.len() {
            break;
        }
        p = end + 1;
    }
    layout_end_column(ctx);
}

pub fn label(ctx: &mut Context, text: &str) {
    let r = layout_next(ctx);
    draw_control_text(ctx, text, r, MU_COLOR_TEXT, 0);
}

pub fn button_ex(ctx: &mut Context, label: Option<&str>, icon: i32, opt: i32) -> i32 {
    let id = if let Some(label) = label {
        get_id(ctx, label.as_bytes())
    } else {
        get_id(ctx, &icon.to_ne_bytes())
    };
    let r = layout_next(ctx);
    update_control(ctx, id, r, opt);
    let mut res = 0;
    if ctx.mouse_pressed == MU_MOUSE_LEFT && ctx.focus == id {
        res |= MU_RES_SUBMIT;
    }
    draw_control_frame(ctx, id, r, MU_COLOR_BUTTON, opt);
    if let Some(label) = label {
        draw_control_text(ctx, label, r, MU_COLOR_TEXT, opt);
    }
    if icon != 0 {
        draw_icon(ctx, icon, r, ctx.style.colors[MU_COLOR_TEXT]);
    }
    res
}

pub fn button(ctx: &mut Context, label: &str) -> i32 {
    button_ex(ctx, Some(label), 0, MU_OPT_ALIGNCENTER)
}

pub fn checkbox<S: CheckboxState>(ctx: &mut Context, label: &str, state: &mut S) -> i32 {
    let ptr = state as *mut S as usize;
    let id = get_id(ctx, &ptr.to_ne_bytes());
    let mut r = layout_next(ctx);
    let box_rect = rect(r.x, r.y, r.h, r.h);
    update_control(ctx, id, r, 0);
    let mut res = 0;
    if ctx.mouse_pressed == MU_MOUSE_LEFT && ctx.focus == id {
        res |= MU_RES_CHANGE;
        state.toggle();
    }
    draw_control_frame(ctx, id, box_rect, MU_COLOR_BASE, 0);
    if state.is_checked() {
        draw_icon(ctx, MU_ICON_CHECK, box_rect, ctx.style.colors[MU_COLOR_TEXT]);
    }
    r = rect(r.x + box_rect.w, r.y, r.w - box_rect.w, r.h);
    draw_control_text(ctx, label, r, MU_COLOR_TEXT, 0);
    res
}

pub fn textbox_raw(ctx: &mut Context, buf: &mut String, bufsz: usize, id: Id, r: Rect, opt: i32) -> i32 {
    let mut res = 0;
    update_control(ctx, id, r, opt | MU_OPT_HOLDFOCUS);
    if ctx.focus == id {
        let len = buf.len();
        let input = input_text_string(ctx);
        let n = min(bufsz.saturating_sub(len + 1), input.len());
        if n > 0 {
            buf.push_str(&input[..n]);
            res |= MU_RES_CHANGE;
        }
        if ctx.key_pressed & MU_KEY_BACKSPACE != 0 && !buf.is_empty() {
            let bytes = buf.as_bytes();
            let mut new_len = bytes.len() - 1;
            while new_len > 0 && (bytes[new_len] & 0xc0) == 0x80 {
                new_len -= 1;
            }
            buf.truncate(new_len);
            res |= MU_RES_CHANGE;
        }
        if ctx.key_pressed & MU_KEY_RETURN != 0 {
            set_focus(ctx, 0);
            res |= MU_RES_SUBMIT;
        }
    }
    draw_control_frame(ctx, id, r, MU_COLOR_BASE, opt);
    if ctx.focus == id {
        let color_ = ctx.style.colors[MU_COLOR_TEXT];
        let font = ctx.style.font;
        let textw = call_text_width(ctx, font, buf, -1);
        let texth = call_text_height(ctx, font);
        let ofx = r.w - ctx.style.padding - textw - 1;
        let textx = r.x + min(ofx, ctx.style.padding);
        let texty = r.y + (r.h - texth) / 2;
        push_clip_rect(ctx, r);
        draw_text(ctx, font, buf, -1, vec2(textx, texty), color_);
        draw_rect(ctx, rect(textx + textw, texty, 1, texth), color_);
        pop_clip_rect(ctx);
    } else {
        draw_control_text(ctx, buf, r, MU_COLOR_TEXT, opt);
    }
    res
}

pub fn textbox_ex(ctx: &mut Context, buf: &mut String, bufsz: usize, opt: i32) -> i32 {
    let ptr = buf as *const String as usize;
    let id = get_id(ctx, &ptr.to_ne_bytes());
    let r = layout_next(ctx);
    textbox_raw(ctx, buf, bufsz, id, r, opt)
}

pub fn textbox(ctx: &mut Context, buf: &mut String, bufsz: usize) -> i32 {
    textbox_ex(ctx, buf, bufsz, 0)
}

pub fn slider_ex(
    ctx: &mut Context,
    value: &mut Real,
    low: Real,
    high: Real,
    step: Real,
    fmt: &str,
    opt: i32,
) -> i32 {
    let ptr = value as *const Real as usize;
    let id = get_id(ctx, &ptr.to_ne_bytes());
    let base = layout_next(ctx);
    let mut last = *value;
    let mut v = last;
    if number_textbox(ctx, &mut v, base, id) {
        return 0;
    }
    update_control(ctx, id, base, opt);
    if ctx.focus == id && (ctx.mouse_down | ctx.mouse_pressed) == MU_MOUSE_LEFT {
        v = low + (ctx.mouse_pos.x - base.x) as Real * (high - low) / base.w as Real;
        if step != 0.0 {
            v = (((v + step / 2.0) / step) as i64 as Real) * step;
        }
    }
    *value = clamp_real(v, low, high);
    let mut res = 0;
    if last != *value {
        res |= MU_RES_CHANGE;
        last = *value;
    }
    let _ = last;
    draw_control_frame(ctx, id, base, MU_COLOR_BASE, opt);
    let w = ctx.style.thumb_size;
    let x = ((*value - low) * (base.w - w) as Real / (high - low)) as i32;
    let thumb = rect(base.x + x, base.y, w, base.h);
    draw_control_frame(ctx, id, thumb, MU_COLOR_BUTTON, opt);
    let buf = format_real(fmt, *value);
    draw_control_text(ctx, &buf, base, MU_COLOR_TEXT, opt);
    res
}

pub fn slider(ctx: &mut Context, value: &mut Real, low: Real, high: Real) -> i32 {
    slider_ex(ctx, value, low, high, 0.0, MU_SLIDER_FMT, MU_OPT_ALIGNCENTER)
}

pub fn number_ex(ctx: &mut Context, value: &mut Real, step: Real, fmt: &str, opt: i32) -> i32 {
    let ptr = value as *const Real as usize;
    let id = get_id(ctx, &ptr.to_ne_bytes());
    let base = layout_next(ctx);
    if number_textbox(ctx, value, base, id) {
        return 0;
    }
    let last = *value;
    update_control(ctx, id, base, opt);
    if ctx.focus == id && ctx.mouse_down == MU_MOUSE_LEFT {
        *value += ctx.mouse_delta.x as Real * step;
    }
    let mut res = 0;
    if *value != last {
        res |= MU_RES_CHANGE;
    }
    draw_control_frame(ctx, id, base, MU_COLOR_BASE, opt);
    let buf = format_real(fmt, *value);
    draw_control_text(ctx, &buf, base, MU_COLOR_TEXT, opt);
    res
}

pub fn number(ctx: &mut Context, value: &mut Real, step: Real) -> i32 {
    number_ex(ctx, value, step, MU_SLIDER_FMT, MU_OPT_ALIGNCENTER)
}

pub fn header_ex(ctx: &mut Context, label: &str, opt: i32) -> i32 {
    header_impl(ctx, label, false, opt)
}

pub fn header(ctx: &mut Context, label: &str) -> i32 {
    header_ex(ctx, label, 0)
}

pub fn begin_treenode_ex(ctx: &mut Context, label: &str, opt: i32) -> i32 {
    let res = header_impl(ctx, label, true, opt);
    if res & MU_RES_ACTIVE != 0 {
        get_layout_mut(ctx).indent += ctx.style.indent;
        ctx.id_stack.push(ctx.last_id);
    }
    res
}

pub fn begin_treenode(ctx: &mut Context, label: &str) -> i32 {
    begin_treenode_ex(ctx, label, 0)
}

pub fn end_treenode(ctx: &mut Context) {
    get_layout_mut(ctx).indent -= ctx.style.indent;
    pop_id(ctx);
}

pub fn begin_window_ex(ctx: &mut Context, title: &str, rect_: Rect, opt: i32) -> i32 {
    let id = get_id(ctx, title.as_bytes());
    let cnt_idx = match get_container_index_internal(ctx, id, opt) {
        Some(idx) => idx,
        None => return 0,
    };
    if !ctx.containers[cnt_idx].open {
        return 0;
    }
    ctx.id_stack.push(id);
    if ctx.containers[cnt_idx].rect.w == 0 {
        ctx.containers[cnt_idx].rect = rect_;
    }
    begin_root_container(ctx, cnt_idx);
    let rect_ = ctx.containers[cnt_idx].rect;
    let mut body = rect_;
    if opt & MU_OPT_NOFRAME == 0 {
        call_draw_frame(ctx, rect_, MU_COLOR_WINDOWBG as i32);
    }
    if opt & MU_OPT_NOTITLE == 0 {
        let mut tr = rect_;
        tr.h = ctx.style.title_height;
        call_draw_frame(ctx, tr, MU_COLOR_TITLEBG as i32);
        let title_id = get_id(ctx, b"!title");
        update_control(ctx, title_id, tr, opt);
        draw_control_text(ctx, title, tr, MU_COLOR_TITLETEXT, opt);
        if title_id == ctx.focus && ctx.mouse_down == MU_MOUSE_LEFT {
            ctx.containers[cnt_idx].rect.x += ctx.mouse_delta.x;
            ctx.containers[cnt_idx].rect.y += ctx.mouse_delta.y;
        }
        body.y += tr.h;
        body.h -= tr.h;
        if opt & MU_OPT_NOCLOSE == 0 {
            let close_id = get_id(ctx, b"!close");
            let r = rect(tr.x + tr.w - tr.h, tr.y, tr.h, tr.h);
            draw_icon(ctx, MU_ICON_CLOSE, r, ctx.style.colors[MU_COLOR_TITLETEXT]);
            update_control(ctx, close_id, r, opt);
            if ctx.mouse_pressed == MU_MOUSE_LEFT && close_id == ctx.focus {
                ctx.containers[cnt_idx].open = false;
            }
        }
    }
    push_container_body(ctx, cnt_idx, body, opt);
    if opt & MU_OPT_NORESIZE == 0 {
        let sz = ctx.style.title_height;
        let resize_id = get_id(ctx, b"!resize");
        let r = rect(rect_.x + rect_.w - sz, rect_.y + rect_.h - sz, sz, sz);
        update_control(ctx, resize_id, r, opt);
        if resize_id == ctx.focus && ctx.mouse_down == MU_MOUSE_LEFT {
            ctx.containers[cnt_idx].rect.w = max(96, ctx.containers[cnt_idx].rect.w + ctx.mouse_delta.x);
            ctx.containers[cnt_idx].rect.h = max(64, ctx.containers[cnt_idx].rect.h + ctx.mouse_delta.y);
        }
    }
    if opt & MU_OPT_AUTOSIZE != 0 {
        let r = get_layout(ctx).body;
        let content = ctx.containers[cnt_idx].content_size;
        ctx.containers[cnt_idx].rect.w = content.x + (ctx.containers[cnt_idx].rect.w - r.w);
        ctx.containers[cnt_idx].rect.h = content.y + (ctx.containers[cnt_idx].rect.h - r.h);
    }
    if opt & MU_OPT_POPUP != 0 && ctx.mouse_pressed != 0 && ctx.hover_root != Some(cnt_idx) {
        ctx.containers[cnt_idx].open = false;
    }
    let body = ctx.containers[cnt_idx].body;
    push_clip_rect(ctx, body);
    MU_RES_ACTIVE
}

pub fn begin_window(ctx: &mut Context, title: &str, rect_: Rect) -> i32 {
    begin_window_ex(ctx, title, rect_, 0)
}

pub fn end_window(ctx: &mut Context) {
    pop_clip_rect(ctx);
    end_root_container(ctx);
}

pub fn open_popup(ctx: &mut Context, name: &str) {
    let id = get_id(ctx, name.as_bytes());
    let cnt_idx = get_container_index_internal(ctx, id, 0).expect("popup container should exist");
    ctx.hover_root = Some(cnt_idx);
    ctx.next_hover_root = Some(cnt_idx);
    ctx.containers[cnt_idx].rect = rect(ctx.mouse_pos.x, ctx.mouse_pos.y, 1, 1);
    ctx.containers[cnt_idx].open = true;
    bring_to_front(ctx, cnt_idx);
}

pub fn begin_popup(ctx: &mut Context, name: &str) -> i32 {
    let opt = MU_OPT_POPUP | MU_OPT_AUTOSIZE | MU_OPT_NORESIZE | MU_OPT_NOSCROLL | MU_OPT_NOTITLE | MU_OPT_CLOSED;
    begin_window_ex(ctx, name, rect(0, 0, 0, 0), opt)
}

pub fn end_popup(ctx: &mut Context) {
    end_window(ctx);
}

pub fn begin_panel_ex(ctx: &mut Context, name: &str, opt: i32) {
    push_id(ctx, name.as_bytes());
    let idx = get_container_index_internal(ctx, ctx.last_id, opt).expect("panel container should exist");
    let next = layout_next(ctx);
    ctx.containers[idx].rect = next;
    if opt & MU_OPT_NOFRAME == 0 {
        call_draw_frame(ctx, ctx.containers[idx].rect, MU_COLOR_PANELBG as i32);
    }
    ctx.container_stack.push(idx);
    let rect_ = ctx.containers[idx].rect;
    push_container_body(ctx, idx, rect_, opt);
    let body = ctx.containers[idx].body;
    push_clip_rect(ctx, body);
}

pub fn begin_panel(ctx: &mut Context, name: &str) {
    begin_panel_ex(ctx, name, 0);
}

pub fn end_panel(ctx: &mut Context) {
    pop_clip_rect(ctx);
    pop_container(ctx);
}

pub fn mu_vec2(x: i32, y: i32) -> Vec2 {
    vec2(x, y)
}

pub fn mu_rect(x: i32, y: i32, w: i32, h: i32) -> Rect {
    rect(x, y, w, h)
}

pub fn mu_color(r: u8, g: u8, b: u8, a: u8) -> Color {
    color(r, g, b, a)
}

pub fn mu_init(ctx: &mut Context) {
    init(ctx)
}

pub fn mu_begin(ctx: &mut Context) {
    begin(ctx)
}

pub fn mu_end(ctx: &mut Context) {
    end(ctx)
}

pub fn mu_set_focus(ctx: &mut Context, id: Id) {
    set_focus(ctx, id)
}

pub fn mu_get_id(ctx: &mut Context, data: &[u8]) -> Id {
    get_id(ctx, data)
}

pub fn mu_push_id(ctx: &mut Context, data: &[u8]) {
    push_id(ctx, data)
}

pub fn mu_pop_id(ctx: &mut Context) {
    pop_id(ctx)
}

pub fn mu_push_clip_rect(ctx: &mut Context, r: Rect) {
    push_clip_rect(ctx, r)
}

pub fn mu_pop_clip_rect(ctx: &mut Context) {
    pop_clip_rect(ctx)
}

pub fn mu_get_clip_rect(ctx: &Context) -> Rect {
    get_clip_rect(ctx)
}

pub fn mu_check_clip(ctx: &Context, r: Rect) -> i32 {
    check_clip(ctx, r)
}

pub fn mu_get_current_container(ctx: &Context) -> &Container {
    get_current_container(ctx)
}

pub fn mu_get_container<'a>(ctx: &'a mut Context, name: &str) -> &'a mut Container {
    get_container(ctx, name)
}

pub fn mu_bring_to_front(ctx: &mut Context, idx: usize) {
    bring_to_front(ctx, idx)
}

pub fn mu_pool_init(ctx: &mut Context, items: &mut [PoolItem], id: Id) -> usize {
    pool_init(ctx, items, id)
}

pub fn mu_pool_get(ctx: &Context, items: &[PoolItem], id: Id) -> Option<usize> {
    pool_get(ctx, items, id)
}

pub fn mu_pool_update(ctx: &Context, items: &mut [PoolItem], idx: usize) {
    pool_update(ctx, items, idx)
}

pub fn mu_input_mousemove(ctx: &mut Context, x: i32, y: i32) {
    input_mousemove(ctx, x, y)
}

pub fn mu_input_mousedown(ctx: &mut Context, x: i32, y: i32, btn: i32) {
    input_mousedown(ctx, x, y, btn)
}

pub fn mu_input_mouseup(ctx: &mut Context, x: i32, y: i32, btn: i32) {
    input_mouseup(ctx, x, y, btn)
}

pub fn mu_input_scroll(ctx: &mut Context, x: i32, y: i32) {
    input_scroll(ctx, x, y)
}

pub fn mu_input_keydown(ctx: &mut Context, key: i32) {
    input_keydown(ctx, key)
}

pub fn mu_input_keyup(ctx: &mut Context, key: i32) {
    input_keyup(ctx, key)
}

pub fn mu_input_text(ctx: &mut Context, text: &str) {
    input_text(ctx, text)
}

pub fn mu_next_command<'a>(ctx: &'a Context, cmd: &mut Option<usize>) -> Option<Command<'a>> {
    next_command(ctx, cmd)
}

pub fn mu_push_command(ctx: &mut Context, type_: i32, size: usize) -> usize {
    push_command(ctx, type_, size)
}

pub fn mu_set_clip(ctx: &mut Context, r: Rect) {
    set_clip(ctx, r)
}

pub fn mu_draw_rect(ctx: &mut Context, r: Rect, c: Color) {
    draw_rect(ctx, r, c)
}

pub fn mu_draw_box(ctx: &mut Context, r: Rect, c: Color) {
    draw_box(ctx, r, c)
}

pub fn mu_draw_text(ctx: &mut Context, font: Font, text: &str, len: i32, pos: Vec2, c: Color) {
    draw_text(ctx, font, text, len, pos, c)
}

pub fn mu_draw_icon(ctx: &mut Context, id: i32, r: Rect, c: Color) {
    draw_icon(ctx, id, r, c)
}

pub fn mu_layout_row(ctx: &mut Context, items: i32, widths: Option<&[i32]>, height: i32) {
    layout_row(ctx, items, widths, height)
}

pub fn mu_layout_width(ctx: &mut Context, width: i32) {
    layout_width(ctx, width)
}

pub fn mu_layout_height(ctx: &mut Context, height: i32) {
    layout_height(ctx, height)
}

pub fn mu_layout_begin_column(ctx: &mut Context) {
    layout_begin_column(ctx)
}

pub fn mu_layout_end_column(ctx: &mut Context) {
    layout_end_column(ctx)
}

pub fn mu_layout_set_next(ctx: &mut Context, r: Rect, relative: bool) {
    layout_set_next(ctx, r, relative)
}

pub fn mu_layout_next(ctx: &mut Context) -> Rect {
    layout_next(ctx)
}

pub fn mu_draw_control_frame(ctx: &mut Context, id: Id, rect_: Rect, colorid: usize, opt: i32) {
    draw_control_frame(ctx, id, rect_, colorid, opt)
}

pub fn mu_draw_control_text(ctx: &mut Context, text: &str, rect_: Rect, colorid: usize, opt: i32) {
    draw_control_text(ctx, text, rect_, colorid, opt)
}

pub fn mu_mouse_over(ctx: &Context, rect_: Rect) -> bool {
    mouse_over(ctx, rect_)
}

pub fn mu_update_control(ctx: &mut Context, id: Id, rect_: Rect, opt: i32) {
    update_control(ctx, id, rect_, opt)
}

pub fn mu_text(ctx: &mut Context, text_: &str) {
    text(ctx, text_)
}

pub fn mu_label(ctx: &mut Context, text_: &str) {
    label(ctx, text_)
}

pub fn mu_button_ex(ctx: &mut Context, label: Option<&str>, icon: i32, opt: i32) -> i32 {
    button_ex(ctx, label, icon, opt)
}

pub fn mu_button(ctx: &mut Context, label: &str) -> i32 {
    button(ctx, label)
}

pub fn mu_checkbox<S: CheckboxState>(ctx: &mut Context, label: &str, state: &mut S) -> i32 {
    checkbox(ctx, label, state)
}

pub fn mu_textbox_raw(ctx: &mut Context, buf: &mut String, bufsz: usize, id: Id, r: Rect, opt: i32) -> i32 {
    textbox_raw(ctx, buf, bufsz, id, r, opt)
}

pub fn mu_textbox_ex(ctx: &mut Context, buf: &mut String, bufsz: usize, opt: i32) -> i32 {
    textbox_ex(ctx, buf, bufsz, opt)
}

pub fn mu_textbox(ctx: &mut Context, buf: &mut String, bufsz: usize) -> i32 {
    textbox(ctx, buf, bufsz)
}

pub fn mu_slider_ex(
    ctx: &mut Context,
    value: &mut Real,
    low: Real,
    high: Real,
    step: Real,
    fmt: &str,
    opt: i32,
) -> i32 {
    slider_ex(ctx, value, low, high, step, fmt, opt)
}

pub fn mu_slider(ctx: &mut Context, value: &mut Real, low: Real, high: Real) -> i32 {
    slider(ctx, value, low, high)
}

pub fn mu_number_ex(ctx: &mut Context, value: &mut Real, step: Real, fmt: &str, opt: i32) -> i32 {
    number_ex(ctx, value, step, fmt, opt)
}

pub fn mu_number(ctx: &mut Context, value: &mut Real, step: Real) -> i32 {
    number(ctx, value, step)
}

pub fn mu_header_ex(ctx: &mut Context, label: &str, opt: i32) -> i32 {
    header_ex(ctx, label, opt)
}

pub fn mu_header(ctx: &mut Context, label: &str) -> i32 {
    header(ctx, label)
}

pub fn mu_begin_treenode_ex(ctx: &mut Context, label: &str, opt: i32) -> i32 {
    begin_treenode_ex(ctx, label, opt)
}

pub fn mu_begin_treenode(ctx: &mut Context, label: &str) -> i32 {
    begin_treenode(ctx, label)
}

pub fn mu_end_treenode(ctx: &mut Context) {
    end_treenode(ctx)
}

pub fn mu_begin_window_ex(ctx: &mut Context, title: &str, rect_: Rect, opt: i32) -> i32 {
    begin_window_ex(ctx, title, rect_, opt)
}

pub fn mu_begin_window(ctx: &mut Context, title: &str, rect_: Rect) -> i32 {
    begin_window(ctx, title, rect_)
}

pub fn mu_end_window(ctx: &mut Context) {
    end_window(ctx)
}

pub fn mu_open_popup(ctx: &mut Context, name: &str) {
    open_popup(ctx, name)
}

pub fn mu_begin_popup(ctx: &mut Context, name: &str) -> i32 {
    begin_popup(ctx, name)
}

pub fn mu_end_popup(ctx: &mut Context) {
    end_popup(ctx)
}

pub fn mu_begin_panel_ex(ctx: &mut Context, name: &str, opt: i32) {
    begin_panel_ex(ctx, name, opt)
}

pub fn mu_begin_panel(ctx: &mut Context, name: &str) {
    begin_panel(ctx, name)
}

pub fn mu_end_panel(ctx: &mut Context) {
    end_panel(ctx)
}

fn default_style() -> Style {
    Style {
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

fn hash(hash: &mut Id, data: &[u8]) {
    for byte in data {
        *hash = (*hash ^ (*byte as Id)).wrapping_mul(16_777_619);
    }
}

fn expand_rect(r: Rect, n: i32) -> Rect {
    rect(r.x - n, r.y - n, r.w + n * 2, r.h + n * 2)
}

fn intersect_rects(r1: Rect, r2: Rect) -> Rect {
    let x1 = max(r1.x, r2.x);
    let y1 = max(r1.y, r2.y);
    let mut x2 = min(r1.x + r1.w, r2.x + r2.w);
    let mut y2 = min(r1.y + r1.h, r2.y + r2.h);
    if x2 < x1 {
        x2 = x1;
    }
    if y2 < y1 {
        y2 = y1;
    }
    rect(x1, y1, x2 - x1, y2 - y1)
}

fn rect_overlaps_vec2(r: Rect, p: Vec2) -> bool {
    p.x >= r.x && p.x < r.x + r.w && p.y >= r.y && p.y < r.y + r.h
}

fn default_draw_frame(ctx: &mut Context, rect_: Rect, colorid: i32) {
    draw_rect(ctx, rect_, ctx.style.colors[colorid as usize]);
    if colorid as usize == MU_COLOR_SCROLLBASE
        || colorid as usize == MU_COLOR_SCROLLTHUMB
        || colorid as usize == MU_COLOR_TITLEBG
    {
        return;
    }
    if ctx.style.colors[MU_COLOR_BORDER].a != 0 {
        draw_box(ctx, expand_rect(rect_, 1), ctx.style.colors[MU_COLOR_BORDER]);
    }
}

fn call_text_width(ctx: &Context, font: Font, text: &str, len: i32) -> i32 {
    (ctx.text_width.expect("text_width callback missing"))(font, text, len)
}

fn call_text_height(ctx: &Context, font: Font) -> i32 {
    (ctx.text_height.expect("text_height callback missing"))(font)
}

fn call_draw_frame(ctx: &mut Context, rect_: Rect, colorid: i32) {
    let draw = ctx.draw_frame.expect("draw_frame callback missing");
    draw(ctx, rect_, colorid);
}

fn input_text_string(ctx: &Context) -> String {
    c_buf_to_string(&ctx.input_text)
}

fn number_edit_string(ctx: &Context) -> String {
    c_buf_to_string(&ctx.number_edit_buf)
}

fn set_number_edit_string(ctx: &mut Context, text: &str) {
    write_c_buf(&mut ctx.number_edit_buf, text);
}

fn clamp_real(x: Real, lo: Real, hi: Real) -> Real {
    x.max(lo).min(hi)
}

fn parse_command<'a>(ctx: &'a Context, at: usize) -> Command<'a> {
    let base = BaseCommand {
        type_: read_i32(&ctx.command_list, at),
        size: read_size(&ctx.command_list, at),
    };
    match base.type_ {
        MU_COMMAND_JUMP => Command::Jump(JumpCommand {
            base,
            dst: read_usize(&ctx.command_list, at + 8),
        }),
        MU_COMMAND_CLIP => Command::Clip(ClipCommand {
            base,
            rect: read_rect(&ctx.command_list, at + 8),
        }),
        MU_COMMAND_RECT => Command::Rect(RectCommand {
            base,
            rect: read_rect(&ctx.command_list, at + 8),
            color: read_color(&ctx.command_list, at + 24),
        }),
        MU_COMMAND_TEXT => {
            let font = read_usize(&ctx.command_list, at + 8);
            let pos = read_vec2(&ctx.command_list, at + 8 + size_of::<usize>());
            let color_ = read_color(&ctx.command_list, at + 8 + size_of::<usize>() + 8);
            let start = at + TEXT_COMMAND_FIXED_SIZE;
            let end = at + base.size - 1;
            let text = std::str::from_utf8(&ctx.command_list[start..end]).unwrap_or("");
            Command::Text(TextCommand {
                base,
                font,
                pos,
                color: color_,
                text,
            })
        }
        MU_COMMAND_ICON => Command::Icon(IconCommand {
            base,
            rect: read_rect(&ctx.command_list, at + 8),
            id: read_i32(&ctx.command_list, at + 24),
            color: read_color(&ctx.command_list, at + 28),
        }),
        _ => panic!("unknown command type"),
    }
}

fn current_container_index(ctx: &Context) -> usize {
    assert!(ctx.container_stack.idx > 0, "container stack empty");
    ctx.container_stack.last()
}

fn get_layout(ctx: &Context) -> &Layout {
    assert!(ctx.layout_stack.idx > 0, "layout stack empty");
    &ctx.layout_stack.items[ctx.layout_stack.idx - 1]
}

fn get_layout_mut(ctx: &mut Context) -> &mut Layout {
    assert!(ctx.layout_stack.idx > 0, "layout stack empty");
    let idx = ctx.layout_stack.idx - 1;
    &mut ctx.layout_stack.items[idx]
}

fn push_layout(ctx: &mut Context, body: Rect, scroll: Vec2) {
    let layout = Layout {
        body: rect(body.x - scroll.x, body.y - scroll.y, body.w, body.h),
        max: vec2(-0x1000000, -0x1000000),
        ..Layout::default()
    };
    ctx.layout_stack.push(layout);
    layout_row(ctx, 1, Some(&[0]), 0);
}

fn pop_container(ctx: &mut Context) {
    let cnt_idx = current_container_index(ctx);
    let layout = *get_layout(ctx);
    ctx.containers[cnt_idx].content_size.x = layout.max.x - layout.body.x;
    ctx.containers[cnt_idx].content_size.y = layout.max.y - layout.body.y;
    ctx.container_stack.pop();
    ctx.layout_stack.pop();
    pop_id(ctx);
}

fn get_container_index_internal(ctx: &mut Context, id: Id, opt: i32) -> Option<usize> {
    if let Some(idx) = pool_get(ctx, &ctx.container_pool, id) {
        if ctx.containers[idx].open || opt & MU_OPT_CLOSED == 0 {
            let frame = ctx.frame;
            ctx.container_pool[idx].last_update = frame;
        }
        return Some(idx);
    }
    if opt & MU_OPT_CLOSED != 0 {
        return None;
    }
    let idx = {
        let mut f = ctx.frame;
        let mut n = None;
        for (i, item) in ctx.container_pool.iter().enumerate() {
            if item.last_update < f {
                f = item.last_update;
                n = Some(i);
            }
        }
        let idx = n.expect("container pool full");
        ctx.container_pool[idx].id = id;
        ctx.container_pool[idx].last_update = ctx.frame;
        idx
    };
    ctx.containers[idx] = Container::default();
    ctx.containers[idx].open = true;
    bring_to_front(ctx, idx);
    Some(idx)
}

fn write_i32(buf: &mut [u8], at: usize, value: i32) {
    buf[at..at + 4].copy_from_slice(&value.to_ne_bytes());
}

fn read_i32(buf: &[u8], at: usize) -> i32 {
    i32::from_ne_bytes(buf[at..at + 4].try_into().expect("i32 bytes"))
}

fn write_usize(buf: &mut [u8], at: usize, value: usize) {
    buf[at..at + size_of::<usize>()].copy_from_slice(&value.to_ne_bytes());
}

fn read_usize(buf: &[u8], at: usize) -> usize {
    usize::from_ne_bytes(buf[at..at + size_of::<usize>()].try_into().expect("usize bytes"))
}

fn write_vec2(buf: &mut [u8], at: usize, value: Vec2) {
    write_i32(buf, at, value.x);
    write_i32(buf, at + 4, value.y);
}

fn read_vec2(buf: &[u8], at: usize) -> Vec2 {
    vec2(read_i32(buf, at), read_i32(buf, at + 4))
}

fn write_rect(buf: &mut [u8], at: usize, value: Rect) {
    write_i32(buf, at, value.x);
    write_i32(buf, at + 4, value.y);
    write_i32(buf, at + 8, value.w);
    write_i32(buf, at + 12, value.h);
}

fn read_rect(buf: &[u8], at: usize) -> Rect {
    rect(
        read_i32(buf, at),
        read_i32(buf, at + 4),
        read_i32(buf, at + 8),
        read_i32(buf, at + 12),
    )
}

fn write_color(buf: &mut [u8], at: usize, value: Color) {
    buf[at] = value.r;
    buf[at + 1] = value.g;
    buf[at + 2] = value.b;
    buf[at + 3] = value.a;
}

fn read_color(buf: &[u8], at: usize) -> Color {
    color(buf[at], buf[at + 1], buf[at + 2], buf[at + 3])
}

fn read_size(buf: &[u8], at: usize) -> usize {
    read_i32(buf, at + 4) as usize
}

fn write_jump_dst(ctx: &mut Context, at: usize, dst: usize) {
    write_usize(&mut ctx.command_list, at + 8, dst);
}

fn push_jump(ctx: &mut Context, dst: usize) -> usize {
    let at = push_command(ctx, MU_COMMAND_JUMP, JUMP_COMMAND_SIZE);
    write_usize(&mut ctx.command_list, at + 8, dst);
    at
}

fn in_hover_root(ctx: &Context) -> bool {
    let mut i = ctx.container_stack.idx;
    while i > 0 {
        i -= 1;
        let idx = ctx.container_stack.items[i];
        if Some(idx) == ctx.hover_root {
            return true;
        }
        if ctx.containers[idx].head.is_some() {
            break;
        }
    }
    false
}

fn header_impl(ctx: &mut Context, label: &str, is_treenode: bool, opt: i32) -> i32 {
    let id = get_id(ctx, label.as_bytes());
    let idx = pool_get(ctx, &ctx.treenode_pool, id);
    let width = -1;
    layout_row(ctx, 1, Some(&[width]), 0);
    let mut active = idx.is_some();
    let expanded = if opt & MU_OPT_EXPANDED != 0 { !active } else { active };
    let mut r = layout_next(ctx);
    update_control(ctx, id, r, 0);
    active ^= ctx.mouse_pressed == MU_MOUSE_LEFT && ctx.focus == id;
    if let Some(idx) = idx {
        if active {
            ctx.treenode_pool[idx].last_update = ctx.frame;
        } else {
            ctx.treenode_pool[idx] = PoolItem::default();
        }
    } else if active {
        let mut f = ctx.frame;
        let mut n = None;
        for (i, item) in ctx.treenode_pool.iter().enumerate() {
            if item.last_update < f {
                f = item.last_update;
                n = Some(i);
            }
        }
        let i = n.expect("treenode pool full");
        ctx.treenode_pool[i].id = id;
        ctx.treenode_pool[i].last_update = ctx.frame;
    }
    if is_treenode {
        if ctx.hover == id {
            call_draw_frame(ctx, r, MU_COLOR_BUTTONHOVER as i32);
        }
    } else {
        draw_control_frame(ctx, id, r, MU_COLOR_BUTTON, 0);
    }
    draw_icon(
        ctx,
        if expanded { MU_ICON_EXPANDED } else { MU_ICON_COLLAPSED },
        rect(r.x, r.y, r.h, r.h),
        ctx.style.colors[MU_COLOR_TEXT],
    );
    r.x += r.h - ctx.style.padding;
    r.w -= r.h - ctx.style.padding;
    draw_control_text(ctx, label, r, MU_COLOR_TEXT, 0);
    if expanded { MU_RES_ACTIVE } else { 0 }
}

fn scrollbars(ctx: &mut Context, cnt_idx: usize, body: &mut Rect) {
    let sz = ctx.style.scrollbar_size;
    let mut cs = ctx.containers[cnt_idx].content_size;
    cs.x += ctx.style.padding * 2;
    cs.y += ctx.style.padding * 2;
    push_clip_rect(ctx, *body);
    if cs.y > ctx.containers[cnt_idx].body.h {
        body.w -= sz;
    }
    if cs.x > ctx.containers[cnt_idx].body.w {
        body.h -= sz;
    }
    scrollbar_y(ctx, cnt_idx, body, cs);
    scrollbar_x(ctx, cnt_idx, body, cs);
    pop_clip_rect(ctx);
}

fn scrollbar_y(ctx: &mut Context, cnt_idx: usize, body: &Rect, cs: Vec2) {
    let maxscroll = cs.y - body.h;
    if maxscroll > 0 && body.h > 0 {
        let id = get_id(ctx, b"!scrollbary");
        let mut base = *body;
        base.x = body.x + body.w;
        base.w = ctx.style.scrollbar_size;
        update_control(ctx, id, base, 0);
        if ctx.focus == id && ctx.mouse_down == MU_MOUSE_LEFT {
            ctx.containers[cnt_idx].scroll.y += ctx.mouse_delta.y * cs.y / base.h;
        }
        ctx.containers[cnt_idx].scroll.y = ctx.containers[cnt_idx].scroll.y.clamp(0, maxscroll);
        call_draw_frame(ctx, base, MU_COLOR_SCROLLBASE as i32);
        let mut thumb = base;
        thumb.h = max(ctx.style.thumb_size, base.h * body.h / cs.y);
        thumb.y += ctx.containers[cnt_idx].scroll.y * (base.h - thumb.h) / maxscroll;
        call_draw_frame(ctx, thumb, MU_COLOR_SCROLLTHUMB as i32);
        if mouse_over(ctx, *body) {
            ctx.scroll_target = Some(cnt_idx);
        }
    } else {
        ctx.containers[cnt_idx].scroll.y = 0;
    }
}

fn scrollbar_x(ctx: &mut Context, cnt_idx: usize, body: &Rect, cs: Vec2) {
    let maxscroll = cs.x - body.w;
    if maxscroll > 0 && body.w > 0 {
        let id = get_id(ctx, b"!scrollbarx");
        let mut base = *body;
        base.y = body.y + body.h;
        base.h = ctx.style.scrollbar_size;
        update_control(ctx, id, base, 0);
        if ctx.focus == id && ctx.mouse_down == MU_MOUSE_LEFT {
            ctx.containers[cnt_idx].scroll.x += ctx.mouse_delta.x * cs.x / base.w;
        }
        ctx.containers[cnt_idx].scroll.x = ctx.containers[cnt_idx].scroll.x.clamp(0, maxscroll);
        call_draw_frame(ctx, base, MU_COLOR_SCROLLBASE as i32);
        let mut thumb = base;
        thumb.w = max(ctx.style.thumb_size, base.w * body.w / cs.x);
        thumb.x += ctx.containers[cnt_idx].scroll.x * (base.w - thumb.w) / maxscroll;
        call_draw_frame(ctx, thumb, MU_COLOR_SCROLLTHUMB as i32);
        if mouse_over(ctx, *body) {
            ctx.scroll_target = Some(cnt_idx);
        }
    } else {
        ctx.containers[cnt_idx].scroll.x = 0;
    }
}

fn push_container_body(ctx: &mut Context, cnt_idx: usize, mut body: Rect, opt: i32) {
    if opt & MU_OPT_NOSCROLL == 0 {
        scrollbars(ctx, cnt_idx, &mut body);
    }
    let scroll = ctx.containers[cnt_idx].scroll;
    push_layout(ctx, expand_rect(body, -ctx.style.padding), scroll);
    ctx.containers[cnt_idx].body = body;
}

fn begin_root_container(ctx: &mut Context, cnt_idx: usize) {
    ctx.container_stack.push(cnt_idx);
    ctx.root_list.push(cnt_idx);
    let head = push_jump(ctx, 0);
    ctx.containers[cnt_idx].head = Some(head);
    if rect_overlaps_vec2(ctx.containers[cnt_idx].rect, ctx.mouse_pos)
        && ctx
            .next_hover_root
            .map(|idx| ctx.containers[cnt_idx].zindex > ctx.containers[idx].zindex)
            .unwrap_or(true)
    {
        ctx.next_hover_root = Some(cnt_idx);
    }
    ctx.clip_stack.push(UNCLIPPED_RECT);
}

fn end_root_container(ctx: &mut Context) {
    let cnt_idx = current_container_index(ctx);
    let tail = push_jump(ctx, 0);
    ctx.containers[cnt_idx].tail = Some(tail);
    let head = ctx.containers[cnt_idx].head.expect("root head missing");
    write_jump_dst(ctx, head, ctx.command_list_idx);
    pop_clip_rect(ctx);
    pop_container(ctx);
}

fn number_textbox(ctx: &mut Context, value: &mut Real, r: Rect, id: Id) -> bool {
    if ctx.mouse_pressed == MU_MOUSE_LEFT && ctx.key_down & MU_KEY_SHIFT != 0 && ctx.hover == id {
        ctx.number_edit = id;
        let text = format_real(MU_REAL_FMT, *value);
        set_number_edit_string(ctx, &text);
    }
    if ctx.number_edit == id {
        let mut edit = number_edit_string(ctx);
        let res = textbox_raw(ctx, &mut edit, MU_MAX_FMT, id, r, 0);
        set_number_edit_string(ctx, &edit);
        if res & MU_RES_SUBMIT != 0 || ctx.focus != id {
            *value = parse_real(&number_edit_string(ctx));
            ctx.number_edit = 0;
        } else {
            return true;
        }
    }
    false
}

fn c_buf_len<const N: usize>(buf: &[u8; N]) -> usize {
    buf.iter().position(|b| *b == 0).unwrap_or(N)
}

fn c_buf_to_string<const N: usize>(buf: &[u8; N]) -> String {
    let len = c_buf_len(buf);
    String::from_utf8_lossy(&buf[..len]).into_owned()
}

fn write_c_buf<const N: usize>(buf: &mut [u8; N], text: &str) {
    buf.fill(0);
    let bytes = text.as_bytes();
    let len = min(bytes.len(), N.saturating_sub(1));
    buf[..len].copy_from_slice(&bytes[..len]);
}

fn format_real(fmt: &str, value: Real) -> String {
    let cfmt = CString::new(fmt).expect("format string");
    let mut out = [0u8; MU_MAX_FMT + 1];
    // SAFETY: `out` is a valid writable buffer, `cfmt` is NUL-terminated, and the
    // varargs call matches `%f`/`%g`-style formats used by microui.
    unsafe {
        snprintf(
            out.as_mut_ptr() as *mut c_char,
            out.len(),
            cfmt.as_ptr(),
            value as c_double,
        );
        CStr::from_ptr(out.as_ptr() as *const c_char)
            .to_string_lossy()
            .into_owned()
    }
}

fn parse_real(text: &str) -> Real {
    let ctext = CString::new(text).expect("number string");
    // SAFETY: `ctext` is NUL-terminated and lives for the duration of the call.
    unsafe { strtod(ctext.as_ptr(), std::ptr::null_mut()) as Real }
}
