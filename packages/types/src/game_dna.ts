/**
 * Shared Game DNA TypeScript types.
 *
 * This package is intended to be generated from the canonical schema (proto/Rust).
 * For now, it mirrors the Rust `entropic-dna-core` schema closely enough for the
 * dev portal + CLI.
 */

export type Genre =
  | 'FPS'
  | 'RPG'
  | 'TPS'
  | 'Strategy'
  | 'Casual'
  | 'Horror'
  | 'Racing'
  | 'Simulation'
  | 'Puzzle'
  | 'Educational'
  | { CustomGenre: string };

export type CameraMode =
  | 'Perspective2D'
  | 'Perspective2_5D'
  | 'Perspective3D'
  | 'Isometric'
  | 'VR'
  | { CustomCamera: string };

export type Tone =
  | 'Realistic'
  | 'Arcade'
  | 'Cinematic'
  | 'Stylized'
  | 'Minimalist'
  | { CustomTone: string };

export type WorldScale =
  | 'TinyLevel'
  | 'SmallLevel'
  | 'MediumLevel'
  | 'LargeLevel'
  | 'OpenWorld'
  | 'Planet'
  | 'Galaxy'
  | { CustomScale: string };

export type TargetPlatform = 'Mobile' | 'PC' | 'Console' | 'XR' | 'CloudStreamed' | 'MultiPlatform';

export type MonetizationModel =
  | 'FreeToPlay'
  | 'PremiumBuy'
  | 'Subscription'
  | 'OneTimePay'
  | 'Hybrid'
  | { Custom: string };

export type PhysicsProfile = 'Arcade' | 'SemiRealistic' | 'Realistic' | { CustomPhysics: string };

export type DifficultyMode = 'Easy' | 'Medium' | 'Hard' | 'Dynamic' | { CustomDifficulty: string };

export interface SemanticVersion {
  major: number;
  minor: number;
  patch: number;
}

export interface GameDNA {
  // Identity
  id: string;
  name: string;
  version: SemanticVersion;

  // Core Configuration
  genre: Genre;
  camera: CameraMode;
  tone: Tone;
  world_scale: WorldScale;
  target_platforms: TargetPlatform[];

  // Gameplay
  physics_profile: PhysicsProfile;
  max_players: number;
  is_competitive: boolean;
  supports_coop: boolean;
  difficulty: DifficultyMode;

  // Business
  monetization: MonetizationModel;
  target_audience: string;
  esrb_rating: string | null;

  // Performance
  target_fps: number;
  max_draw_distance: number;
  max_entities: number;
  max_npc_count: number;

  // World Simulation
  time_scale: number;
  weather_enabled: boolean;
  seasons_enabled: boolean;
  day_night_cycle: boolean;
  persistent_world: boolean;

  // AI & NPCs
  npc_count: number;
  ai_enabled: boolean;
  ai_difficulty_scaling: boolean;

  // Narrative
  has_campaign: boolean;
  has_side_quests: boolean;
  dynamic_quests: boolean;

  // Metadata
  tags: string[];
  custom_properties: Record<string, string>;
}

export interface ValidationError {
  code: string;
  field: string;
  message: string;
  details: string;
}

export interface ValidationWarning {
  code: string;
  field: string;
  message: string;
  suggestion: string;
}

export interface ValidationResult {
  is_valid: boolean;
  errors: ValidationError[];
  warnings: ValidationWarning[];
  suggestions: string[];
}

export interface LockedGameDNA extends GameDNA {
  checksum: string;
  locked_at: string;
  locked_by: string;
}

export interface VersionInfo {
  version: number;
  created_at: string;
  created_by: string;
  changes: string[];
  config: GameDNA;
}

export interface GameTemplate {
  name: string;
  description: string;
  config: Partial<GameDNA>;
}

export interface ListGameDNARequest {
  filters?: {
    genre?: Genre;
    platform?: TargetPlatform;
    search?: string;
  };
  limit?: number;
  offset?: number;
}

export function getEnumString<T extends string, C extends Record<string, string>>(value: T | C, key: keyof C): string {
  if (typeof value === 'string') return value;
  return String(value[key]);
}

export function getGenreString(genre: Genre): string {
  return getEnumString(genre, 'CustomGenre');
}

export function getCameraModeString(camera: CameraMode): string {
  return getEnumString(camera, 'CustomCamera');
}

export function getToneString(tone: Tone): string {
  return getEnumString(tone, 'CustomTone');
}

export function getWorldScaleString(scale: WorldScale): string {
  return getEnumString(scale, 'CustomScale');
}
