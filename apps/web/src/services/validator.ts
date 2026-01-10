import type { GameDNA, ValidationResult } from '@entropic/types';
import { validateGameDNA } from '@entropic/wasm-validator';

export async function validateConfigLocal(config: GameDNA): Promise<ValidationResult> {
  return validateGameDNA(config);
}
