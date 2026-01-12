import { Command } from 'commander';
import { logger } from '../utils/logger.js';

export const syncCommand = new Command('sync')
  .description('Sync config with remote server (push/pull)')
  .argument('<file>', 'Config file path')
  .option('--pull', 'Pull changes from remote')
  .option('--push', 'Push changes to remote')
  .action(async (file: string, options: { pull?: boolean; push?: boolean }) => {
    logger.warn('Sync not yet implemented');
    logger.info(`File: ${file}`);
    if (options.pull) {
      logger.info('Pull mode (stub)');
    }
    if (options.push) {
      logger.info('Push mode (stub)');
    }
  });
