# Entropic CLI Reference

## Installation

### Development
```bash
pnpm install
pnpm build:cli
```

### Global Install (future)
```bash
npm install -g @entropic/cli
```

## Commands

### `entropic create <name>`
Create a new game configuration from a template.

```bash
entropic create my-game --template fps
```

Options:
- `--template <type>`: Template type (fps, rpg, casual, strategy)
- `--output <dir>`: Output directory (defaults to `<name>/`)

Output:
- Creates `<output>/game.json`

### `entropic validate <file>`
Validate a local Game DNA JSON file.

```bash
entropic validate game.json
```

Options:
- `--fix`: Auto-fix warnings (not implemented)

Exit codes:
- `0`: Valid configuration
- `1`: Validation errors or file not found

### `entropic build <file>`
Build a config for distribution.

```bash
entropic build game.json --output game.dna.json
```

Creates a bundle with:
- Config JSON
- SHA-256 checksum
- Metadata

### `entropic publish <file>`
Publish a config to remote server.

```bash
entropic publish game.json --server https://api.entropic.dev
```

Flow:
1. Validate config
2. Publish via API
3. Lock config (immutable)
4. Return checksum/hash

(Currently stubbed)

### `entropic version <file>`
Show version history.

```bash
entropic version game.json --list
```

(Currently stubbed)

### `entropic sync <file>`
Sync with remote server.

```bash
entropic sync game.json --pull
entropic sync game.json --push
```

(Currently stubbed)

## Templates

Available templates:
- `fps`: Competitive FPS
- `rpg`: Open world RPG
- `casual`: Mobile-first casual
- `strategy`: RTS/strategy

## Configuration File Format

The CLI works with the canonical Game DNA JSON schema.

Key fields:
- `id`: UUID
- `name`: Game name
- `version`: Semantic version object
- `genre`: Game genre
- `camera`: Camera mode
- ... and 30+ more

## Troubleshooting

### Validation fails
- Ensure JSON is valid
- Check required fields are present
- Use `entropic validate` to see detailed errors

### Publish fails
- Check server URL
- Ensure API key configured (future)

## Future Features
- Full WASM validator integration
- gRPC-web publishing
- Interactive wizard (`entropic init`)
- Auto-fix warnings
- Remote sync
