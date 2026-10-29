# Rust Plugin Template

A minimal Inbox Grinder plugin written in Rust that compiles to both a **native binary** (`local-binary` runtime) and a **WebAssembly module** (`wasm-component` runtime).

## Structure

```
rust-template/
├── Cargo.toml          — workspace-free crate (cdylib + binary targets)
├── manifest.json       — binary runtime (default)
├── manifest-wasm.json  — WASM runtime
├── build.sh            — builds both artifacts
├── src/
│   ├── plugin.rs       — shared extraction logic
│   ├── main.rs         — binary entry (reads stdin, writes stdout)
│   └── lib.rs          — WASM exports (alloc / process)
└── fixtures/
    ├── email-input.json
    └── extraction-result.json
```

## Quick start

### Binary (no toolchain extras needed)

```sh
cargo build --release --bin plugin
cp target/release/plugin plugin
# Test manually:
./plugin < fixtures/email-input.json | jq .
```

Install the binary variant into the app by dropping the folder (with `manifest.json`) onto the Plugins screen.

### WASM

First add the WASM target once:

```sh
rustup target add wasm32-unknown-unknown
```

Then build:

```sh
cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/rust_template.wasm plugin.wasm
# Switch manifest to WASM:
cp manifest-wasm.json manifest.json
```

Or run `build.sh` to produce both artifacts at once.

## Making your own plugin

1. **Copy this directory** and rename it (e.g., `my-plugin`).
2. **Edit `manifest.json`** — change `id`, `name`, `matches`, and `permissions`.
3. **Edit `src/plugin.rs`** — implement your extraction logic in `run()`.
4. Build and drop the folder into the app.

## WASM ABI

The WASM host calls:

| Export | Signature | Purpose |
|--------|-----------|---------|
| `alloc` | `(i32) → i32` | Allocate `n` bytes; host writes input JSON there |
| `process` | `(i32, i32) → i64` | Run extraction; return `(output_ptr << 32) | output_len` |
| `memory` | — | Linear memory shared with the host |

`process` returns `0` on error (host treats this as plugin failure).

## Notes

- `lib.rs` uses `thread_local!` + `RefCell<Vec<u8>>` for the output buffer — WASM is single-threaded so this is safe.
- `alloc` delegates to the global allocator (`std::alloc::alloc`). The allocation is never freed; the module is discarded after each call.
- The binary variant reads the full `EmailInput` JSON from stdin and writes `ExtractionResult` JSON to stdout. Non-zero exit code signals failure; diagnostic output goes to stderr.
