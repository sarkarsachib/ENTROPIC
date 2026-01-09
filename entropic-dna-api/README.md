# Entropic DNA API Server

Production-grade API server in Go that exposes the Rust Game DNA engine via gRPC and REST, enabling distributed access to game configuration management, validation, and storage with versioning and rollback capabilities.

## Features

- ✅ **gRPC Service** - Full CRUD operations for GameDNA configurations
- ✅ **REST Gateway** - Auto-generated REST endpoints via grpc-gateway
- ✅ **PostgreSQL Storage** - Persistent storage with SQL migrations
- ✅ **In-Memory Fallback** - Development-friendly fallback storage
- ✅ **Rust FFI Bindings** - Optional integration with Rust validation engine
- ✅ **Version History** - Automatic versioning of all configurations
- ✅ **Rollback Support** - Revert to any previous version
- ✅ **Publish/Lock** - Immutable snapshots for production
- ✅ **Clone Configurations** - Duplicate existing configs
- ✅ **Structured Logging** - Production-ready logging with Zap
- ✅ **Docker Support** - Fully containerized deployment

## Quick Start

### Prerequisites

- Go 1.22+
- PostgreSQL 15+ (optional, will fall back to in-memory)
- Docker & Docker Compose (optional)

### Local Development

1. **Install dependencies**:
```bash
make deps
make install-tools
```

2. **Generate protobuf code**:
```bash
make proto
```

3. **Run the server**:
```bash
export DATABASE_URL="postgres://entropic:entropic@localhost:5432/game_dna?sslmode=disable"
make run
```

The server will start:
- gRPC: `localhost:50051`
- REST: `http://localhost:8080`

### Docker Deployment

```bash
docker-compose up -d
```

This starts:
- PostgreSQL on `localhost:5432`
- API server gRPC on `localhost:50051`
- API server REST on `http://localhost:8080`

## API Documentation

See [docs/API.md](docs/API.md) for detailed API documentation.

### Example: Create a GameDNA

```bash
curl -X POST http://localhost:8080/api/v1/game-dna \
  -H 'Content-Type: application/json' \
  -d '{
    "gameDna": {
      "name": "My FPS Game",
      "genre": "FPS",
      "camera": "Perspective3D",
      "targetPlatforms": ["PC"],
      "targetFps": 60,
      "timeScale": 1.0
    }
  }'
```

### Example: List GameDNAs

```bash
curl http://localhost:8080/api/v1/game-dna?page=1&pageSize=10
```

## Configuration

Configuration can be provided via:
1. Environment variables
2. Config file (set `CONFIG_FILE` env var)
3. Defaults in code

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | memory |
| `GRPC_PORT` | gRPC server port | 50051 |
| `HTTP_PORT` | REST server port | 8080 |
| `SERVER_HOST` | Server bind address | 0.0.0.0 |
| `RUST_LIB_PATH` | Path to Rust library | ./lib/libentropic_dna_core.so |
| `RUST_ENABLED` | Enable Rust validation | false |
| `LOG_LEVEL` | Log level (debug/info/warn/error) | info |
| `LOG_FORMAT` | Log format (json/console) | console |

## Project Structure

```
entropic-dna-api/
├── cmd/
│   └── server/          # Server entry point
├── internal/
│   ├── api/             # gRPC & REST implementations
│   ├── config/          # Configuration management
│   ├── ffi/             # Rust FFI bindings
│   ├── models/          # Data models
│   └── storage/         # Storage implementations
│       ├── memory.go    # In-memory storage
│       ├── postgres.go  # PostgreSQL storage
│       └── migrations/  # SQL migrations
├── proto/               # Protobuf definitions
│   └── entropic/dna/v1/
├── gen/                 # Generated code
│   ├── proto/           # Generated Go code
│   └── openapi/         # Generated OpenAPI docs
├── docs/                # Documentation
├── docker/              # Docker files
└── tests/               # Tests
```

## Development

### Running Tests

```bash
make test
```

### Building

```bash
make build
```

The binary will be placed in `bin/server`.

### Generating Protobuf Code

```bash
make proto
```

## Rust FFI Integration

To enable Rust validation:

1. Build the Rust library:
```bash
cd ../entropic-dna-core
cargo build --release
```

2. Copy the library:
```bash
cp target/release/libentropic_dna_core.so ../entropic-dna-api/lib/
```

3. Enable Rust validation:
```bash
export RUST_ENABLED=true
export RUST_LIB_PATH=./lib/libentropic_dna_core.so
```

## Deployment

### Docker

Build the image:
```bash
make docker-build
```

Run with docker-compose:
```bash
docker-compose up -d
```

### Production Considerations

- Set `LOG_FORMAT=json` for structured logging
- Use `DATABASE_URL` to connect to a managed PostgreSQL instance
- Enable `RUST_ENABLED=true` for deterministic validation
- Configure connection pooling via `DATABASE_MAX_CONNECTIONS`
- Use SSL mode for database connections in production

## License

MIT OR Apache-2.0
