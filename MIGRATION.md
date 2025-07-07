# Rust Migration Plan

This document outlines a stepwise approach to port the existing .NET codebase to Rust. Each step builds on the previous so that the project compiles and tests pass throughout the migration.

1. [x] **Initialize Rust workspace** (`cargo init`).
2. [x] **Add CLI dependencies**: `cargo add clap@4.5.40`.
3. [x] **Add serialization dependencies**: `cargo add serde@1.0.219 serde_json@1.0.140`.
4. [x] **Add GUI framework**: `cargo add eframe@0.31.1`.
5. [x] **Port core structs**: `Species`, `Stats`, `ServerMultipliers`.
6. [x] **Implement JSON loading tests** to verify data parsing.
7. [x] **Build minimal CLI and GUI** that load and display data.
8. [ ] **Continue porting features** such as the breeding planner, OCR, and other tools.

Each migration step should be kept small, with tests and formatting run after every change to ensure stability.
