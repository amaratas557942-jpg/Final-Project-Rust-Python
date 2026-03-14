# Game Backlog

**Desktop Application — Built with Rust · egui · eframe**

---

## Overview

Game Backlog is a native desktop application for tracking your personal game library. Built in Rust using the egui immediate-mode GUI framework and eframe, it provides a fast, dark-themed interface for managing games across four play states: **Backlog**, **Playing**, **Completed**, and **Dropped**.

The app ships with sample data so you can explore all features immediately on first launch.

---

## Project Info

| Property | Details |
|---|---|
| Language | Rust (2021 Edition) |
| GUI Framework | egui + eframe |
| File | `main.rs` |
| Window Size | 1200 × 750 px (min: 900 × 550 px) |
| UI Scale | 1.2× pixels per point |
| Theme | Custom dark theme (navy / slate palette) |

---

## Features

### Game Management
- Add, edit, and delete games from your library
- Track play status: Backlog, Playing, Completed, or Dropped
- Assign a rating (1–10) and optional genre and notes
- Pin favourite games to the top of the list
- Quick-change status directly from any game card without opening the edit form

### Navigation & Filtering
- Tab bar to filter the list by status (All, Backlog, Playing, Completed, Dropped)
- Live search across game title, genre, and notes
- Summary stats in the header: per-status counts at a glance
- Pinned games always float above the rest within the current filter

### UI / UX
- Keyboard shortcut: press `N` anywhere to open the Add Game form
- Color-coded left strip on each card matches the game's status
- Gold star rating displayed on each card
- Italic notes shown inline on each card
- Confirmation-free deletion (delete button triggers removal immediately)

---

## Data Structures

### Status Enum

Four variants — `Backlog`, `Playing`, `Completed`, `Dropped` — each with a display label, icon character, and associated `Color32` for visual coding.

| Variant | Icon | Color |
|---|---|---|
| Backlog | ◉ | Slate gray |
| Playing | ▶ | Green |
| Completed | ✓ | Sky blue |
| Dropped | ✕ | Red |

### Game Struct

| Field | Type | Description |
|---|---|---|
| `id` | `usize` | Auto-incremented unique identifier |
| `title` | `String` | Game title (required) |
| `genre` | `String` | Genre label (optional) |
| `status` | `Status` | Current play state |
| `rating` | `u8` | Score 0–10 (0 = unrated) |
| `notes` | `String` | Free-text personal notes |
| `pinned` | `bool` | Pinned to top of list |
| `date_added` | `String` | Display date string |

---

## Build & Run

### Prerequisites
- Rust toolchain (stable) — install via [rustup.rs](https://rustup.rs)
- Cargo (included with Rust)
- On Linux: system libraries for egui/eframe (`libxcb`, `libxkbcommon`, etc.)

### Dependencies (`Cargo.toml`)

```toml
[dependencies]
eframe = "0.28"
egui   = "0.28"
```

### Commands

| Command | Purpose |
|---|---|
| `cargo run` | Build and launch the app (debug) |
| `cargo run --release` | Build optimized release binary |
| `cargo build --release` | Compile without running |
| `cargo check` | Fast compile check without producing binary |

---

## Architecture Notes

The entire application is contained in a single file (`main.rs`) using egui's immediate-mode pattern. The `App` struct holds all state and the `update()` method is called every frame to rebuild the UI. There is no persistent storage — all data lives in memory for the duration of the session.

### Key Impl Blocks

| Method | Description |
|---|---|
| `App::with_sample_data()` | Initialises the app with 8 pre-loaded games |
| `App::visible_games()` | Returns filtered, pin-sorted game IDs for the current tab and search query |
| `App::save()` | Handles both create and update via `editing: Option<usize>` |
| `eframe::App::update()` | Main render loop: sidebar stats, tab bar, search, game cards, modal form |

---

## Keyboard Shortcuts

| Key | Action |
|---|---|
| `N` | Open Add Game form |
| `Esc` | Close the Add / Edit form (via Cancel button) |

---

## Notes & Limitations

- **No persistence** — all data is lost when the app closes. Consider integrating `serde` + a JSON file for save/load.
- **No undo** — deleting a game cannot be reversed within a session.
- `date_added` is hardcoded to `"Mar 2026"` and is not dynamically set.
- Single-file architecture keeps things simple but may benefit from module separation as the project grows.
