import { readFileSync, writeFileSync, existsSync, mkdirSync } from 'fs';
import { homedir } from 'os';
import { join } from 'path';
import type { GameDNA } from '@entropic/types';

const CONFIG_DIR = join(homedir(), '.entropic');
const CONFIG_FILE = join(CONFIG_DIR, 'config.json');

export interface CLIConfig {
  serverUrl?: string;
  apiKey?: string;
  defaultTemplate?: string;
}

export function loadConfig(): CLIConfig {
  if (!existsSync(CONFIG_FILE)) {
    return {};
  }
  return JSON.parse(readFileSync(CONFIG_FILE, 'utf-8'));
}

export function saveConfig(config: CLIConfig): void {
  if (!existsSync(CONFIG_DIR)) {
    mkdirSync(CONFIG_DIR, { recursive: true });
  }
  writeFileSync(CONFIG_FILE, JSON.stringify(config, null, 2));
}

export function loadConfigFile(filePath: string): GameDNA {
  if (!existsSync(filePath)) {
    throw new Error(`Config file not found: ${filePath}`);
  }
  const content = readFileSync(filePath, 'utf-8');
  return JSON.parse(content);
}

export function saveConfigFile(filePath: string, config: GameDNA): void {
  const dir = filePath.substring(0, filePath.lastIndexOf('/'));
  if (dir && !existsSync(dir)) {
    mkdirSync(dir, { recursive: true });
  }
  writeFileSync(filePath, JSON.stringify(config, null, 2));
}
