# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```sh
# Build everything
cargo build --workspace

# Run all tests
cargo test --workspace

# Test a single crate
cargo test -p mm-adapter-zaber

# Test a single test by name
cargo test -p mm-adapter-zaber move_absolute

# Check without building
cargo check --workspace
```

## Architecture

This is a pure-Rust port of MicroManager (`mmCoreAndDevices`). The original C++ source lives in `mmCoreAndDevices/` for reference only — it is never compiled.

### Three-layer structure

**`mm-device`** — trait definitions only, no hardware logic:
- `traits.rs` — `Device` (base) + device-type traits (`Camera`, `Stage`, `XYStage`, `Shutter`, `StateDevice`, `Hub`, etc.), plus `AnyDevice` enum for type-safe dispatch and `AdapterModule` for registration
- `property.rs` — `PropertyMap` embedded in every device struct (analogous to `CDeviceBase<T,U>`)
- `transport.rs` — `Transport` trait (`send`, `receive_line`, `send_recv`, `send_bytes`, `receive_bytes`) + `MockTransport` for tests
- `types.rs` — `PropertyValue`, `DeviceType`, `MmError`

**`mm-core`** — the engine, depends on `mm-device`:
- `CMMCore` orchestrates everything: loads devices via `AdapterRegistry`, dispatches to `DeviceManager`, manages `CircularBuffer` for sequences
- Adapters register via `CMMCore::register_adapter(Box<dyn AdapterModule>)` at runtime (no dynamic linking)

**`adapters/mm-adapter-*`** — one crate per hardware family, depends only on `mm-device`:
- Each implements `Device` + the appropriate device-type trait(s)
- Embed `PropertyMap` and `Option<Box<dyn Transport>>`
- Use `with_transport(Box<dyn Transport>)` builder pattern; tests inject `MockTransport`

### Key patterns used in every adapter

```rust
// Transport dispatch — avoids lifetime issues
fn call_transport<R, F>(&mut self, f: F) -> MmResult<R>
where F: FnOnce(&mut dyn Transport) -> MmResult<R> { ... }

// Build command string then send_recv
fn cmd(&mut self, command: &str) -> MmResult<String> {
    let full = format!("...", ...);
    self.call_transport(|t| Ok(t.send_recv(&full)?.trim().to_string()))
}
```

### MockTransport in tests

```rust
MockTransport::new()
    .expect("/1 1 get pos\n", "@01 01 IDLE -- 0")  // exact cmd → scripted response
    .any("ok")                                       // wildcard: any cmd → response
    .expect_binary(b"\x01\x02")                     // for receive_bytes()
```

`send_bytes(&[u8])` records to `mock.received_bytes: Vec<Vec<u8>>` (no scripted responses needed for pure-write devices). `send(cmd)` records to `mock.received: Vec<String>`.

### Trait method signatures to remember

- `StateDevice::get_number_of_positions(&self) -> u64` (not `MmResult`)
- `XYStage::get_step_size_um(&self) -> (f64, f64)` (bare tuple)
- `XYStage::get_limits_um(&self) -> MmResult<(f64, f64, f64, f64)>`
- `Stage::get_limits(&self) -> MmResult<(f64, f64)>`
- `Shutter::fire(&mut self, delta_t: f64) -> MmResult<()>`

### Adding a new adapter

1. `cargo new --lib adapters/mm-adapter-<name>` with `mm-device = { path = "../../mm-device" }` dependency
2. Add `"adapters/mm-adapter-<name>"` to workspace `Cargo.toml`
3. Implement `Device` + the device-type trait(s); embed `PropertyMap` and `Option<Box<dyn Transport>>`
4. Add a row to the adapter table in `README.md`

Adapters that require vendor SDKs (closed DLLs / proprietary C headers) cannot be implemented as pure serial adapters and should be skipped or flagged — e.g. `KuriosLCTF` (Thorlabs Windows DLLs), `Okolab` (`okolib.h`), `Lumencor` main adapter (`LightEngineAPI`).
