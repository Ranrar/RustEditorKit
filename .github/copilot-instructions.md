<!--
Use this file to provide workspace-specific custom instructions to Copilot.
More info: https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file
-->

This workspace is the `rusteditorkit` project — a cross-platform, GTK4-based Rust editor engine with full rendering, input, and configuration systems. It is designed for modularity, configurability, and embeddability.

## Architecture Overview
- `src/lib.rs`: Public API entry point.
- `src/corelogic/`: Core editor logic (buffer, editing, cursor, undo, file I/O).
- `src/render/`: Custom rendering engine (Pango + GTK4), dirty region invalidation, cursor, gutter, theme, etc.
- `src/config/`: Configuration system with `.ron` support and per-platform fallback.
- `src/keybinds/`: Platform-specific and shared keybindings (Linux/macOS/Windows).
- `src/widget/` : High-level GTK widget binding all components together.
- `src/imcontext.rs`: IME integration logic.
- `src/selection.rs`, `src/multicursor.rs`, `src/indent.rs`: Advanced editing features.
- `doc/`: Project documentation (API, Roadmap, Widget interface).

## Summary Table

| Layer      | API/Logic Location            | Responsibility            | Recommendation                         |
|------------|-------------------------------|---------------------------|----------------------------------------|
| CoreLogic  | `buffer.rs`, `config.rs`      | Data/config only          | Expose unified line height             |
| Render     | `render/text.rs`, `gutter.rs` | All drawing, highlight logic | Draw highlight, use baseline         |
| Widget     | `editor.rs`                   | Delegation only           | Call render layer, no drawing          |

## Copilot Guidelines
- Always prioritize **clean**, **idiomatic**, and **safe** Rust.
- Prefer modular functions with minimal side effects.
- Comment all public APIs with `///` doc comments.
- Use feature flags when logic must be platform-specific.
- Match the existing code style (Rust 2021 edition, snake_case, `mod.rs` or inline modules).
- When writing render logic, favor **dirty region updates** and **GPU-friendly batching**.
- Prefer immutable data where possible, and `Rc<RefCell<>>` only when necessary.

## Suggestions for Code Generation
- Use `corelogic::buffer` and `corelogic::cursor` as the editing foundation.
- Add rendering features inside `render/`, behind functions like `fn draw_background(...)`, `fn draw_gutter(...)`, etc.
- When adding keybindings, group logic per-platform (`keybinds/linux.rs`, etc.).
- When reading configuration, prefer structured `.ron` via `config/config.ron`.

## Testing
- All logic should be unit-testable without GTK runtime (use `#[cfg(test)]` in modules).
- Avoid panics in production code. Use `Result` or graceful error propagation.


## Cross-Platform Notes
- Code must compile and work on **Linux, Windows, and macOS**.
- Rendering is based on **GTK4** and **Pango**, and designed to be adaptable for other frontends (e.g., Slint or Egui in the future).


## Future-Proofing
- Keep feature boundaries sharp: core logic, rendering, and platform integration should stay separate.
- Refer to `doc/roadmap.md` for planned refactors and milestones.