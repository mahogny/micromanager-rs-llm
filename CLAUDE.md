# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```sh
# Build everything (no vendor SDKs needed by default)
cargo build --workspace

# Run all tests
cargo test --workspace

# Test a single adapter module
cargo test zaber

# Test a single test by name
cargo test zaber::stage::tests::move_absolute

# Check without building
cargo check --workspace

# Run the demo (DemoCamera + minifb window)
cargo run -p mm-demo
```

Feature-gated SDK adapters: `cargo build --features andor-sdk3` (requires `ANDOR_SDK3_ROOT`). Other SDK features: `aravis`, `basler`, `daheng`, `jai`, `picam`, `spot`, `tsi`, `twain`, `iidc`, `opencv`.

## Architecture

Pure-Rust port of [MicroManager](https://micro-manager.org/) (`mmCoreAndDevices`). The original C++ source lives in `mmCoreAndDevices/` for reference only — it is never compiled.

### Single-crate structure

Everything lives in one library crate (`micromanager`) with two workspace members: `.` (the library) and `mm-demo` (demo binary). There are no separate `mm-device` or `mm-core` crates — the README's three-layer description is logical, not physical.

**Core modules** (`src/`):
- `traits.rs` — `Device` (base) + device-type traits (`Camera`, `Stage`, `XYStage`, `Shutter`, `StateDevice`, `Hub`, etc.), `AnyDevice` enum, `AdapterModule`
- `property.rs` — `PropertyMap` embedded in every device struct
- `transport.rs` — `Transport` trait (`send`, `receive_line`, `send_recv`, `send_bytes`, `receive_bytes`) + `MockTransport`
- `types.rs` — `PropertyValue`, `DeviceType`, `MmError`
- `core.rs` — `CMMCore` orchestrator: loads devices via `AdapterRegistry`, dispatches to `DeviceManager`, manages `CircularBuffer`
- `device_manager.rs`, `adapter_registry.rs`, `circular_buffer.rs`, `config.rs`

**Adapters** (`src/adapters/`): ~100+ submodules, one per hardware family. Each adapter module contains device structs that implement `Device` + the appropriate trait(s).

### Key patterns used in every adapter

```rust
// Device struct — always embeds PropertyMap + optional Transport
pub struct FooStage {
    props: PropertyMap,
    transport: Option<Box<dyn Transport>>,
    // ... device-specific state
}

// Builder for injecting transport (tests use MockTransport)
pub fn with_transport(mut self, t: Box<dyn Transport>) -> Self {
    self.transport = Some(t); self
}

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

1. Create `src/adapters/<name>/mod.rs` (and sub-files as needed)
2. Add `pub mod <name>;` to `src/adapters/mod.rs`
3. Implement `Device` + the device-type trait(s); embed `PropertyMap` and `Option<Box<dyn Transport>>`
4. Add a row to the adapter table in `README.md`

### Build system (`build.rs`)

Uses `cc` to compile C/C++ shim files for vendor SDK adapters, gated behind Cargo features. Each SDK adapter looks for an environment variable (e.g., `ANDOR_SDK3_ROOT`, `EBUS_SDK_ROOT`). Default builds require no vendor SDKs.

### SDK-dependent adapters

Adapters requiring vendor SDKs (closed DLLs / proprietary C headers) cannot be pure serial — e.g. `KuriosLCTF` (Thorlabs Windows DLLs), `Okolab` (`okolib.h`), `Lumencor` main adapter (`LightEngineAPI`). These should be skipped or flagged.
