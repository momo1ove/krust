# krust (procedural FPS prototype in Rust)

A tiny, self-contained **procedural FPS** prototype inspired by the “small-but-complete” demoscene spirit of **.kkrieger**.

- **No external texture assets**: surfaces use a runtime-generated procedural texture.
- **FPS controls**: mouselook + WASD.
- **Shooting**: click-to-shoot + hit feedback.
- **UI**: crosshair + minimal HUD.

Repository: https://github.com/momo1ove/krust

---

## Features

- Procedural 256×256 sRGB texture generated at startup (Perlin noise + simple striping)
- Simple room geometry (floor + walls) and random target boxes
- Cursor lock + mouse look
- WASD / arrow-key movement (camera kept at eye height)
- Left click to shoot (ray-based hit test)
- Hit feedback (targets despawn + a small “explosion” visual)
- Crosshair (center screen)
- HUD: controls hint + hit counter

> Notes
> - This is a **prototype** (no physics/collision yet).
> - It is intended to be easy to extend: procgen rooms, raycast-based interactions, size optimization, etc.

---

## Requirements

- Linux (tested on Ubuntu 24.04)
- Rust toolchain (stable)

If you don’t have Rust:

```bash
curl https://sh.rustup.rs -sSf | sh -s -- -y
source "$HOME/.cargo/env"
```

On Ubuntu, Bevy typically needs a few system libraries:

```bash
sudo apt-get update
sudo apt-get install -y \
  build-essential pkg-config \
  libx11-dev libxi-dev libxkbcommon-dev libwayland-dev \
  libasound2-dev libudev-dev
```

---

## Build & Run

### Debug

```bash
cargo run
```

### Release (smaller + faster)

```bash
cargo run --release
```

Release binary output:

```bash
./target/release/kkrieger-rust-fps
```

---

## Controls

- **Mouse**: look around
- **W/A/S/D** or **Arrow keys**: move
- **Left click**: shoot

---

## Project layout

- `src/main.rs` — single-file prototype
- `.gitignore` — excludes build artifacts (`target/`) and editor files
- `STATUS.md` — progress log used by the VPS hourly report script

---

## Next steps (ideas)

- Procedural room/corridor generator (seeded)
- Simple collision + acceleration/friction
- Better material variety (multiple procedural textures, triplanar, etc.)
- Binary size optimization
  - Reduce Bevy features
  - `panic = "abort"`, LTO, strip (already enabled)

---

## License

MIT (or pick one if you want—currently not set).
