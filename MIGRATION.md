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
    - [x] Port `Score` struct from breeding planner.
    - [ ] Add `Creature` stat fields and implement breeding pair scoring.
9. [ ] **Persist creature library**.
    - [ ] Serialize and deserialize `Creature` records to JSON.
    - [ ] Provide CLI commands to add/remove creatures.
    - [ ] Write tests for library persistence.
10. [ ] **GUI for creature library**.
    - [ ] List creatures with filtering by species and sex.
    - [ ] Allow adding and removing creatures via the GUI.
11. [ ] **Server multiplier profiles**.
    - [ ] Parse multiplier files and allow selecting a profile via CLI flag.
    - [ ] Apply multipliers when calculating stats in the library and breeding planner.
12. [ ] **Breeding planner implementation**.
    - [ ] Calculate possible offspring levels and mutation chances.
    - [ ] Display top scoring breeding pairs in the GUI.
    - [ ] Unit tests for breeding calculations.
13. [ ] **OCR import**.
    - [ ] Use the `leptess` crate to read creature data from screenshots.
    - [ ] Provide a CLI command and GUI button to trigger OCR import.
14. [ ] **Packaging and release**.
    - [ ] Build native binaries for major platforms with `cargo build --release`.
    - [ ] Update `README.md` with build and usage instructions.

Each migration step should be kept small, with tests and formatting run after every change to ensure stability.
