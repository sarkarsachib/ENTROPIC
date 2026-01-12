import { Command } from 'commander';
import { loadConfigFile, loadConfig } from '../utils/config.js';
import { logger } from '../utils/logger.js';

export const publishCommand = new Command('publish')
  .description('Publish config to remote server')
  .argument('<file>', 'Config file path')
  .option('--server <url>', 'Server URL')
  .action(async (file: string, options: { server?: string }) => {
    try {
      const config = loadConfigFile(file);
      const cliConfig = loadConfig();
      const serverUrl = options.server ?? cliConfig.serverUrl ?? 'https://api.entropic.dev';

      logger.info(`Publishing to ${serverUrl}...`);

      // TODO: Implement gRPC-web client
      logger.warn('Publishing not yet implemented - requires gRPC-web client');
      logger.info(`Config ID: ${config.id}`);
      logger.info(`Config Name: ${config.name}`);
    } catch (error) {
      logger.error((error as Error).message);
      process.exit(1);
    }
  });
