import { Command } from 'commander';
import { loadConfigFile } from '../utils/config.js';
import { logger } from '../utils/logger.js';
import { formatValidationResult } from '../utils/formatter.js';
import type { ValidationResult } from '@entropic/types';

// Mock validator for now - will be replaced with actual WASM validator
function validateLocal(config: unknown): ValidationResult {
  // Basic validation - checking required fields
  const errors: ValidationResult['errors'] = [];
  const warnings: ValidationResult['warnings'] = [];

  if (!config || typeof config !== 'object') {
    errors.push({
      code: 'INVALID_CONFIG',
      field: 'root',
      message: 'Config must be an object',
      details: 'The configuration file is not valid JSON or is not an object',
      severity: 'error',
    });
  }

  return {
    is_valid: errors.length === 0,
    errors,
    warnings,
    suggestions: [],
  };
}

export const validateCommand = new Command('validate')
  .description('Validate Game DNA config')
  .argument('<file>', 'Config file path')
  .option('--fix', 'Auto-fix warnings')
  .action(async (file: string, options: { fix?: boolean }) => {
    try {
      const config = loadConfigFile(file);
      const result = validateLocal(config);

      console.log(formatValidationResult(result));

      if (!result.is_valid) {
        process.exit(1);
      }
    } catch (error) {
      logger.error((error as Error).message);
      process.exit(1);
    }
  });
