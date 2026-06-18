# microui Rust Port Design

Date: 2026-06-18
Target: port `src/microui.c` and `src/microui.h` fully into idiomatic strict Rust

## Goal

Port the original microui library into a Rust-only crate that preserves the full
public API surface and behavior of the C implementation, while expressing that
surface in idiomatic Rust constructs instead of C macros and typedefs.

The acceptance bar is strict behavioral parity, including undocumented edge
cases visible in the C source.

## Decisions

- Public compatibility means preserving the full function set and behavior while
  translating C macros, constants, and typedef-style constructs into idiomatic
  Rust equivalents.
- ABI compatibility with C is not required.
- Undocumented edge cases are in scope and must be matched exactly where they
  are observable through public behavior or context state.
- The port will be executed with red-green TDD, using the original C
  implementation as the behavioral oracle.

## Recommended Approach

Use a direct transliteration-first strategy.

The implementation should stay close to the C source internally during the
first pass so that fixed-capacity storage, pool mechanics, command packing,
layout progression, focus handling, and widget behavior can be matched without
reinterpreting the design. Once parity is proven by tests, limited internal
cleanup is allowed where it does not alter behavior.

This is preferred over a fresh Rust-first redesign because the project requires
exact parity, including undocumented quirks such as pool eviction order, command
ordering, focus transitions, and textbox edge behavior.

## Architecture

The crate will expose a Rust-native public API that preserves the original
conceptual model:

- `Context`, `Style`, `Layout`, `Container`, `Command`, `Rect`, `Vec2`,
  `Color`, `PoolItem`
- lifecycle functions such as init, begin, end, input collection, command
  iteration, layout helpers, and container management
- built-in controls for text, labels, buttons, checkbox, textbox, slider,
  number, header, tree node, window, popup, and panel
- constants and flags for commands, colors, icons, result bits, options, mouse
  buttons, and keys

Internally the implementation will use two layers:

1. A parity-focused core that mirrors the C implementation’s state machine,
   storage model, and widget logic closely.
2. A thin public Rust surface that exposes idiomatic Rust types and helper
   constructors without changing semantics.

Key constraints:

- no heap allocation inside the UI engine
- fixed capacities remain compile-time constants matching the original values
- user-supplied callbacks remain part of `Context`
- command generation remains explicit and replayable through iteration

## Internal Components

The implementation should be split into small modules aligned with the original
source responsibilities:

- `types`: primitives, numeric aliases, constants, flags, and command structs
- `storage`: fixed-capacity stack and buffer helpers replacing C macros
- `context`: context state, default style, frame lifecycle, ID hashing, and
  stack ownership
- `command`: packed command buffer writing and iteration
- `layout`: row and column layout progression, next-rect logic, relative and
  absolute placement
- `pool`: retained-state container and tree node pool behavior
- `input`: mouse, key, scroll, and text input accumulation
- `controls`: shared helpers for hover, focus, frame drawing, and control text
- `widgets`: concrete built-in controls and container widgets

Implementation rules:

- packed command storage must preserve command sizes, ordering, jump semantics,
  and iteration behavior
- arithmetic and branch ordering should stay close to the C source in
  parity-sensitive paths
- internal assertion failures remain hard failures
- public helpers equivalent to `mu_vec2`, `mu_rect`, and `mu_color` should
  exist, while normal Rust struct construction should also work

## TDD And Verification Strategy

The port will use the original C implementation as the source of truth.

Workflow:

1. Build a behavior harness around the original C code.
2. Capture observable results for targeted scenarios.
3. Write failing Rust tests for those scenarios.
4. Implement only enough Rust code to make the tests pass.
5. Repeat subsystem by subsystem until the full surface is covered.

Observable behavior to lock down:

- return flags from all public controls
- context state transitions such as `hover`, `focus`, `last_id`, `last_rect`,
  `updated_focus`, `scroll_target`, and `number_edit`
- retained-state pool lookup, update, and eviction behavior
- layout progression and computed rects
- emitted command stream contents, ordering, clipping behavior, jump linking,
  and final iteration order
- textbox UTF-8 backspace behavior and submit semantics
- slider stepping behavior and number edit mode
- root container ordering, z-index promotion, popup handling, and scrolling

## Error Handling

The Rust port should preserve the original failure model for invariant
violations. Cases that would trigger the C `expect(...)` assertions, such as:

- stack overflow or underflow
- command buffer overflow
- missing required callbacks before `begin`
- invalid internal state transitions

should remain hard failures through explicit Rust assertions in parity-critical
paths.

Normal UI outcomes continue to be reported only through the original result-flag
semantics.

## Test Coverage Scope

The final test suite should include:

- unit tests for primitives, hash and ID generation, clip logic, stack and pool
  behavior, layout math, and input accumulation
- parity tests for each public widget and container flow
- golden command-stream tests for representative multi-control frames
- regression tests for tricky edge cases including partial clipping, jump
  linking, root sorting, focus clearing, popup open state, tree node retention,
  UTF-8 backspace, slider stepping, and shift-click number editing
- integration scenarios modeled on the documented usage flow and demo patterns

## Sequencing

Implementation should proceed in this order:

1. Establish the spec and implementation plan.
2. Create the C-behavior harness and the first failing parity tests.
3. Port foundational types and fixed-capacity storage.
4. Port context lifecycle, pool logic, command buffer, and layout.
5. Port shared control and input behavior.
6. Port widgets one by one under red-green tests.
7. Run full parity verification.
8. Only after parity is green, perform limited internal cleanup where tests
   prove equivalence.

## Non-Goals

- changing the library’s interaction model
- adding new controls or features
- introducing heap-backed dynamic storage as a substitute for fixed capacities
- redesigning the public API around a more opinionated Rust UI abstraction

## Success Criteria

The port is complete when:

- every public feature from `src/microui.h` and `src/microui.c` is present in
  the Rust crate
- documented and undocumented observable behavior matches the C implementation
- the Rust test suite proves parity across primitives, state transitions,
  command emission, widgets, and integration flows
- the public Rust API is idiomatic in representation without changing semantics
