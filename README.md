# Entropic Dev Portal

A comprehensive developer interface for Game DNA management - web portal, CLI tool, and WASM validation.

## 🎯 Overview

This monorepo provides the complete human interface layer for ENTROPIC Game DNA:

- **Web Portal**: Visual editor for creating and managing game configurations
- **CLI Tool**: Command-line interface for local development workflows
- **WASM Validator**: Instant browser/Node.js validation without network calls

## 🚀 Quick Start

### Prerequisites
- Node.js 20+
- pnpm 8+
- Rust 1.70+ (for WASM building)

### Install Dependencies
```bash
pnpm install
```

### Run Web Portal
```bash
pnpm --filter @entropic/web dev
```
Open http://localhost:3000

### Run CLI
```bash
pnpm --filter @entropic/cli dev create test-game --template fps
```

### Build Everything
```bash
pnpm build
```

## 📦 Packages

### Apps
- **apps/web**: React web portal with visual config editor
- **apps/cli**: Node.js CLI tool for local workflows

### Packages
- **packages/types**: Shared TypeScript types (auto-generated from Rust)
- **packages/api-client**: gRPC-web client library
- **packages/wasm**: Rust→WASM validator bridge

## 🧪 Testing

```bash
# Run all tests
pnpm test

# Test specific package
pnpm --filter @entropic/cli test
```

## 📚 Documentation

- [Portal User Guide](docs/PORTAL.md)
- [CLI Reference](docs/CLI.md)
- [WASM Internals](docs/WASM.md)

## 🏗️ Architecture

```
┌─────────────────────────────────────────────┐
│           Web Portal (React)                │
│  ┌──────────────┐    ┌──────────────┐      │
│  │ Config Editor│    │  Validator   │      │
│  └──────────────┘    └──────────────┘      │
└─────────────┬───────────────┬───────────────┘
              │               │
        ┌─────▼───────┐ ┌─────▼──────┐
        │ WASM        │ │ API Client │
        │ Validator   │ │ (gRPC-web) │
        └─────────────┘ └─────┬──────┘
              │               │
        ┌─────▼───────────────▼───────┐
        │  entropic-dna-core (Rust)   │
        │  Validation Engine           │
        └─────────────────────────────┘
```

## 🛠️ Development Workflow

### 1. Create a Game Config
```bash
# Using CLI
pnpm --filter @entropic/cli dev create my-game --template rpg

# Or use web portal at localhost:3000
```

### 2. Validate Locally
```bash
# CLI validation (instant, offline)
pnpm --filter @entropic/cli dev validate my-game/game.json

# Or paste JSON in web portal validator
```

### 3. Build for Distribution
```bash
pnpm --filter @entropic/cli dev build my-game/game.json
```

### 4. Publish (requires API server)
```bash
pnpm --filter @entropic/cli dev publish my-game/game.json --server https://api.entropic.dev
```

## 🐳 Docker

### Build Images
```bash
docker-compose -f docker/docker-compose.yml build
```

### Run Portal
```bash
docker-compose -f docker/docker-compose.yml up portal
```

## 🎮 Features

### Web Portal
✅ Visual config editor  
✅ Real-time validation (WASM)  
✅ Live JSON preview  
✅ Offline support (IndexedDB)  
✅ Dark mode  
✅ Responsive design  

### CLI Tool
✅ Create from templates  
✅ Local validation  
✅ Build for distribution  
🚧 Publish to server (stub)  
🚧 Version management (stub)  
🚧 Remote sync (stub)  

### WASM Validator
✅ Browser validation  
✅ Node.js validation  
✅ <1ms validation time  
✅ ~50KB gzipped  

## 🔧 Tech Stack

- **Frontend**: React 18, TypeScript, Vite, Tailwind CSS
- **CLI**: Node.js, Commander.js, Chalk
- **State**: Zustand, TanStack Query
- **Storage**: IndexedDB (local), gRPC-web (remote)
- **Validation**: Rust + WebAssembly
- **Monorepo**: pnpm workspaces + Turbo

## 📝 Scripts

```bash
# Development
pnpm dev                 # Run all dev servers
pnpm build              # Build all packages

# Web portal
pnpm --filter @entropic/web dev      # Dev server
pnpm --filter @entropic/web build    # Production build

# CLI
pnpm --filter @entropic/cli dev      # Run CLI in dev mode
pnpm --filter @entropic/cli build    # Build CLI

# WASM (requires Rust)
cd packages/wasm && ./build.sh       # Build WASM validator

# Testing
pnpm test               # Run all tests
pnpm test:watch         # Watch mode

# Code quality
pnpm lint              # Lint all packages
pnpm format            # Format code
pnpm typecheck         # Type check
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `pnpm test`
5. Submit a pull request

## 📄 License

MIT or Apache-2.0 (your choice)

## 🔗 Related Projects

- [entropic-dna-core](../entropic-dna-core/): Core Game DNA library (Rust)
- [entropic-engine](https://github.com/entropic-engine/core): Main game engine

## 📞 Support

- 📧 Email: support@entropic.engine
- 💬 Discord: [Entropic Engine Community](https://discord.gg/entropic)
- 🐛 Issues: [GitHub Issues](https://github.com/entropic-engine/entropic-dev-portal/issues)
