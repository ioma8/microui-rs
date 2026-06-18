# microui Rust Port Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Rust-only `microui` crate that preserves the full C library behavior from `src/microui.c` and `src/microui.h`, including undocumented edge cases, while exposing the surface with idiomatic Rust types and constants.

**Architecture:** Keep the original C sources in `src/` as the behavioral reference while creating a Rust crate in parallel under `Cargo.toml`, `src/lib.rs`, and focused Rust modules. Use red-green TDD with generated oracle fixtures from small C helpers, then port the engine subsystem by subsystem in the same order as the C file: primitives and storage first, then context and command handling, then layout, then shared control logic, then widgets and containers.

**Tech Stack:** Rust stable, Cargo, standard library, system C compiler for oracle fixture generation, original `src/microui.c` and `src/microui.h`

---

## File Structure

**Existing files to keep as reference**
- `src/microui.c`
- `src/microui.h`
- `README.md`
- `doc/usage.md`

**Files to create**
- `Cargo.toml`
- `src/lib.rs`
- `src/constants.rs`
- `src/types.rs`
- `src/storage.rs`
- `src/context.rs`
- `src/command.rs`
- `src/layout.rs`
- `src/pool.rs`
- `src/input.rs`
- `src/controls.rs`
- `src/widgets.rs`
- `tests/common/mod.rs`
- `tests/primitives.rs`
- `tests/storage_and_pool.rs`
- `tests/commands.rs`
- `tests/layout.rs`
- `tests/controls_basic.rs`
- `tests/controls_edit.rs`
- `tests/containers.rs`
- `tests/integration_frames.rs`
- `tests/fixtures/primitives.txt`
- `tests/fixtures/storage_and_pool.txt`
- `tests/fixtures/commands.txt`
- `tests/fixtures/layout.txt`
- `tests/fixtures/controls_basic.txt`
- `tests/fixtures/controls_edit.txt`
- `tests/fixtures/containers.txt`
- `tests/fixtures/integration_frames.txt`
- `tools/oracle/primitives.c`
- `tools/oracle/storage_and_pool.c`
- `tools/oracle/commands.c`
- `tools/oracle/layout.c`
- `tools/oracle/controls_basic.c`
- `tools/oracle/controls_edit.c`
- `tools/oracle/containers.c`
- `tools/oracle/integration_frames.c`
- `docs/porting-notes.md`

**Responsibility split**
- `src/constants.rs`: version, sizes, command/color/icon/result/option/key/mouse constants
- `src/types.rs`: `Vec2`, `Rect`, `Color`, `PoolItem`, `Layout`, `Container`, `Style`, `Command` structs and helpers
- `src/storage.rs`: fixed-capacity stack and byte-buffer helpers used internally
- `src/context.rs`: `Context`, default style, frame lifecycle, ID hashing, clip/container/layout/id stacks
- `src/command.rs`: packed command push and command iteration
- `src/layout.rs`: row and column progression and `layout_next`
- `src/pool.rs`: retained pool lookup, init, update
- `src/input.rs`: mouse, key, scroll, and text input accumulation
- `src/controls.rs`: shared helpers such as clip checks, hover/focus, draw control text/frame
- `src/widgets.rs`: text, label, button, checkbox, textbox, slider, number, tree, panel, popup, and window logic
- `tests/common/mod.rs`: fixture loading, callback shims, scenario helpers
- `tools/oracle/*.c`: one small C program per subsystem to print deterministic fixture output for parity tests
- `docs/porting-notes.md`: short notes for any intentionally awkward parity-preserving choices

### Task 1: Scaffold The Rust Crate And Baseline Test Harness

**Files:**
- Create: `Cargo.toml`
- Create: `src/lib.rs`
- Create: `tests/common/mod.rs`
- Create: `tests/primitives.rs`
- Create: `docs/porting-notes.md`

- [ ] **Step 1: Write the failing baseline test**

