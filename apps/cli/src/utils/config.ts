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

/**
 * Load the global CLI configuration from the user's config file.
 *
 * @returns The parsed `CLIConfig` from the global config file (`~/.entropic/config.json`); an empty object if the file does not exist.
 */
export function loadConfig(): CLIConfig {
  if (!existsSync(CONFIG_FILE)) {
    return {};
  }
  return JSON.parse(readFileSync(CONFIG_FILE, 'utf-8'));
}

/**
 * Persist CLI configuration to the user's global config file.
 *
 * Ensures the config directory exists and writes the provided configuration as formatted JSON to the global CLI config file (`~/.entropic/config.json`).
 *
 * @param config - Configuration values to persist
 */
export function saveConfig(config: CLIConfig): void {
  if (!existsSync(CONFIG_DIR)) {
    mkdirSync(CONFIG_DIR, { recursive: true });
  }
  writeFileSync(CONFIG_FILE, JSON.stringify(config, null, 2));
}

/**
 * Load and parse a GameDNA JSON file from disk.
 *
 * @param filePath - Path to the JSON file containing the GameDNA
 * @returns The parsed `GameDNA` object
 * @throws Error if the file at `filePath` does not exist
 */
export function loadConfigFile(filePath: string): GameDNA {
  if (!existsSync(filePath)) {
    throw new Error(`Config file not found: ${filePath}`);
  }
  const content = readFileSync(filePath, 'utf-8');
  return JSON.parse(content);
}

/**
 * Write a GameDNA object as pretty-printed JSON to the specified filesystem path, creating parent directories if necessary.
 *
 * @param filePath - Destination file path where the config will be written
 * @param config - The GameDNA object to serialize and save
 */
export function saveConfigFile(filePath: string, config: GameDNA): void {
  const dir = filePath.substring(0, filePath.lastIndexOf('/'));
  if (dir && !existsSync(dir)) {
    mkdirSync(dir, { recursive: true });
  }
  writeFileSync(filePath, JSON.stringify(config, null, 2));
}