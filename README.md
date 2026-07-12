# metaballs-rs

A small [metaballs](https://en.wikipedia.org/wiki/Metaballs) renderer in Rust.

The core (`src/lib.rs`) is graphics-API agnostic: it simulates the balls and
writes RGB pixels into a plain buffer. What you do with that buffer is up to the
frontend, the `examples/` show three of them.

## Usage

```rust
use metaballs_rs::{Action, Metaballs};

let mut app = Metaballs::setup();
app.update(&[Action::MovePlayerBy { x: 250, y: 0 }], dt);
let buffer = app.buffer(); // &[Pixel], BUFFER_WIDTH * BUFFER_HEIGHT
```

## Examples

- **`simple`**: renders frames to PPM and stitches them into a video with
  `ffmpeg`. Run `./make_animation.sh` from `examples/simple/`.

  > [!WARNING]
  >
  > Read the script before running it. It has a `cleanup` trap that runs
  > `rm -rf out/` on exit, so don't run it in a directory where you keep an
  > `out/` you care about.

- **`raylib-demo`**: interactive desktop window; move the ball with the arrow
  keys. `cd examples/raylib-demo && cargo run --release`.

- **`wasm-demo`**: the same, in the browser via `wasm-bindgen`:

  ```bash
  cd examples/wasm-demo
  wasm-pack build --target web
  python3 -m http.server 8000   # then open http://localhost:8000
  ```

> [!WARNING]
>
> `raylib-demo` and `wasm-demo` are largely vibe-coded scaffolding around the
> core library.

## Credits

The idea and the "the graphics API is irrelevant, just fill a buffer" approach comes
from Tsoding: [Graphics API is irrelevant](https://youtu.be/xNX9H_ZkfNE).
