# Entropic Dev Portal User Guide

## Overview

The Entropic Dev Portal is a comprehensive web application for creating, editing, and managing Game DNA configurations. It provides a visual interface for defining all aspects of your game's configuration.

## Features

### Dashboard
- **Overview**: View all your game configurations at a glance
- **Quick Stats**: See total configs, recent edits, and published configs
- **Search & Filter**: Find configs by genre, platform, or name
- **Create New**: Start a new configuration with a single click

### Config Editor
- **Visual Form**: Edit all 30+ Game DNA fields with intuitive inputs
- **Real-time Validation**: Instant feedback via WASM validator
- **Live Preview**: See JSON output as you edit
- **Auto-save**: Drafts automatically save to IndexedDB
- **Conflict Detection**: Get suggestions for incompatible combinations

### Validator
- **Paste & Validate**: Test any Game DNA JSON instantly
- **Error Details**: See exactly what's wrong and how to fix it
- **Export Fixed Config**: Download corrected configuration

### Version History
- **Timeline View**: See all versions of a configuration
- **Diff Viewer**: Compare versions side-by-side
- **Rollback**: Restore any previous version
- **Change Log**: Track what changed and when

## Getting Started

### 1. Create a New Config
1. Click "Create New Config" on the dashboard
2. Fill in basic information (name, genre, camera)
3. Configure advanced options as needed
4. Save the configuration

### 2. Edit Fields
- **Genre**: Choose from FPS, RPG, Strategy, etc.
- **Camera**: Select 2D, 3D, Isometric, VR
- **Tone**: Realistic, Arcade, Cinematic, Stylized
- **Platforms**: Check all target platforms (Mobile, PC, Console, XR)
- **Physics**: Arcade, Semi-Realistic, Realistic
- **Monetization**: Free-to-Play, Premium, Subscription

### 3. Validation
The editor validates your configuration in real-time:
- **Errors** (red): Must be fixed before publishing
- **Warnings** (yellow): Suggestions for improvement
- **Suggestions** (blue): Optional optimizations

Common validations:
- FPS + Isometric camera = Conflict (suggest Perspective3D)
- Galaxy scale + Mobile platform = Conflict (suggest PC/Console)
- Realistic tone + Arcade physics = Warning

### 4. Save & Publish
- **Save**: Store locally in IndexedDB (offline-capable)
- **Publish**: Lock configuration and push to server

## Keyboard Shortcuts
- `Ctrl+S` / `Cmd+S`: Save configuration
- `Ctrl+Enter` / `Cmd+Enter`: Publish configuration
- `Ctrl+Z` / `Cmd+Z`: Undo last change (when implemented)

## Offline Support
The portal works offline:
- Drafts save to IndexedDB automatically
- WASM validator runs locally (no network needed)
- Sync to server when connection restored

## Tips
1. Start with a template (FPS, RPG, Casual, Strategy)
2. Use the live preview to check JSON output
3. Fix all errors before publishing
4. Use version history to track changes over time

## API Server
The portal connects to an API server for:
- Publishing locked configurations
- Fetching version history
- Syncing across devices

Set the API URL via environment variable:
```bash
VITE_API_URL=https://api.entropic.dev
```

## Browser Compatibility
- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+

WebAssembly and IndexedDB required.
