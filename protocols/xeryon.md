# Xeryon Serial Communication Protocol

Extracted from the Xeryon C++ library source code.

## Transport Layer

- **Interface:** RS232/USB serial port
- **Baud rate:** Typically 115200 (supported: 110, 300, 600, 1200, 2400, 4800, 9600, 14400, 19200, 38400, 56000, 57600, 115200, 128000, 256000)
- **Data bits:** 8
- **Stop bits:** 1
- **Parity:** None
- **Line terminator:** `\n` (sent), `\r` or `\n` (received)
- **Max line length:** 64 bytes

## Message Format

### Sent (host to controller)

Single-axis systems:
```
TAG=VALUE\n
```

Multi-axis systems (axis letter prefix):
```
X:TAG=VALUE\n
```

Where `X` is the axis letter (e.g. `X`, `Y`, `Z`).

Some commands are sent without a value (raw string):
```
XLS1=312\n          (encoder resolution command, sent during init)
```

### Received (controller to host)

Single-axis systems:
```
TAG=VALUE\r\n
```

Multi-axis systems:
```
X:TAG=VALUE\r\n
```

Values are always integers (parsed with `atoi`).

## Commands (host to controller)

### Motion Commands (direct, not cached as settings)

| Tag    | Value        | Description                                     |
|--------|--------------|-------------------------------------------------|
| `DPOS` | encoder counts | Set desired position (absolute)                |
| `STOP` | 1            | Stop current movement                           |
| `INDX` | 0            | Find encoder index                              |
| `SCAN` | -1, 0, or 1  | Start scanning (direction) or stop (0)          |
| `FFRQ` | 0            | Find optimal frequency (calibration)            |
| `ZERO` | 0            | Zero the axis                                   |
| `RSET` | 0            | Reset the axis                                  |

Other direct command tags (not cached): `EPOS`, `HOME`, `STEP`, `MOVE`, `CONT`

### Settings Commands (cached and re-sent on reset)

| Tag    | Description                          | Unit in settings file | Multiplier applied         |
|--------|--------------------------------------|-----------------------|----------------------------|
| `SSPD` | Speed setpoint                       | mm/s                  | speed_multiplier (1000)    |
| `MSPD` | Max speed                            | mm/s                  | speed_multiplier (1000)    |
| `MAMP` | Max amplitude                        | raw                   | amplitude_multiplier (1456)|
| `AMPL` | Amplitude                            | raw                   | amplitude_multiplier (1456)|
| `MAM2` | Max amplitude 2                      | raw                   | amplitude_multiplier (1456)|
| `OSFA` | Offset A                             | raw                   | amplitude_multiplier (1456)|
| `OFSB` | Offset B                             | raw                   | amplitude_multiplier (1456)|
| `PHAC` | Phase AC                             | raw                   | phase_multiplier (182)     |
| `PHAS` | Phase                                | raw                   | phase_multiplier (182)     |
| `LLIM` | Left limit                           | mm                    | converted to encoder units |
| `RLIM` | Right limit                          | mm                    | converted to encoder units |
| `HLIM` | Home limit                           | mm                    | converted to encoder units |
| `ZON1` | Zone 1                               | mm                    | converted to encoder units |
| `ZON2` | Zone 2                               | mm                    | converted to encoder units |
| `POLI` | Polling interval                     | raw                   | none (stored as default)   |
| `PTOL` | Position tolerance                   | encoder counts        | none                       |
| `PTO2` | Position tolerance 2                 | encoder counts        | none                       |
| `MASS` | Payload mass (mapped to CFRQ)        | grams                 | see mapping below          |
| `LFRQ` | Low frequency limit (for calibration)| Hz                    | none                       |
| `HFRQ` | High frequency limit (for calibration)| Hz                   | none                       |
| `TOUT` | Timeout                              | minutes               | none                       |
| `TOU2` | Timeout 2                            | minutes               | none                       |

### Encoder Resolution Commands (sent during init)

Sent as raw strings (not TAG=VALUE format from the axis, but part of stage definition):