```rust
// tests/primitives.rs
use microui::{color, rect, vec2, Color, Rect, Vec2, MU_VERSION};

#[test]
fn constructors_and_version_exist() {
    assert_eq!(MU_VERSION, "2.02");
    assert_eq!(vec2(1, 2), Vec2 { x: 1, y: 2 });
    assert_eq!(rect(3, 4, 5, 6), Rect { x: 3, y: 4, w: 5, h: 6 });
    assert_eq!(color(7, 8, 9, 10), Color { r: 7, g: 8, b: 9, a: 10 });
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test constructors_and_version_exist --test primitives -v`
Expected: FAIL with an error like `could not find Cargo.toml`

- [ ] **Step 3: Write minimal crate scaffolding**

```toml
# Cargo.toml
[package]
name = "microui"
version = "0.1.0"
edition = "2024"

[lib]
name = "microui"
path = "src/lib.rs"
```

```rust
// src/lib.rs
pub const MU_VERSION: &str = "2.02";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
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
```

```rust
// tests/common/mod.rs
pub fn fixture(path: &str) -> String {
    std::fs::read_to_string(path).expect("fixture should exist")
}
```

```markdown
<!-- docs/porting-notes.md -->
# Porting Notes

- Keep this file short.
- Add one bullet per parity-preserving Rust choice that looks unusual.
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test constructors_and_version_exist --test primitives -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml src/lib.rs tests/common/mod.rs tests/primitives.rs docs/porting-notes.md
git commit -m "feat: scaffold Rust crate baseline"
```

### Task 2: Build The C Oracle Workflow And First Fixture

**Files:**
- Create: `tools/oracle/primitives.c`
- Create: `tests/fixtures/primitives.txt`
- Modify: `tests/primitives.rs`

- [ ] **Step 1: Write the failing parity test against a fixture**

```rust
// tests/primitives.rs
mod common;

use common::fixture;
use microui::{color, rect, vec2, Color, Rect, Vec2, MU_VERSION};

#[test]
fn constructors_and_version_exist() {
    assert_eq!(MU_VERSION, "2.02");
    assert_eq!(vec2(1, 2), Vec2 { x: 1, y: 2 });
    assert_eq!(rect(3, 4, 5, 6), Rect { x: 3, y: 4, w: 5, h: 6 });
    assert_eq!(color(7, 8, 9, 10), Color { r: 7, g: 8, b: 9, a: 10 });
}

#[test]
fn primitive_fixture_matches_c_oracle() {
    let fixture_text = fixture("tests/fixtures/primitives.txt");
    let actual = format!(
        "version={}\nvec2={:?}\nrect={:?}\ncolor={:?}\n",
        MU_VERSION,
        vec2(1, 2),
        rect(3, 4, 5, 6),
        color(7, 8, 9, 10),
    );
    assert_eq!(actual, fixture_text);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test primitive_fixture_matches_c_oracle --test primitives -v`
Expected: FAIL with `fixture should exist`

- [ ] **Step 3: Add the oracle program and generate the fixture**

```c
/* tools/oracle/primitives.c */
#include <stdio.h>
#include "../../src/microui.h"

int main(void) {
  mu_Vec2 v = mu_vec2(1, 2);
  mu_Rect r = mu_rect(3, 4, 5, 6);
  mu_Color c = mu_color(7, 8, 9, 10);

  printf("version=%s\n", MU_VERSION);
  printf("vec2=Vec2 { x: %d, y: %d }\n", v.x, v.y);
  printf("rect=Rect { x: %d, y: %d, w: %d, h: %d }\n", r.x, r.y, r.w, r.h);
  printf("color=Color { r: %u, g: %u, b: %u, a: %u }\n", c.r, c.g, c.b, c.a);
  return 0;
}
```

```text
# tests/fixtures/primitives.txt
version=2.02
vec2=Vec2 { x: 1, y: 2 }
rect=Rect { x: 3, y: 4, w: 5, h: 6 }
color=Color { r: 7, g: 8, b: 9, a: 10 }
```

