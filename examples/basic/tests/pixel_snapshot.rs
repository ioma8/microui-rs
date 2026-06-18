use basic::{make_ctx, Renderer};
use microui::*;

/// Render one deterministic frame of the basic-usage example — no user input,
/// no windowing system — and return the raw RGBA pixel bytes.
fn render_frame() -> Vec<u8> {
    let mut ctx = make_ctx();

    mu_begin(&mut ctx);

    if mu_begin_window(&mut ctx, "My Window", mu_rect(10, 10, 140, 86)) != 0 {
        mu_layout_row(&mut ctx, 2, Some(&[60, -1]), 0);

        mu_label(&mut ctx, "First:");
        mu_button(&mut ctx, "Button1");

        mu_label(&mut ctx, "Second:");
        mu_button(&mut ctx, "Button2");

        // Popup is closed by default (no mouse input); begin_popup returns 0.
        if mu_begin_popup(&mut ctx, "My Popup") != 0 {
            mu_label(&mut ctx, "Hello world!");
            mu_end_popup(&mut ctx);
        }

        mu_end_window(&mut ctx);
    }

    mu_end(&mut ctx);

    let mut r = Renderer::new(200, 120);
    r.clear(40, 40, 40);
    r.render(&ctx);
    r.pixels().to_vec()
}

/// Compare rendered pixels against a stored golden file.
///
/// To regenerate the golden (after an intentional visual change):
///   BLESS=1 cargo test -p microui-basic
#[test]
fn pixel_output_is_stable() {
    let pixels = render_frame();

    let golden = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/snapshot.raw");

    if std::env::var("BLESS").is_ok() || !golden.exists() {
        std::fs::create_dir_all(golden.parent().unwrap()).unwrap();
        std::fs::write(&golden, &pixels).unwrap();
        println!("golden written → {}", golden.display());
        // If we just generated it, the test passes trivially.
        return;
    }

    let expected = std::fs::read(&golden).expect("golden missing; run BLESS=1 cargo test");
    assert_eq!(pixels.len(), expected.len(), "canvas size changed");
    assert_eq!(pixels, expected, "pixel output changed — run BLESS=1 cargo test to update golden");
}