| Command      | Stage type                    |
|--------------|-------------------------------|
| `XLS1=312`   | XLS linear, 312.5 nm res      |
| `XLS1=1250`  | XLS linear, 1250 nm res       |
| `XLS1=78`    | XLS linear, 78.125 nm res     |
| `XLS1=5`     | XLS linear, 5 nm res          |
| `XLS1=1`     | XLS linear, 1 nm res          |
| `XLS3=...`   | XLS 3N (multi-axis) variants  |
| `XLA1=312`   | XLA linear, 312.5 nm res      |
| `XLA1=1250`  | XLA linear, 1250 nm res       |
| `XLA1=78`    | XLA linear, 78.125 nm res     |
| `XLA3=...`   | XLA 3N (multi-axis) variants  |
| `XRTA=109`   | XRTA rotation stage           |
| `XRT1=2`     | XRTU-40-3 rotation            |
| `XRT1=18`    | XRTU-40-19 rotation           |
| `XRT1=47`    | XRTU-40-49 rotation           |
| `XRT1=73`    | XRTU-40-73 rotation           |
| `XRT1=3`     | XRTU-30-3 rotation            |
| `XRT1=19`    | XRTU-30-19 rotation           |
| `XRT1=49`    | XRTU-30-49 rotation           |
| `XRT1=109`   | XRTU-30-109 rotation          |

## Responses (controller to host)

### Data Tags (real-time, updated continuously)

| Tag    | Description                          | Unit              |
|--------|--------------------------------------|-------------------|
| `EPOS` | Encoder position                     | encoder counts    |
| `DPOS` | Desired position                     | encoder counts    |
| `STAT` | Status register                      | bitmask (see below)|
| `FREQ` | Current operating frequency          | Hz                |
| `TIME` | Device timestamp                     | raw               |

### Metadata Tags (informational, not logged)

| Tag    | Description          |
|--------|----------------------|
| `SRNO` | Serial number        |
| `XLS ` | XLS stage identifier |
| `XRTU` | XRTU stage identifier|
| `XLA ` | XLA stage identifier |
| `XTRA` | Extra info           |
| `SOFT` | Software version     |
| `SYNC` | Sync marker          |

## Status Register (STAT)

The STAT value is a bitmask. Bit positions:

| Bit | Flag                        | Description                              |
|-----|-----------------------------|------------------------------------------|
| 4   | Force Zero                  | Force zero active                        |
| 5   | Motor On                    | Motor is energized                       |
| 6   | Closed Loop                 | Closed-loop control active               |
| 7   | Encoder at Index            | Encoder is at index position             |
| 8   | Encoder Valid               | Encoder reading is valid                 |
| 9   | Searching Index             | Currently searching for encoder index    |
| 10  | Position Reached            | Target position has been reached         |
| 11  | (unused)                    |                                          |
| 12  | Encoder Error               | Encoder error detected                   |
| 13  | Scanning                    | Continuous scan in progress              |
| 14  | At Left End                 | At left end stop                         |
| 15  | At Right End                | At right end stop                        |
| 16  | Error Limit                 | Error limit exceeded                     |
| 17  | Searching Optimal Frequency | Frequency calibration in progress        |

## MASS to CFRQ Mapping

When `MASS` is set in the settings file, it is converted to a `CFRQ` (resonance frequency) value:

| Mass (grams) | CFRQ value |
|--------------|------------|
| 0-50         | 100000     |
| 51-100       | 60000      |
| 101-250      | 30000      |
| 251-500      | 10000      |
| 501-1000     | 5000       |
| >1000        | 3000       |

## Position Units

Positions are exchanged in encoder counts. Conversion to physical units:

- **Linear stages:** `position_nm = encoder_counts * encoder_resolution`
  - Resolutions: 1 nm, 5 nm, 78.125 nm, 312.5 nm, 1250 nm (per count)
- **Rotation stages:** `position_rad = encoder_counts * (2 * PI * 1e6 / counts_per_rev)`
  - Counts per revolution: 57600, 86400, 144000, 360000, 1843200

## Position Tolerance Check

Position is considered reached when:
```
|DPOS| - PTO2 <= |EPOS| <= |DPOS| + PTO2
```
Where PTO2 defaults to 10 if not set. The STAT bit 10 (Position Reached) must also be set.

## Startup Sequence

1. Open serial port
2. Send `RSET=0` to each axis
3. Wait 200ms
4. Load settings from `settings_default.txt`
5. Send all cached settings to each axis
6. Send encoder resolution command for each axis (e.g. `XLS1=312`)

## Settings File Format

File: `settings_default.txt`

```
% This is a comment
SSPD=50         % Speed in mm/s (applied to first axis)
X:AMPL=10       % Amplitude for axis X
Y:AMPL=12       % Amplitude for axis Y
MASS=200        % Payload mass in grams (becomes CFRQ=30000)
POLI=200        % Polling interval
```

- Lines without `=` are ignored
- `%` starts a comment (rest of line ignored)
- Whitespace is stripped
- `X:` prefix targets a specific axis; without prefix, targets the first axis
- Multipliers are applied when reading from file (not when setting at runtime)
