# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A GTK 4 / libadwaita desktop application that renders 3D dice (D4, D6, D8, D10, D12, D20) using OpenGL via Glium. Written in Rust.

## Build System

The project uses **Meson** as the primary build system, which orchestrates Cargo for Rust compilation and Blueprint compiler for UI files.

```bash
# Build
meson setup builddir
meson compile -C builddir

# Install (to GNOME standard directories)
meson install -C builddir
```

### macOS build requirements

Install via Homebrew: `gtk4`, `libadwaita`, `meson`, `blueprint-compiler`, `desktop-file-utils`.

The `gettext-sys` crate must link against Homebrew's gettext (not compile from source):
```bash
GETTEXT_DIR=/opt/homebrew/opt/gettext meson compile -C builddir
```

Flatpak packaging is configured via `org.lesslie.dice.json` targeting GNOME Platform 46.

## Architecture

**Entry point** (`main.rs`): Loads libepoxy for OpenGL, loads GResources, and launches the GTK application.

**Core modules:**
- `application.rs` — GTK Application subclass, lifecycle management, GActions
- `window.rs` — Main window with dice buttons; delegates rendering to `DiceArea`
- `dice_area.rs` — Custom GLArea widget implementing Glium's `Backend` trait. Contains the OpenGL renderer with instanced rendering: separate vertex/index buffers per die type, per-instance world matrices, vertex+fragment shaders (GLSL 300 ES / 150). This is the most complex module (~1100 lines).
- `die.rs` — `DieKind` enum and `Die` struct with random value generation
- `animated_roll.rs` — Animation system (work in progress)
- `config.rs` — Build-time constants generated from `config.rs.in` by Meson

**UI templates** (Blueprint `.blp` files in `src/`): Compiled to GTK XML at build time. `window.blp` defines the toolbar, GLArea canvas, and dice buttons.

**Interaction flow:** User clicks a dice button → window adds a die to `DiceArea`'s dice vector → renderer creates/updates per-instance buffers → tick callback drives continuous rendering.

## Key Dependencies

- `gtk4` (0.7) + `libadwaita` (0.5) — UI framework
- `glium` (0.34) — OpenGL wrapper and state management
- `graphene-rs` (0.20) — 3D math/transforms
- `epoxy` (0.1) + `gl` (0.14) — OpenGL loading
- `rand` (0.8) — Dice roll RNG

## Notes

- No test infrastructure currently exists
- The depth buffer has had issues (see git history) — reverted in commit f2a5a74
- D10 geometry currently uses a tetrahedron placeholder
- Application ID: `org.lesslie.dice`
- License: GPL-3.0-or-later
