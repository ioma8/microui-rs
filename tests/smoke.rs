use microui::{begin, begin_window, end, end_window, label, layout_row, next_command, rect, Command, Context};

fn text_width(_font: microui::Font, text: &str, len: i32) -> i32 {
    if len < 0 {
        text.len() as i32
    } else {
        len
    }
}

fn text_height(_font: microui::Font) -> i32 {
    10
}

#[test]
fn simple_window_command_stream_matches_fixture() {
    let mut ctx = Context::new();
    ctx.text_width = Some(text_width);
    ctx.text_height = Some(text_height);

    begin(&mut ctx);
    if begin_window(&mut ctx, "Test", rect(0, 0, 100, 100)) != 0 {
        layout_row(&mut ctx, 1, Some(&[-1]), 0);
        label(&mut ctx, "Hello");
        end_window(&mut ctx);
    }
    end(&mut ctx);

    let mut cursor = None;
    let mut actual = String::new();
    while let Some(cmd) = next_command(&ctx, &mut cursor) {
        match cmd {
            Command::Clip(c) => {
                actual.push_str(&format!("clip {} {} {} {}\n", c.rect.x, c.rect.y, c.rect.w, c.rect.h));
            }
            Command::Rect(c) => {
                actual.push_str(&format!(
                    "rect {} {} {} {} {} {} {} {}\n",
                    c.rect.x, c.rect.y, c.rect.w, c.rect.h, c.color.r, c.color.g, c.color.b, c.color.a
                ));
            }
            Command::Text(c) => {
                actual.push_str(&format!("text {} {} {}\n", c.pos.x, c.pos.y, c.text));
            }
            Command::Icon(c) => {
                actual.push_str(&format!("icon {} {} {} {} {}\n", c.id, c.rect.x, c.rect.y, c.rect.w, c.rect.h));
            }
            Command::Jump(_) => {}
        }
    }

    let expected = std::fs::read_to_string("tests/fixtures/smoke.txt").unwrap();
    assert_eq!(actual, expected);
}
