import { Command } from 'commander';
import { existsSync, mkdirSync } from 'fs';
import { join } from 'path';
import { randomUUID } from 'crypto';

import { loadTemplate, TEMPLATES } from '../templates/index.js';
import { saveConfigFile } from '../utils/config.js';
import { logger } from '../utils/logger.js';

export const createCommand = new Command('create')
  .description('Create new game config from template')
  .argument('<name>', 'Game name')
  .option('--template <type>', `Template type (${Object.keys(TEMPLATES).join(', ')})`, 'casual')
  .option('--output <dir>', 'Output directory (defaults to <name>/)', '')
  .action(async (name: string, options: { template: string; output: string }) => {
    const outDir = options.output ? options.output : name;

    if (!existsSync(outDir)) {
      mkdirSync(outDir, { recursive: true });
    }

    const config = loadTemplate(options.template);
    config.id = randomUUID();
    config.name = name;

    const filePath = join(outDir, 'game.json');
    saveConfigFile(filePath, config);

    logger.success(`Created ${filePath}`);
    logger.info(`Run: entropic validate ${filePath}`);
  });
