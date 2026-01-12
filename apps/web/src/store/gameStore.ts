import { create } from 'zustand';
import type { GameDNA, ValidationError } from '@entropic/types';

interface GameStore {
  currentConfig: GameDNA | null;
  isLoading: boolean;
  errors: ValidationError[];
  warnings: ValidationError[];
  isDirty: boolean;

  // Actions
  setCurrentConfig: (config: GameDNA | null) => void;
  setLoading: (loading: boolean) => void;
  setErrors: (errors: ValidationError[]) => void;
  setWarnings: (warnings: ValidationError[]) => void;
  setDirty: (dirty: boolean) => void;
  resetForm: () => void;
}

export const useGameStore = create<GameStore>((set) => ({
  currentConfig: null,
  isLoading: false,
  errors: [],
  warnings: [],
  isDirty: false,

  setCurrentConfig: (config) => set({ currentConfig: config, isDirty: true }),
  setLoading: (loading) => set({ isLoading: loading }),
  setErrors: (errors) => set({ errors }),
  setWarnings: (warnings) => set({ warnings }),
  setDirty: (dirty) => set({ isDirty: dirty }),
  resetForm: () =>
    set({
      currentConfig: null,
      isLoading: false,
      errors: [],
      warnings: [],
      isDirty: false,
    }),
}));