Run: `cc tools/oracle/primitives.c src/microui.c -o /tmp/microui-oracle-primitives && /tmp/microui-oracle-primitives > tests/fixtures/primitives.txt`
Expected: command succeeds and writes the exact fixture text above

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test primitives -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add tools/oracle/primitives.c tests/fixtures/primitives.txt tests/primitives.rs
git commit -m "test: add primitive oracle fixture workflow"
```

### Task 3: Port Constants And Public Types

**Files:**
- Create: `src/constants.rs`
- Create: `src/types.rs`
- Modify: `src/lib.rs`
- Modify: `tests/primitives.rs`

- [ ] **Step 1: Write the failing constant coverage test**

```rust
// tests/primitives.rs
use microui::{
    color, rect, vec2, Color, Rect, Vec2, MU_CLIP_ALL, MU_CLIP_PART, MU_COLOR_MAX,
    MU_COMMAND_CLIP, MU_COMMAND_ICON, MU_COMMAND_JUMP, MU_COMMAND_MAX, MU_COMMAND_RECT,
    MU_COMMAND_TEXT, MU_CONTAINERPOOL_SIZE, MU_IDSTACK_SIZE, MU_KEY_BACKSPACE,
    MU_KEY_RETURN, MU_KEY_SHIFT, MU_MAX_FMT, MU_MAX_WIDTHS, MU_MOUSE_LEFT, MU_OPT_AUTOSIZE,
    MU_OPT_POPUP, MU_REAL_FMT, MU_ROOTLIST_SIZE, MU_SLIDER_FMT, MU_TREENODEPOOL_SIZE,
    MU_VERSION,
};

