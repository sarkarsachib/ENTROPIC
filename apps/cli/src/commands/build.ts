import { Command } from 'commander';
import { readFileSync, writeFileSync } from 'fs';
import { basename } from 'path';
import { createHash } from 'crypto';

import { logger } from '../utils/logger.js';

export const buildCommand = new Command('build')
  .description('Build config for distribution (compile to .dna bundle)')
  .argument('<file>', 'Config file path')
  .option('--output <file>', 'Output file path')
  .action(async (file: string, options: { output?: string }) => {
    try {
      const json = readFileSync(file, 'utf-8');
      const checksum = createHash('sha256').update(json).digest('hex');

      const bundle = {
        format: 'entropic.dna.bundle.v1',
        checksum,
        created_at: new Date().toISOString(),
        source_file: basename(file),
        config: JSON.parse(json),
      };

      const outputPath = options.output ?? file.replace(/\.json$/i, '.dna.json');
      writeFileSync(outputPath, JSON.stringify(bundle, null, 2));

      logger.success(`Built ${outputPath}`);
      logger.info(`Checksum: ${checksum}`);
    } catch (error) {
      logger.error((error as Error).message);
      process.exit(1);
    }
  });
