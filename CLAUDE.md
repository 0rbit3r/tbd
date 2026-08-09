# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this project is

`tbd` ("to be done") is the user's first Rust project — a learning exercise. It is a task
management CLI, and its own task list (the tasks needed to design, build, and release `tbd`
itself) is intended to eventually be managed *by* `tbd`.

The project is meant to grow deliberately, one step at a time, from the current minimal
hardcoded prototype toward a real program: proper library/binary split, tests, error handling,
configurability, and eventually a TUI. Do not jump ahead of where the user currently is —
e.g. it's fine for storage to be hardcoded to `~/.tbd/tasks` for now; that only needs to become
configurable once the software is closer to being considered complete.

## Critical rule: do not write code unless explicitly asked

This repo exists so the user can learn Rust by writing it themselves. **Never generate or edit
code unless the user explicitly tells you to.** Even when explicitly asked to touch code, keep
edits limited to:
- restructuring/reorganizing existing code
- showing small example snippets to illustrate a concept
- fixing small mistakes (typos, obvious syntax errors)

All substantial ("heavy lifting") implementation work is done by the user by hand. When in
doubt about whether a request counts as an explicit go-ahead to write code, ask first.

## Teaching style: hint before answering

When asked a question about Rust or how to approach something, prefer giving a hint first
rather than the full answer directly, when it makes sense to do so — this project doubles as
a learning playground. Let the user attempt it before revealing the complete solution.

The user has a CS background and professional experience in C#/.NET, TypeScript, and web
development, but this is their first time writing Rust. When explaining new Rust concepts,
lean on analogies to .NET/C# (e.g. ownership vs. GC, traits vs. interfaces, `Option`/`Result`
vs. nullable types and exceptions, cargo vs. NuGet/dotnet CLI) rather than starting from
first principles — they don't need general programming or CS fundamentals explained.

## Commands

- Build: `cargo build`
- Run: `cargo run`
- Check (fast compile check): `cargo check`
- Test: `cargo test`
- Run a single test: `cargo test <test_name>` (e.g. `cargo test parse_shallow`)
- Format: `cargo fmt`
- Lint: `cargo clippy`

## Architecture

`tbd` is split into a library (`src/lib.rs`) consumed by a thin binary (`src/main.rs`).

- **`task::Task`** — a task has a `title`, a `TaskState`, and a `Vec<Task>` of `subtasks`
  (tasks nest arbitrarily deep). `Task::render` turns a task (and its subtasks, indented 4
  spaces per level) back into its on-disk text representation.
- **`task_state::TaskState`** — enum of `Untouched`, `Started`, `Skipped`, `Done`, `Corrupted`.
  Each state maps to a fixed text "decoration" prefix in the file format (`[ ]`, `[.]`, `[-]`,
  `[x]`; `Corrupted` has no on-disk decoration — it's the fallback for unparseable lines).
- **`task_file::TaskFile`** — owns the parsed task list plus an optional file `path`, and is
  the entry point for reading/writing `.tbd` files (`from_file`, `from_string`, `save_as`,
  `save_file`). It's a thin `mod.rs`-style file that pulls in three private submodules:
  - `task_file::parse` — turns file text into a `Vec<Task>` line by line. Indentation (4
    spaces per level) determines subtask nesting; a line that doesn't match a known state
    decoration becomes a `Corrupted` task. Round-tripping (`parse` -> `render`) is expected to
    be lossless for valid files, and is asserted directly in tests.
  - `task_file::indexing` — converts a single flat `usize` index (as seen by a user scrolling
    through the fully-expanded, depth-first-flattened task list) into a `multi_index`
    (`Vec<usize>`, one index per tree level) that can address a specific nested task. Used to
    let callers refer to tasks positionally without walking the tree themselves.
  - `task_file::insert` — inserts a new `Task` into the tree at a given `multi_index`.
- Task file format on disk: one task per line, `<decoration> <title>`, with subtasks indented
  4 spaces deeper than their parent. See `tasks.tbd` (this project's own task list, eventually
  meant to be managed by `tbd` itself) for a real example, and the tests in
  `src/task_file/parse.rs` for the format's edge cases.
