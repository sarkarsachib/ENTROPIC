import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { mkdtempSync, rmSync, existsSync, readFileSync } from 'fs';
import { tmpdir } from 'os';
import { join } from 'path';

import { loadTemplate } from '../templates/index.js';
import { saveConfigFile } from '../utils/config.js';

describe('CLI templates', () => {
  let testDir: string;

  beforeEach(() => {
    testDir = mkdtempSync(join(tmpdir(), 'entropic-cli-test-'));
  });

  afterEach(() => {
    rmSync(testDir, { recursive: true, force: true });
  });

  it('should create a new config from template', () => {
    const config = loadTemplate('fps');
    config.name = 'test-game';

    const filePath = join(testDir, 'game.json');
    saveConfigFile(filePath, config);

    expect(existsSync(filePath)).toBe(true);

    const saved = JSON.parse(readFileSync(filePath, 'utf-8'));
    expect(saved.name).toBe('test-game');
    expect(saved.genre).toBe('FPS');
  });
});
