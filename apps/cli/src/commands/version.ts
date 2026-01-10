import { Command } from 'commander';
import { logger } from '../utils/logger.js';

export const versionCommand = new Command('version')
  .description('Show version history for a config (local or remote)')
  .argument('<file>', 'Config file path')
  .option('--list', 'List all versions')
  .action(async (file: string, options: { list?: boolean }) => {
    logger.warn('Version history not yet implemented');
    logger.info(`File: ${file}`);
    if (options.list) {
      logger.info('No versions available (stub)');
    }
  });
