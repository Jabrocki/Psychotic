# Psychotic

`Psychotic` is an experimental first-person shooter prototype written in Rust.
It renders a textured 2D world with a raycaster and generates a fresh dungeon
from room definitions each time the game starts.

> This is a work in progress. The game currently has no menu or persistent
> configuration; it starts directly in fullscreen mode at 1920×1080.

## Features

- raycast-based first-person renderer with textured walls, floors and ceilings;
- procedural 100×100 level generation from the JSON room definitions in
  [`rooms/`](rooms/);
- player movement, mouse-look and hitscan shooting;
- enemies that seek, chase and shoot at the player, including grid pathfinding;
- sprites, HUD/minimap rendering and bundled texture assets.

## Requirements

- a current [Rust toolchain](https://www.rust-lang.org/tools/install), including
  Cargo;
- SDL2 and SDL2_image development libraries.

The Rust dependencies are declared in [`Cargo.toml`](Cargo.toml) and are
downloaded automatically by Cargo on the first build.

### macOS

```bash
brew install sdl2 sdl2_image
```

### Arch Linux

```bash
sudo pacman -S sdl2 sdl2_image
```

For other systems, install the equivalent SDL2 and SDL2_image development
packages with your package manager.

## Run

From the project root:

```bash
cargo run --release
```

Release mode is recommended because debug builds can run noticeably slower.

## Controls

| Input | Action |
| --- | --- |
| Mouse | Look around |
| Arrow keys | Move forward, backward and sideways |
| Left mouse button | Shoot |
| Escape | Quit |

## Development

Run the test suite:

```bash
cargo test
```

Generate and open the API documentation:

```bash
cargo doc --open
```

## Project layout

- `src/renderer.rs` — raycast renderer;
- `src/level/` — room placement, room connection and level generation;
- `src/entity/` and `src/player.rs` — player and enemy behavior;
- `src/pathfinder.rs` — grid pathfinding;
- `rooms/` — source room definitions used by the generator;
- `images/` — game textures and sprites.
