import type { GameDNA, ValidationResult } from '@entropic/types';
import { validateGameDNA } from '@entropic/wasm-validator';

/**
 * Validate a GameDNA configuration and produce a validation outcome.
 *
 * @param config - The GameDNA configuration to validate
 * @returns The validation result for `config`, including any errors or warnings found
 */
export async function validateConfigLocal(config: GameDNA): Promise<ValidationResult> {
  return validateGameDNA(config);
}