# WASM Validator Internals

## Overview

The WASM validator bridges Rust's `entropic-dna-core` library to JavaScript, enabling instant validation in both browser and Node.js without network calls.

## Architecture

```
┌─────────────────────────┐
│   JavaScript/TypeScript │
│  (web portal / CLI)     │
└───────────┬─────────────┘
            │
            │ JS API
            ▼
┌─────────────────────────┐
│  wasm-bindgen bindings  │
│  (packages/wasm/src/lib.rs) │
└───────────┬─────────────┘
            │
            │ Rust API
            ▼
┌─────────────────────────┐
│  entropic-dna-core      │
│  (validation engine)    │
└─────────────────────────┘
```

## Building WASM

### Prerequisites
```bash
cargo install wasm-pack
```

### Build
```bash
cd packages/wasm
./build.sh
```

Generates:
- `pkg/entropic_wasm_validator_bg.wasm` (WASM binary)
- `pkg/entropic_wasm_validator.js` (JS wrapper)
- `pkg/entropic_wasm_validator.d.ts` (TypeScript types)

### Build Configuration

`Cargo.toml`:
```toml
[profile.release]
opt-level = "s"  # Optimize for size
lto = true       # Link-time optimization
```

## JavaScript API

### validate_game_dna
```typescript
function validate_game_dna(json: string): ValidationResult
```

Validates a Game DNA JSON string.

### calculate_checksum
```typescript
function calculate_checksum(json: string): string
```

Calculates SHA-256 checksum.

### serialize_game_dna
```typescript
function serialize_game_dna(json: string): string
```

Deterministic JSON serialization.

## Performance

- **Cold start**: ~5ms (WASM load)
- **Validation**: <1ms for typical config
- **Size**: ~50KB gzipped WASM

## Browser Integration

```typescript
import * as wasm from '@entropic/wasm-validator'

const result = await wasm.validate_game_dna(JSON.stringify(config))
```

Runs synchronously in browser (no async needed after load).

## Node.js Integration

Works identically in Node.js:
```typescript
import * as wasm from '@entropic/wasm-validator'
const result = wasm.validate_game_dna(json)
```

## Debugging

### Build with debug symbols
```bash
wasm-pack build --dev
```

### View WASM in DevTools
Chrome/Edge: DevTools → Sources → WASM modules

### Log from Rust
```rust
web_sys::console::log_1(&format!("Debug: {:?}", value).into());
```

## Limitations

- No dynamic code execution
- No file I/O (use JS for that)
- Limited std library (no threading, no OS calls)

## Future Optimizations
- SIMD validation for large configs
- Incremental validation (only changed fields)
- Parallel validation of multiple configs
