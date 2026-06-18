use font8x8::legacy::BASIC_LEGACY;
use microui::{
    mu_next_command, mu_rect, Color, Command, Context, Font, Rect, Vec2,
    MU_ICON_CHECK, MU_ICON_CLOSE, MU_ICON_COLLAPSED, MU_ICON_EXPANDED,
};

pub fn text_width(_font: Font, text: &str, len: i32) -> i32 {
    let n = if len < 0 { text.len() } else { (len as usize).min(text.len()) };
    (n * 8) as i32
}

pub fn text_height(_font: Font) -> i32 {
    8
}

pub fn make_ctx() -> Context {
    let mut ctx = Context::new();
    ctx.text_width = Some(text_width);
    ctx.text_height = Some(text_height);
    ctx
}

pub struct Renderer {
    pub width: u32,
    pub height: u32,
    buf: Vec<u8>,
    clip: Rect,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buf: vec![0u8; (width * height * 4) as usize],
            clip: mu_rect(0, 0, width as i32, height as i32),
        }
    }

    pub fn pixels(&self) -> &[u8] {
        &self.buf
    }

    pub fn clear(&mut self, r: u8, g: u8, b: u8) {
        for px in self.buf.chunks_mut(4) {
            px[0] = r;
            px[1] = g;
            px[2] = b;
            px[3] = 255;
        }
    }

    pub fn render(&mut self, ctx: &Context) {
        let mut cursor = None;
        while let Some(cmd) = mu_next_command(ctx, &mut cursor) {
            match cmd {
                Command::Clip(c) => self.clip = c.rect,
                Command::Rect(c) => self.fill_rect(c.rect, c.color),
                Command::Text(c) => self.draw_text(c.text, c.pos, c.color),
                Command::Icon(c) => self.draw_icon(c.id, c.rect, c.color),
                Command::Jump(_) => {}
            }
        }
    }

    fn put(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8) {
        if x < self.clip.x
            || x >= self.clip.x + self.clip.w
            || y < self.clip.y
            || y >= self.clip.y + self.clip.h
            || x >= self.width as i32
            || y >= self.height as i32
        {
            return;
        }
        let i = ((y as u32 * self.width + x as u32) * 4) as usize;
        self.buf[i] = r;
        self.buf[i + 1] = g;
        self.buf[i + 2] = b;
        self.buf[i + 3] = 255;
    }

    fn fill_rect(&mut self, rect: Rect, c: Color) {
        if c.a == 0 {
            return;
        }
        let x0 = rect.x.max(self.clip.x).max(0);
        let y0 = rect.y.max(self.clip.y).max(0);
        let x1 = (rect.x + rect.w)
            .min(self.clip.x + self.clip.w)
            .min(self.width as i32);
        let y1 = (rect.y + rect.h)
            .min(self.clip.y + self.clip.h)
            .min(self.height as i32);
        for y in y0..y1 {
            for x in x0..x1 {
                let i = ((y as u32 * self.width + x as u32) * 4) as usize;
                if c.a == 255 {
                    self.buf[i] = c.r;
                    self.buf[i + 1] = c.g;
                    self.buf[i + 2] = c.b;
                    self.buf[i + 3] = 255;
                } else {
                    let a = c.a as u32;
                    let ia = 255 - a;
                    self.buf[i] = ((self.buf[i] as u32 * ia + c.r as u32 * a) / 255) as u8;
                    self.buf[i + 1] =
                        ((self.buf[i + 1] as u32 * ia + c.g as u32 * a) / 255) as u8;
                    self.buf[i + 2] =
                        ((self.buf[i + 2] as u32 * ia + c.b as u32 * a) / 255) as u8;
                    self.buf[i + 3] = 255;
                }
            }
        }
    }

    fn draw_text(&mut self, text: &str, pos: Vec2, c: Color) {
        let mut cx = pos.x;
        for ch in text.chars() {
            let glyph = BASIC_LEGACY[(ch as usize).min(127)];
            for row in 0..8i32 {
                let bits = glyph[row as usize];
                for col in 0..8i32 {
                    if bits & (1 << col) != 0 {
                        self.put(cx + col, pos.y + row, c.r, c.g, c.b);
                    }
                }
            }
            cx += 8;
        }
    }

    fn draw_icon(&mut self, id: i32, rect: Rect, c: Color) {
        let cx = rect.x + rect.w / 2;
        let cy = rect.y + rect.h / 2;
        match id {
            MU_ICON_CLOSE => {
                for i in -3i32..=3 {
                    self.put(cx + i, cy + i, c.r, c.g, c.b);
                    self.put(cx - i, cy + i, c.r, c.g, c.b);
                }
            }
            MU_ICON_CHECK => {
                for i in 0..3i32 {
                    self.put(cx - 3 + i, cy + i, c.r, c.g, c.b);
                }
                for i in 0..5i32 {
                    self.put(cx - 1 + i, cy + 3 - i, c.r, c.g, c.b);
                }
            }
            MU_ICON_COLLAPSED => {
                for i in 0..5i32 {
                    let h = 2 - (i - 2).abs();
                    for j in 0..=h {
                        self.put(cx - 2 + j, cy - 2 + i, c.r, c.g, c.b);
                    }
                }
            }
            MU_ICON_EXPANDED => {
                for i in 0..5i32 {
                    let h = 2 - (i - 2).abs();
                    for j in -h..=h {
                        self.put(cx + j, cy - 2 + i, c.r, c.g, c.b);
                    }
                }
            }
            _ => {}
        }
    }
}
