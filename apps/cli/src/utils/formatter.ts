import chalk from 'chalk';
import type { ValidationResult } from '@entropic/types';

export function formatValidationResult(result: ValidationResult): string {
  const lines: string[] = [];

  if (result.errors.length > 0) {
    lines.push(chalk.red('❌ Validation failed:'));
    for (const err of result.errors) {
      lines.push(chalk.red(`  [${err.code}] ${err.message}`));
      if (err.details) {
        lines.push(chalk.gray(`  → ${err.details}`));
      }
    }
  }

  if (result.warnings.length > 0) {
    lines.push(chalk.yellow('⚠️  Warnings:'));
    for (const warn of result.warnings) {
      lines.push(chalk.yellow(`  [${warn.code}] ${warn.message}`));
      if (warn.details) {
        lines.push(chalk.gray(`  → ${warn.details}`));
      }
    }
  }

  if (result.is_valid) {
    lines.push(chalk.green('✓ Config is valid'));
  }

  return lines.join('\n');
}
