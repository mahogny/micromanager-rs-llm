# claude-micromanager

A pure-Rust port of [MicroManager](https://micro-manager.org/) (`mmCoreAndDevices`). No C FFI, no Java bindings — Rust API only.

The port is based on https://github.com/micro-manager/mmCoreAndDevices, hash 67fe60267bc8d95554369d7fa42912775588e538

The license follows from the original code. To simplify Rust integration, the core code will be replaced, and also made
to be less monolithic while at it.

## Structure

```
claude-micromanager/
├── mm-device/          # Trait definitions (replaces MMDevice/)
├── mm-core/            # Engine: device manager, config, circular buffer (replaces MMCore/)
└── adapters/           # Hardware adapters, one crate per device family
```

### `mm-device`

Defines the core abstractions:

- **Traits** — `Device`, `Camera`, `Stage`, `XYStage`, `Shutter`, `StateDevice`, `VolumetricPump`, `Hub`, and more
- **`PropertyMap`** — typed property storage with allowed-value constraints
- **`Transport`** — serial communication abstraction (`send_recv`, `send_bytes`, `receive_bytes`) + `MockTransport` for unit tests
- **Error types**, **`PropertyValue`**, **`DeviceType`**, **`FocusDirection`**

### `mm-core`

The `CMMCore` engine:

- **`DeviceManager`** — load/unload/dispatch to typed device handles
- **`AdapterRegistry`** — static registration via the `inventory` crate
- **`CircularBuffer`** — fixed-size ring buffer for image sequence acquisition
- **`Config`** / config-file load/save

### Adapters

41 adapter crates (as of this writing), all pure serial — no vendor SDKs required:

| Crate | Device(s) | Protocol |
|---|---|---|
| `mm-adapter-demo` | DemoCamera, DemoStage, DemoShutter | Simulated |
| `mm-adapter-arduino` | Arduino shutter/state | ASCII `\r` |
| `mm-adapter-asi-stage` | ASI XY + Z stage | `:A`/`:N` ASCII |
| `mm-adapter-asi-fw` | ASI filter wheel | `:A`/`:N` ASCII |
| `mm-adapter-asi-tiger` | ASI Tiger XY + Z stage | `:A`/`:N` ASCII, 115200 baud |
| `mm-adapter-asifw1000` | ASI FW1000 filter wheel + shutter | `\n\r` terminator |
| `mm-adapter-aladdin` | World Precision Instruments Aladdin pump | ASCII `\r` |
| `mm-adapter-carvii` | BD/CrEST CARVII confocal (shutter, filter wheels, sliders) | Single-char ASCII `\r` |
| `mm-adapter-chuoseiki` | ChuoSeiki MD-5000 XY stage | ASCII `\r` |
| `mm-adapter-cobolt` | Cobolt diode laser | ASCII `\r` |
| `mm-adapter-coherent-cube` | Coherent CUBE laser | ASCII `\r` |
| `mm-adapter-coherent-obis` | Coherent OBIS laser | ASCII `\r` |
| `mm-adapter-conix` | Conix filter cubes, XY + Z stage | `:A`/`:N` ASCII |
| `mm-adapter-coolled` | CoolLED pE-300 LED | CSS format |
| `mm-adapter-coolled-pe4000` | CoolLED pE-4000 LED (4-channel) | CSS format |
| `mm-adapter-corvus` | Corvus XY + Z stage | ASCII space-terminated |
| `mm-adapter-csuw1` | Yokogawa CSU-W1 spinning disk (shutter, filter wheel, dichroic) | CSV ASCII `\r` |
| `mm-adapter-elliptec` | Thorlabs Elliptec linear stage + 2-position slider | Hex-position `\r` |
| `mm-adapter-hamilton-mvp` | Hamilton MVP modular valve positioner | `0x06` ACK binary |
| `mm-adapter-ismatec` | Ismatec MCP peristaltic pump | Address-prefixed `*`-ACK |
| `mm-adapter-laser-quantum` | Laser Quantum Gem laser | ASCII `\r` |
| `mm-adapter-ldi` | 89 North LDI laser diode illuminator | ASCII `\n`, dynamic wavelengths |
| `mm-adapter-ludl` | Ludl BioPrecision XY + Z stage, filter wheel, shutter | `:A` ASCII |
| `mm-adapter-marzhauser` | Märzhäuser TANGO XY + Z stage | ASCII `\r` |
| `mm-adapter-neos` | Neos Technologies acousto-optic shutter | No-response serial |
| `mm-adapter-newport-stage` | Newport CONEX-CC / SMC100 Z stage | ASCII `\r\n` |
| `mm-adapter-niji` | BlueboxOptics niji 7-channel LED | Binary sync + `\r\n` |
| `mm-adapter-omicron` | Omicron PhoxX/LuxX/BrixX laser | `?CMD`/`!CMD` hex `\r` |
| `mm-adapter-oxxius` | Oxxius L6Cc laser combiner | ASCII `\r` |
| `mm-adapter-pecon` | Pecon TempControl 37-2 temperature + CO2 | Raw 3-byte BCD |
| `mm-adapter-precis-excite` | PrecisExcite LED illuminator | ASCII `\r` |
| `mm-adapter-prior` | Prior ProScan XY + Z stage, filter wheel, shutter | ASCII `\r` |
| `mm-adapter-sapphire` | Coherent Sapphire laser | ASCII `\r` |
| `mm-adapter-scientifica` | Scientifica XY + Z stage | ASCII `\r` |
| `mm-adapter-spectral-lmm5` | Spectral LMM5 laser combiner | Hex-encoded binary `\r` |
| `mm-adapter-sutter-lambda` | Sutter Lambda filter wheel | Binary |
| `mm-adapter-sutter-stage` | Sutter MP-285 XY + Z stage | `:A` ASCII |
| `mm-adapter-thorlabs-fw` | Thorlabs filter wheel | ASCII `\r` |
| `mm-adapter-varispec` | CRI VariSpec LCTF | ASCII `\r` |
| `mm-adapter-vincent` | Vincent Associates Uniblitz shutter | ASCII `\r` |
| `mm-adapter-vortran` | Vortran Stradus laser | ASCII `\r` |
| `mm-adapter-xcite` | Excelitas X-Cite arc lamp | ASCII `\r` |

## Building

```sh
cargo build --workspace
```

## Testing

```sh
cargo test --workspace
```

All adapters have unit tests that run against a `MockTransport` — no hardware required.

## Adding an Adapter

1. Create `adapters/mm-adapter-<name>/` with a `Cargo.toml` depending on `mm-device`.
2. Implement `Device` (and the appropriate device-type trait) for your struct.
3. Embed a `PropertyMap` and `Option<Box<dyn Transport>>`.
4. Add the crate to the workspace `Cargo.toml`.
5. Write tests using `MockTransport`.

Minimal example (`Cargo.toml`):

```toml
[package]
name = "mm-adapter-mydevice"
version = "0.1.0"
edition = "2021"

[dependencies]
mm-device = { path = "../../mm-device" }
```

Minimal struct pattern:

```rust
use mm_device::{error::MmResult, property::PropertyMap, traits::Device,
                transport::Transport, types::{DeviceType, PropertyValue}};

pub struct MyDevice {
    props: PropertyMap,
    transport: Option<Box<dyn Transport>>,
}

impl MyDevice {
    pub fn new() -> Self { /* define properties */ todo!() }
    pub fn with_transport(mut self, t: Box<dyn Transport>) -> Self {
        self.transport = Some(t); self
    }
}

impl Device for MyDevice {
    fn name(&self) -> &str { "MyDevice" }
    fn description(&self) -> &str { "My serial device" }
    fn initialize(&mut self) -> MmResult<()> { todo!() }
    fn shutdown(&mut self) -> MmResult<()> { Ok(()) }
    fn get_property(&self, name: &str) -> MmResult<PropertyValue> { self.props.get(name).cloned() }
    fn set_property(&mut self, name: &str, val: PropertyValue) -> MmResult<()> { self.props.set(name, val) }
    fn property_names(&self) -> Vec<String> { self.props.property_names().to_vec() }
    fn has_property(&self, name: &str) -> bool { self.props.has_property(name) }
    fn is_property_read_only(&self, name: &str) -> bool { false }
    fn device_type(&self) -> DeviceType { DeviceType::Generic }
    fn busy(&self) -> bool { false }
}
```
