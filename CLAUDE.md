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
- Test: `cargo test` (no tests exist yet)
- Format: `cargo fmt`
- Lint: `cargo clippy`
