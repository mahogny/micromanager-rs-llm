# SquidPlusFilterWheel Protocol Specification

8-position filter wheel for the Squid+ microscope, driven by a stepper motor
on the W axis of the Squid+ microcontroller.

Reference implementation: `squid-control/squid_control/hardware/filter_wheel.py`
and `squid-control/squid_control/hardware/microcontroller.py`

## Hardware

- **Motor:** Stepper motor, 200 full steps/revolution
- **Microstepping:** 64 microsteps/full step
- **Screw pitch:** 1.0 mm/revolution
- **Positions:** 8 (evenly spaced over one revolution)

## Transport Layer

- **Interface:** USB serial
- **Baud rate:** 2 000 000 (2 Mbps)
- **Data bits:** 8, **Stop bits:** 1, **Parity:** None
- **Protocol:** Binary packets (not text-based)

## Packet Format

### Command (Host → Controller) — 8 bytes

```
Byte  Field        Description
─────────────────────────────────────────────
 [0]  cmd_id       Rolling counter 0–255 (for ACK matching)
 [1]  cmd_code     Command identifier (see table below)
 [2]  payload[0]   Command-specific (MSB for multi-byte values)
 [3]  payload[1]
 [4]  payload[2]
 [5]  payload[3]   (LSB for multi-byte values)
 [6]  (reserved)   Zero
 [7]  crc8         CRC-8/CCITT of bytes [0..6]
```

### Response (Controller → Host) — 24 bytes

```
Byte   Field            Description
──────────────────────────────────────────────────────
 [0]   cmd_id           Echoes the command's cmd_id
 [1]   status           Execution status (see below)
 [2-5] x_pos            X axis position (signed 32-bit BE)
 [6-9] y_pos            Y axis position (signed 32-bit BE)
[10-13] z_pos           Z axis position (signed 32-bit BE)
[14-17] theta_pos       Theta axis position (signed 32-bit BE)
 [18]  switches         Button/switch state (bit flags)
[19-22] (reserved)
 [23]  crc8             CRC-8/CCITT of bytes [0..22]
```

### Execution Status Codes (response byte 1)

| Value | Name                  | Meaning                              |
|-------|-----------------------|--------------------------------------|
| 0x00  | COMPLETED             | Command finished successfully        |
| 0x01  | IN_PROGRESS           | Command still executing (poll again) |
| 0x02  | CHECKSUM_ERROR        | CRC mismatch — retransmit            |
| 0x03  | CMD_INVALID           | Unrecognised command                 |
| 0x04  | CMD_EXECUTION_ERROR   | Command failed during execution      |

## CRC-8 Algorithm

CRC-8/CCITT with polynomial 0x07, initial value 0x00, no final XOR.

```
crc = 0x00
for each byte b in data:
    crc = CRC8_TABLE[(crc ^ b) & 0xFF]
```

## Commands Used by the Filter Wheel

### MOVE_W (0x04) — Relative Move

Moves the W axis by a signed number of microsteps.

```
Byte [0]: cmd_id
Byte [1]: 0x04
Byte [2]: microsteps bits [31:24]  (signed 32-bit, big-endian)
Byte [3]: microsteps bits [23:16]
Byte [4]: microsteps bits [15:8]
Byte [5]: microsteps bits [7:0]
Byte [6]: 0x00
Byte [7]: CRC-8
```

### HOME_OR_ZERO (0x05) — Home W Axis

Homes the W axis using the hardware limit/index switch.

```
Byte [0]: cmd_id
Byte [1]: 0x05
Byte [2]: 0x05          (axis identifier: W = 5)
Byte [3]: 0x01          (direction: HOME_NEGATIVE)
Byte [4]: 0x00
Byte [5]: 0x00
Byte [6]: 0x00
Byte [7]: CRC-8
```

## Position Map

| Position | Distance from home | Microsteps from home |
|----------|--------------------|-----------------------|
| 0        | 0.000 mm           | 0                     |
| 1        | 0.125 mm           | 1 600                 |
| 2        | 0.250 mm           | 3 200                 |
| 3        | 0.375 mm           | 4 800                 |
| 4        | 0.500 mm           | 6 400                 |
| 5        | 0.625 mm           | 8 000                 |
| 6        | 0.750 mm           | 9 600                 |
| 7        | 0.875 mm           | 11 200                |

### Unit Conversions

```
microsteps_per_mm  = microstepping × fullsteps_per_rev / screw_pitch
                   = 64 × 200 / 1.0
                   = 12 800

step_mm            = screw_pitch / num_positions
                   = 1.0 / 8
                   = 0.125 mm

usteps_per_step    = step_mm × microsteps_per_mm
                   = 0.125 × 12 800
                   = 1 600
```

Movement between positions is always **relative**: to go from position *a* to
position *b*, send a MOVE_W with `(b − a) × 1600` microsteps.

## Initialization Sequence

| Step | Action                     | Command              | Notes                           |
|------|----------------------------|----------------------|---------------------------------|
| 1    | Home W axis                | HOME_OR_ZERO (0x05)  | Blocking — wait for COMPLETED   |
| 2    | Apply post-home offset     | MOVE_W (0x04), +102 µsteps | 0.008 mm × 12800 = 102.4 ≈ 102 |
| 3    | Set position to 0          | (internal state only) |                                 |

The home command has a longer timeout (15 s in the Python reference) compared
to normal moves.

## Position Change

To move from current position *a* to target position *b*:

1. Compute delta: `Δ = b − a`
2. Compute microsteps: `µsteps = Δ × 1600`
3. Send MOVE_W packet with the signed µsteps value
4. Wait for COMPLETED response

Same-position moves are skipped (no packet sent).

## Shutdown

No specific shutdown command. The driver simply releases the transport.

## Configuration Properties

| Property | Type   | Default   | Pre-init | Description        |
|----------|--------|-----------|----------|--------------------|
| Port     | String | Undefined | yes      | Serial port name   |

## Motor Parameters (Compile-Time Constants)

| Parameter       | Value | Unit              |
|-----------------|-------|-------------------|
| Screw pitch     | 1.0   | mm/revolution     |
| Microstepping   | 64    | µsteps/full step  |
| Full steps/rev  | 200   | steps/revolution  |
| Home offset     | 0.008 | mm                |
| Num positions   | 8     |                   |
