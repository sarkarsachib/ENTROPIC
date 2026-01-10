#!/usr/bin/env node

import { Command } from 'commander';
import { createCommand } from './commands/create.js';
import { validateCommand } from './commands/validate.js';
import { buildCommand } from './commands/build.js';
import { publishCommand } from './commands/publish.js';
import { versionCommand } from './commands/version.js';
import { syncCommand } from './commands/sync.js';

const program = new Command();

program
  .name('entropic')
  .description('Game DNA CLI tool for the ENTROPIC game engine')
  .version('1.0.0');

program.addCommand(createCommand);
program.addCommand(validateCommand);
program.addCommand(buildCommand);
program.addCommand(publishCommand);
program.addCommand(versionCommand);
program.addCommand(syncCommand);

program.parse();
