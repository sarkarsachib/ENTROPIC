import { GameDNAServiceClient } from '@entropic/api-client';
import type { GameDNA, ValidationResult, LockedGameDNA, VersionInfo } from '@entropic/types';

const API_URL = import.meta.env.VITE_API_URL ?? 'http://localhost:8080';

const client = new GameDNAServiceClient({ baseUrl: API_URL });

export const api = {
  createGameDNA: (config: GameDNA): Promise<GameDNA> => client.createGameDNA(config),

  getGameDNA: (id: string): Promise<GameDNA> => client.getGameDNA(id),

  listGameDNA: (): Promise<GameDNA[]> => client.listGameDNA(),

  updateGameDNA: (id: string, config: GameDNA): Promise<GameDNA> => client.updateGameDNA(id, config),

  validateGameDNA: (config: GameDNA): Promise<ValidationResult> => client.validateGameDNA(config),

  publishGameDNA: (id: string): Promise<LockedGameDNA> => client.publishGameDNA(id),

  getVersionHistory: (id: string): Promise<VersionInfo[]> => client.getVersionHistory(id),

  rollbackToVersion: (id: string, version: number): Promise<GameDNA> => client.rollbackToVersion(id, version),
};
