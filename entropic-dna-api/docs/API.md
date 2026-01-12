# Entropic Game DNA API

This document describes the Entropic DNA API server, which exposes the Game DNA engine via **gRPC** and **REST**.

## Overview

The service provides:

- CRUD operations for GameDNA configurations
- Deterministic validation (Rust engine via FFI when enabled; Go fallback otherwise)
- PostgreSQL persistence with version history
- Publish/lock and rollback workflows

## Service Endpoints

### gRPC

Service: `entropic.dna.v1.GameDNAService`

Methods:

- `CreateGameDNA`
- `GetGameDNA`
- `ListGameDNA`
- `UpdateGameDNA`
- `DeleteGameDNA`
- `ValidateGameDNA`
- `PublishGameDNA`
- `GetVersionHistory`
- `RollbackToVersion`
- `CloneGameDNA`

### REST (grpc-gateway)

Base path: `/api/v1`

| REST Endpoint | Method | RPC |
|---|---:|---|
| `/api/v1/game-dna` | POST | CreateGameDNA |
| `/api/v1/game-dna/{id}` | GET | GetGameDNA |
| `/api/v1/game-dna` | GET | ListGameDNA |
| `/api/v1/game-dna/{id}` | PUT | UpdateGameDNA |
| `/api/v1/game-dna/{id}` | DELETE | DeleteGameDNA |
| `/api/v1/game-dna/validate` | POST | ValidateGameDNA |
| `/api/v1/game-dna/{id}/publish` | POST | PublishGameDNA |
| `/api/v1/game-dna/{config_id}/versions` | GET | GetVersionHistory |
| `/api/v1/game-dna/{config_id}/rollback` | POST | RollbackToVersion |
| `/api/v1/game-dna/{id}/clone` | POST | CloneGameDNA |

## Example Usage

### Create a GameDNA (REST)

```bash
curl -X POST http://localhost:8080/api/v1/game-dna \
  -H 'Content-Type: application/json' \
  -d '{
    "gameDna": {
      "name": "My Game",
      "version": "0.1.0",
      "genre": "FPS",
      "camera": "Perspective3D",
      "tone": "Cinematic",
      "worldScale": "OpenWorld",
      "targetPlatforms": ["PC"],
      "targetFps": 60,
      "timeScale": 1.0
    }
  }'
```

### Get version history

```bash
curl http://localhost:8080/api/v1/game-dna/<id>/versions
```

### Rollback

```bash
curl -X POST http://localhost:8080/api/v1/game-dna/<id>/rollback \
  -H 'Content-Type: application/json' \
  -d '{"versionNum": 1}'
```

## OpenAPI

OpenAPI output is generated via buf + grpc-gateway and placed under:

- `gen/openapi/`

## Errors

- Validation errors return an error response (gRPC `InvalidArgument` in future versions).
- Missing records return a not-found error.

## Configuration

Environment variables:

- `DATABASE_URL`
- `GRPC_PORT`
- `HTTP_PORT`
- `SERVER_HOST`
- `RUST_LIB_PATH`
- `RUST_ENABLED`
- `LOG_LEVEL`
- `LOG_FORMAT`