#[test]
fn exported_constants_match_c_header() {
    assert_eq!(MU_CLIP_PART, 1);
    assert_eq!(MU_CLIP_ALL, 2);
    assert_eq!(MU_COMMAND_JUMP, 1);
    assert_eq!(MU_COMMAND_CLIP, 2);
    assert_eq!(MU_COMMAND_RECT, 3);
    assert_eq!(MU_COMMAND_TEXT, 4);
    assert_eq!(MU_COMMAND_ICON, 5);
    assert_eq!(MU_COMMAND_MAX, 6);
    assert_eq!(MU_COLOR_MAX, 14);
    assert_eq!(MU_MOUSE_LEFT, 1);
    assert_eq!(MU_KEY_SHIFT, 1);
    assert_eq!(MU_KEY_BACKSPACE, 8);
    assert_eq!(MU_KEY_RETURN, 16);
    assert_eq!(MU_OPT_AUTOSIZE, 1 << 9);
    assert_eq!(MU_OPT_POPUP, 1 << 10);
    assert_eq!(MU_ROOTLIST_SIZE, 32);
    assert_eq!(MU_IDSTACK_SIZE, 32);
    assert_eq!(MU_CONTAINERPOOL_SIZE, 48);
    assert_eq!(MU_TREENODEPOOL_SIZE, 48);
    assert_eq!(MU_MAX_WIDTHS, 16);
    assert_eq!(MU_MAX_FMT, 127);
    assert_eq!(MU_REAL_FMT, "%.3g");
    assert_eq!(MU_SLIDER_FMT, "%.2f");
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test exported_constants_match_c_header --test primitives -v`
Expected: FAIL with unresolved imports for the missing constants

- [ ] **Step 3: Add constants and move public types into modules**

```rust
// src/constants.rs
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
```

```rust
// src/types.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
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
```

```rust
// src/lib.rs
mod constants;
mod types;

pub use constants::*;
pub use types::*;
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test primitives -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/constants.rs src/types.rs src/lib.rs tests/primitives.rs
git commit -m "feat: export microui constants and public primitives"
```

### Task 4: Add Fixed-Capacity Storage, Pool Items, And Hash/Clip Tests

**Files:**
- Create: `src/storage.rs`
- Create: `src/pool.rs`
- Create: `tests/storage_and_pool.rs`
- Create: `tests/fixtures/storage_and_pool.txt`
- Create: `tools/oracle/storage_and_pool.c`
- Modify: `src/lib.rs`

- [ ] **Step 1: Write the failing storage and pool tests**

```rust
// tests/storage_and_pool.rs
use microui::{
    color, rect, vec2, Color, PoolItem, Rect, Vec2, MU_CLIP_ALL, MU_CLIP_PART,
    MU_CONTAINERPOOL_SIZE,
};

#[test]
fn pool_item_layout_matches_expectations() {
    let item = PoolItem { id: 7, last_update: 9 };
    assert_eq!(item.id, 7);
    assert_eq!(item.last_update, 9);
}

#[test]
fn clip_classifier_constants_exist() {
    assert_eq!(MU_CLIP_PART, 1);
    assert_eq!(MU_CLIP_ALL, 2);
    assert_eq!(MU_CONTAINERPOOL_SIZE, 48);
    assert_eq!(vec2(1, 2), Vec2 { x: 1, y: 2 });
    assert_eq!(rect(3, 4, 5, 6), Rect { x: 3, y: 4, w: 5, h: 6 });
    assert_eq!(color(7, 8, 9, 10), Color { r: 7, g: 8, b: 9, a: 10 });
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test storage_and_pool -v`
Expected: FAIL with unresolved import for `PoolItem`

- [ ] **Step 3: Add the shared storage and pool primitives**

```rust
// src/storage.rs
#[derive(Clone, Debug)]
pub struct FixedStack<T, const N: usize> {
    pub idx: usize,
    pub items: [T; N],
}

impl<T: Copy + Default, const N: usize> FixedStack<T, N> {
    pub fn new() -> Self {
        Self {
            idx: 0,
            items: [T::default(); N],
        }
    }

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
```

```rust
// src/pool.rs
pub type Id = u32;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PoolItem {
    pub id: Id,
    pub last_update: i32,
}
```

```rust
// src/lib.rs
mod pool;
mod storage;

pub use pool::*;
pub(crate) use storage::*;
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test storage_and_pool -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/storage.rs src/pool.rs src/lib.rs tests/storage_and_pool.rs
git commit -m "feat: add fixed storage and pool primitives"
```

### Task 5: Port Command Types And Packed Command Iteration

**Files:**
- Create: `src/command.rs`
- Create: `tests/commands.rs`
- Create: `tests/fixtures/commands.txt`
- Create: `tools/oracle/commands.c`
- Modify: `src/lib.rs`

- [ ] **Step 1: Write the failing command smoke test**

```rust
// tests/commands.rs
use microui::{CommandType, MU_COMMAND_CLIP, MU_COMMAND_ICON, MU_COMMAND_JUMP, MU_COMMAND_RECT, MU_COMMAND_TEXT};

#[test]
fn command_type_constants_are_mapped() {
    assert_eq!(CommandType::Jump as i32, MU_COMMAND_JUMP);
    assert_eq!(CommandType::Clip as i32, MU_COMMAND_CLIP);
    assert_eq!(CommandType::Rect as i32, MU_COMMAND_RECT);
    assert_eq!(CommandType::Text as i32, MU_COMMAND_TEXT);
    assert_eq!(CommandType::Icon as i32, MU_COMMAND_ICON);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test commands -v`
Expected: FAIL with unresolved import for `CommandType`

- [ ] **Step 3: Add the command surface**

```rust
// src/command.rs
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandType {
    Jump = crate::MU_COMMAND_JUMP,
    Clip = crate::MU_COMMAND_CLIP,
    Rect = crate::MU_COMMAND_RECT,
    Text = crate::MU_COMMAND_TEXT,
    Icon = crate::MU_COMMAND_ICON,
}
```

```rust
// src/lib.rs
mod command;

pub use command::*;
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test commands -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/command.rs src/lib.rs tests/commands.rs
git commit -m "feat: add command type surface"
```

### Task 6: Port Context Skeleton, Input Accumulation, And Default Style

**Files:**
- Create: `src/context.rs`
- Create: `src/input.rs`
- Modify: `src/lib.rs`
- Modify: `tests/primitives.rs`

- [ ] **Step 1: Write the failing context initialization test**

```rust
// tests/primitives.rs
use microui::{Context, MU_VERSION};

#[test]
fn context_init_sets_default_versioned_state() {
    let ctx = Context::new();
    assert_eq!(ctx.frame, 0);
    assert_eq!(ctx.hover, 0);
    assert_eq!(ctx.focus, 0);
    assert_eq!(ctx.style.as_ref().unwrap().title_height, 24);
    assert_eq!(MU_VERSION, "2.02");
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test context_init_sets_default_versioned_state --test primitives -v`
Expected: FAIL with unresolved import for `Context`

- [ ] **Step 3: Add the context skeleton and default style**

```rust
// src/context.rs
use crate::{color, vec2, Color, PoolItem, Vec2, MU_COLOR_MAX};

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

#[derive(Default)]
pub struct Context {
    pub style_storage: Style,
    pub style: Option<Style>,
    pub hover: Id,
    pub focus: Id,
    pub frame: i32,
    pub number_edit: Id,
    pub number_edit_buf: [u8; crate::MU_MAX_FMT],
    pub mouse_pos: Vec2,
    pub last_mouse_pos: Vec2,
    pub mouse_delta: Vec2,
    pub scroll_delta: Vec2,
    pub mouse_down: i32,
    pub mouse_pressed: i32,
    pub key_down: i32,
    pub key_pressed: i32,
    pub input_text: [u8; 32],
    pub container_pool: [PoolItem; crate::MU_CONTAINERPOOL_SIZE],
    pub treenode_pool: [PoolItem; crate::MU_TREENODEPOOL_SIZE],
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
```

```rust
// src/input.rs
use crate::{vec2, Context};

impl Context {
    pub fn input_mousemove(&mut self, x: i32, y: i32) {
        self.mouse_pos = vec2(x, y);
    }
}
```

```rust
// src/lib.rs
mod context;
mod input;

pub use context::*;
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test context_init_sets_default_versioned_state --test primitives -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/context.rs src/input.rs src/lib.rs tests/primitives.rs
git commit -m "feat: add context skeleton and default style"
```

### Task 7: Port Layout State And `layout_next` Under Oracle Tests

**Files:**
- Create: `src/layout.rs`
- Create: `tests/layout.rs`
- Create: `tests/fixtures/layout.txt`
- Create: `tools/oracle/layout.c`
- Modify: `src/lib.rs`
- Modify: `src/context.rs`

- [ ] **Step 1: Write the failing layout parity test**

```rust
// tests/layout.rs
use microui::{fixture_rect_sequence, Context};

#[test]
fn layout_sequence_matches_fixture() {
    let mut ctx = Context::new();
    let actual = fixture_rect_sequence(&mut ctx);
    let expected = std::fs::read_to_string("tests/fixtures/layout.txt").unwrap();
    assert_eq!(actual, expected);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test layout -v`
Expected: FAIL with unresolved import for `fixture_rect_sequence`

- [ ] **Step 3: Add the first public layout helper needed by the test**

```rust
// src/layout.rs
use crate::{rect, Context, Rect};

pub fn fixture_rect_sequence(_ctx: &mut Context) -> String {
    let rows = [
        rect(0, 0, 68 + 10, 10 + 10),
        rect(82, 0, 50, 20),
    ];
    rows.iter()
        .map(|r| format!("{},{},{},{}\n", r.x, r.y, r.w, r.h))
        .collect()
}
```

```rust
// src/lib.rs
mod layout;

pub use layout::*;
```

- [ ] **Step 4: Run test to verify it fails for content mismatch, then replace the stub with the real ported layout logic**

Run: `cargo test --test layout -v`
Expected: FAIL with an assertion diff against `tests/fixtures/layout.txt`

Replace the stub in `src/layout.rs` with the actual transliterated `mu_layout_row`, `mu_layout_width`, `mu_layout_height`, `mu_layout_set_next`, and `mu_layout_next` logic from `src/microui.c`, keeping the same arithmetic order.

- [ ] **Step 5: Run test to verify it passes and commit**

Run: `cargo test --test layout -v`
Expected: PASS

```bash
git add src/layout.rs src/lib.rs src/context.rs tests/layout.rs tests/fixtures/layout.txt tools/oracle/layout.c
git commit -m "feat: port layout progression with parity tests"
```

### Task 8: Port Command Buffer Emission And Iteration Under Oracle Tests

**Files:**
- Modify: `src/command.rs`
- Modify: `src/context.rs`
- Modify: `tests/commands.rs`
- Create: `tests/fixtures/commands.txt`
- Create: `tools/oracle/commands.c`

- [ ] **Step 1: Write the failing command emission parity test**

```rust
// tests/commands.rs
use microui::{Context, MU_COMMAND_RECT};

#[test]
fn emitted_commands_match_fixture() {
    let mut ctx = Context::new();
    let rendered = microui::fixture_commands(&mut ctx);
    let expected = std::fs::read_to_string("tests/fixtures/commands.txt").unwrap();
    assert!(rendered.contains(&format!("type={}", MU_COMMAND_RECT)));
    assert_eq!(rendered, expected);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test commands -v`
Expected: FAIL with unresolved item `fixture_commands`

- [ ] **Step 3: Add a temporary stub, confirm mismatch, then replace it with the real packed buffer port**

```rust
// src/command.rs
pub fn fixture_commands(_ctx: &mut crate::Context) -> String {
    "stub\n".to_string()
}
```

Run: `cargo test --test commands -v`
Expected: FAIL with fixture mismatch

Then replace the stub with:
- packed byte-backed command storage in `Context`
- `push_command`
- `push_jump`
- `set_clip`
- `draw_rect`
- `draw_box`
- `draw_text`
- `draw_icon`
- `next_command`

Mirror the C memory layout with explicit Rust structs plus byte serialization so jump destination ordering and command sizes stay stable.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test commands -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/command.rs src/context.rs tests/commands.rs tests/fixtures/commands.txt tools/oracle/commands.c
git commit -m "feat: port packed command buffer and iteration"
```

### Task 9: Port Pool Logic, Clip Logic, ID Hashing, And Frame Lifecycle

**Files:**
- Modify: `src/context.rs`
- Modify: `src/pool.rs`
- Modify: `src/controls.rs`
- Modify: `tests/storage_and_pool.rs`
- Create: `src/controls.rs`
- Create: `tests/fixtures/storage_and_pool.txt`
- Create: `tools/oracle/storage_and_pool.c`

- [ ] **Step 1: Write the failing parity test for IDs, clipping, and pools**

```rust
// tests/storage_and_pool.rs
use microui::Context;

#[test]
fn ids_clip_and_pool_behavior_match_fixture() {
    let mut ctx = Context::new();
    let actual = microui::fixture_storage_and_pool(&mut ctx);
    let expected = std::fs::read_to_string("tests/fixtures/storage_and_pool.txt").unwrap();
    assert_eq!(actual, expected);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test storage_and_pool -v`
Expected: FAIL with unresolved item `fixture_storage_and_pool`

- [ ] **Step 3: Port the shared state core**

Implement the C-equivalent logic for:
- `begin`
- `end`
- `set_focus`
- FNV-1a hashing
- `get_id`
- `push_id`
- `pop_id`
- `push_clip_rect`
- `pop_clip_rect`
- `get_clip_rect`
- `check_clip`
- `pool_init`
- `pool_get`
- `pool_update`

Keep invariants as hard assertions and preserve branch order from `src/microui.c`.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test storage_and_pool -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/context.rs src/pool.rs src/controls.rs tests/storage_and_pool.rs tests/fixtures/storage_and_pool.txt tools/oracle/storage_and_pool.c
git commit -m "feat: port shared retained-state and clip behavior"
```

### Task 10: Port Shared Control Update Logic And Basic Widgets

**Files:**
- Modify: `src/controls.rs`
- Modify: `src/widgets.rs`
- Create: `src/widgets.rs`
- Create: `tests/controls_basic.rs`
- Create: `tests/fixtures/controls_basic.txt`
- Create: `tools/oracle/controls_basic.c`
- Modify: `src/lib.rs`

- [ ] **Step 1: Write the failing test for buttons, labels, text, and checkbox**

```rust
// tests/controls_basic.rs
use microui::Context;

#[test]
fn basic_controls_match_fixture() {
    let mut ctx = Context::new();
    let actual = microui::fixture_controls_basic(&mut ctx);
    let expected = std::fs::read_to_string("tests/fixtures/controls_basic.txt").unwrap();
    assert_eq!(actual, expected);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test controls_basic -v`
Expected: FAIL with unresolved item `fixture_controls_basic`

- [ ] **Step 3: Port the shared control helpers and basic widgets**

Port the C logic for:
- `draw_control_frame`
- `draw_control_text`
- `mouse_over`
- `update_control`
- `text`
- `label`
- `button_ex`
- `checkbox`

Keep clip behavior, hover-root behavior, and result flag semantics identical to the C code.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test controls_basic -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/controls.rs src/widgets.rs src/lib.rs tests/controls_basic.rs tests/fixtures/controls_basic.txt tools/oracle/controls_basic.c
git commit -m "feat: port shared control helpers and basic widgets"
```

### Task 11: Port Textbox, Slider, And Number Controls

**Files:**
- Modify: `src/widgets.rs`
- Create: `tests/controls_edit.rs`
- Create: `tests/fixtures/controls_edit.txt`
- Create: `tools/oracle/controls_edit.c`

- [ ] **Step 1: Write the failing edit-control parity test**

```rust
// tests/controls_edit.rs
use microui::Context;

#[test]
fn edit_controls_match_fixture() {
    let mut ctx = Context::new();
    let actual = microui::fixture_controls_edit(&mut ctx);
    let expected = std::fs::read_to_string("tests/fixtures/controls_edit.txt").unwrap();
    assert_eq!(actual, expected);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test controls_edit -v`
Expected: FAIL with unresolved item `fixture_controls_edit`

- [ ] **Step 3: Port the editing widgets**

Port the C logic for:
- `textbox_raw`
- `textbox_ex`
- `number_textbox`
- `slider_ex`
- `number_ex`

Specific parity checks to preserve:
- UTF-8 backspace skips continuation bytes
- return key clears focus and sets submit result
- shift-click enters numeric edit mode
- slider step rounding follows the same cast and arithmetic order
- number dragging uses `mouse_delta.x * step`

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test controls_edit -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/widgets.rs tests/controls_edit.rs tests/fixtures/controls_edit.txt tools/oracle/controls_edit.c
git commit -m "feat: port textbox slider and number controls"
```

### Task 12: Port Headers, Tree Nodes, Panels, Windows, And Popups

**Files:**
- Modify: `src/widgets.rs`
- Modify: `src/context.rs`
- Create: `tests/containers.rs`
- Create: `tests/fixtures/containers.txt`
- Create: `tools/oracle/containers.c`

- [ ] **Step 1: Write the failing container-flow parity test**

```rust
// tests/containers.rs
use microui::Context;

#[test]
fn container_flows_match_fixture() {
    let mut ctx = Context::new();
    let actual = microui::fixture_containers(&mut ctx);
    let expected = std::fs::read_to_string("tests/fixtures/containers.txt").unwrap();
    assert_eq!(actual, expected);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test containers -v`
Expected: FAIL with unresolved item `fixture_containers`

- [ ] **Step 3: Port the remaining container and tree logic**

Port the C logic for:
- `header`
- `header_ex`
- `begin_treenode_ex`
- `end_treenode`
- scrollbar helper macro behavior
- `scrollbars`
- `push_container_body`
- `begin_root_container`
- `end_root_container`
- `begin_window_ex`
- `end_window`
- `open_popup`
- `begin_popup`
- `end_popup`
- `begin_panel_ex`
- `end_panel`
- container lookup and `bring_to_front`

Preserve:
- root z-index sorting
- hover-root selection
- scroll target updates
- popup flags and close behavior
- autosize and no-scroll handling

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test containers -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/widgets.rs src/context.rs tests/containers.rs tests/fixtures/containers.txt tools/oracle/containers.c
git commit -m "feat: port containers tree nodes and popups"
```

### Task 13: Port Integration Frame Flows And Finish Public Re-Exports

**Files:**
- Modify: `src/lib.rs`
- Create: `tests/integration_frames.rs`
- Create: `tests/fixtures/integration_frames.txt`
- Create: `tools/oracle/integration_frames.c`
- Modify: `README.md`

- [ ] **Step 1: Write the failing integration parity test**

```rust
// tests/integration_frames.rs
use microui::Context;

#[test]
fn multi_frame_ui_flow_matches_fixture() {
    let mut ctx = Context::new();
    let actual = microui::fixture_integration_frames(&mut ctx);
    let expected = std::fs::read_to_string("tests/fixtures/integration_frames.txt").unwrap();
    assert_eq!(actual, expected);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test integration_frames -v`
Expected: FAIL with unresolved item `fixture_integration_frames`

- [ ] **Step 3: Finish the public surface and integration scenario**

Make sure `src/lib.rs` re-exports every public type, constructor, constant, and widget entry point promised by `src/microui.h`, then implement the integration scenario helper using the same frame order as `doc/usage.md` and the demo patterns.

Update `README.md` with a short Rust usage section that shows:
- creating a `Context`
- setting text callbacks
- calling `begin`, UI functions, and `end`
- iterating commands

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test integration_frames -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/lib.rs tests/integration_frames.rs tests/fixtures/integration_frames.txt tools/oracle/integration_frames.c README.md
git commit -m "feat: complete public Rust API and integration coverage"
```

### Task 14: Run Full Verification And Capture Any Parity-Preserving Oddities

**Files:**
- Modify: `docs/porting-notes.md`

- [ ] **Step 1: Run the full Rust suite**

Run: `cargo test --all-targets -v`
Expected: PASS

- [ ] **Step 2: Regenerate all oracle fixtures and verify there is no drift**

Run: `cc tools/oracle/primitives.c src/microui.c -o /tmp/mu-primitives && /tmp/mu-primitives > tests/fixtures/primitives.txt`
Run: `cc tools/oracle/storage_and_pool.c src/microui.c -o /tmp/mu-storage && /tmp/mu-storage > tests/fixtures/storage_and_pool.txt`
Run: `cc tools/oracle/commands.c src/microui.c -o /tmp/mu-commands && /tmp/mu-commands > tests/fixtures/commands.txt`
Run: `cc tools/oracle/layout.c src/microui.c -o /tmp/mu-layout && /tmp/mu-layout > tests/fixtures/layout.txt`
Run: `cc tools/oracle/controls_basic.c src/microui.c -o /tmp/mu-basic && /tmp/mu-basic > tests/fixtures/controls_basic.txt`
Run: `cc tools/oracle/controls_edit.c src/microui.c -o /tmp/mu-edit && /tmp/mu-edit > tests/fixtures/controls_edit.txt`
Run: `cc tools/oracle/containers.c src/microui.c -o /tmp/mu-containers && /tmp/mu-containers > tests/fixtures/containers.txt`
Run: `cc tools/oracle/integration_frames.c src/microui.c -o /tmp/mu-integration && /tmp/mu-integration > tests/fixtures/integration_frames.txt`
Expected: all commands succeed and `git diff --exit-code tests/fixtures` reports no changes

- [ ] **Step 3: Record the non-obvious parity choices**

```markdown
<!-- docs/porting-notes.md -->
# Porting Notes

- Preserve C arithmetic order in slider stepping before clamping to avoid parity drift.
- Keep hard assertions for invariant failures instead of converting them into recoverable errors.
- Retain fixed-capacity storage and explicit packed command semantics even where a Rust collection would be simpler.
```

- [ ] **Step 4: Run the full suite again**

Run: `cargo test --all-targets -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add docs/porting-notes.md tests/fixtures
git commit -m "test: verify full microui parity suite"
```
