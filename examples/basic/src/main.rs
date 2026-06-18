use std::num::NonZeroU32;
use std::sync::Arc;

use basic::Renderer;
use microui::*;
use softbuffer::{Context, Surface};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowId},
};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

struct App {
    state: Option<(Arc<Window>, Context<Arc<Window>>, Surface<Arc<Window>, Arc<Window>>)>,
    ctx: microui::Context,
    renderer: Renderer,
}

impl App {
    fn new() -> Self {
        Self {
            state: None,
            ctx: basic::make_ctx(),
            renderer: Renderer::new(WIDTH, HEIGHT),
        }
    }

    fn process_frame(&mut self) {
        let ctx = &mut self.ctx;
        mu_begin(ctx);

        if mu_begin_window(ctx, "My Window", mu_rect(10, 10, 140, 86)) != 0 {
            mu_layout_row(ctx, 2, Some(&[60, -1]), 0);

            mu_label(ctx, "First:");
            if mu_button(ctx, "Button1") != 0 {
                println!("Button1 pressed");
            }

            mu_label(ctx, "Second:");
            if mu_button(ctx, "Button2") != 0 {
                mu_open_popup(ctx, "My Popup");
            }

            if mu_begin_popup(ctx, "My Popup") != 0 {
                mu_label(ctx, "Hello world!");
                mu_end_popup(ctx);
            }

            mu_end_window(ctx);
        }

        mu_end(ctx);
    }

    fn scale(&self) -> u32 {
        self.state.as_ref().map_or(1, |(w, _, _)| w.scale_factor().round() as u32)
    }

    fn draw(&mut self) {
        let scale = self.scale();
        let Some((_, _, surface)) = self.state.as_mut() else { return };

        self.renderer.clear(40, 40, 40);
        self.renderer.render(&self.ctx);

        let phys_w = WIDTH * scale;
        let phys_h = HEIGHT * scale;

        surface
            .resize(NonZeroU32::new(phys_w).unwrap(), NonZeroU32::new(phys_h).unwrap())
            .expect("resize failed");

        let mut buf = surface.buffer_mut().expect("buffer_mut failed");
        let src = self.renderer.pixels();
        for py in 0..phys_h {
            let ly = (py / scale) as usize;
            for px in 0..phys_w {
                let s = (ly * WIDTH as usize + (px / scale) as usize) * 4;
                buf[(py * phys_w + px) as usize] =
                    ((src[s] as u32) << 16) | ((src[s + 1] as u32) << 8) | src[s + 2] as u32;
            }
        }
        buf.present().expect("present failed");
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("microui basic")
                        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT)),
                )
                .expect("window creation failed"),
        );
        let context = Context::new(Arc::clone(&window)).expect("softbuffer context failed");
        let surface =
            Surface::new(&context, Arc::clone(&window)).expect("softbuffer surface failed");
        self.state = Some((window, context, surface));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::RedrawRequested => {
                self.process_frame();
                self.draw();
            }

            WindowEvent::CursorMoved { position, .. } => {
                let s = self.scale() as f64;
                mu_input_mousemove(&mut self.ctx, (position.x / s) as i32, (position.y / s) as i32);
                self.request_redraw();
            }

            WindowEvent::MouseInput { state, button, .. } => {
                let btn = match button {
                    MouseButton::Left => MU_MOUSE_LEFT,
                    MouseButton::Right => MU_MOUSE_RIGHT,
                    MouseButton::Middle => MU_MOUSE_MIDDLE,
                    _ => 0,
                };
                if btn != 0 {
                    let (x, y) = (self.ctx.mouse_pos.x, self.ctx.mouse_pos.y);
                    match state {
                        ElementState::Pressed  => mu_input_mousedown(&mut self.ctx, x, y, btn),
                        ElementState::Released => mu_input_mouseup(&mut self.ctx, x, y, btn),
                    }
                    self.request_redraw();
                }
            }

            WindowEvent::MouseWheel { delta, .. } => {
                use winit::event::MouseScrollDelta;
                let (dx, dy) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (x as i32 * 30, y as i32 * -30),
                    MouseScrollDelta::PixelDelta(p) => (p.x as i32, -p.y as i32),
                };
                mu_input_scroll(&mut self.ctx, dx, dy);
                self.request_redraw();
            }

            WindowEvent::KeyboardInput { event, .. } => {
                let key = match &event.logical_key {
                    Key::Named(NamedKey::Shift) => MU_KEY_SHIFT,
                    Key::Named(NamedKey::Control) => MU_KEY_CTRL,
                    Key::Named(NamedKey::Alt) => MU_KEY_ALT,
                    Key::Named(NamedKey::Backspace) => MU_KEY_BACKSPACE,
                    Key::Named(NamedKey::Enter) => MU_KEY_RETURN,
                    _ => 0,
                };
                if event.state.is_pressed() {
                    if key != 0 { mu_input_keydown(&mut self.ctx, key); }
                    if let Key::Character(s) = &event.logical_key { mu_input_text(&mut self.ctx, s); }
                } else if key != 0 {
                    mu_input_keyup(&mut self.ctx, key);
                }
                self.request_redraw();
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        self.request_redraw();
    }
}

impl App {
    fn request_redraw(&self) {
        if let Some((window, _, _)) = &self.state {
            window.request_redraw();
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("event loop failed");
    event_loop.run_app(&mut App::new()).expect("run failed");
}
