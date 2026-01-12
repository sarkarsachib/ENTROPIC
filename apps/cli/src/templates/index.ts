import type { GameDNA, GameTemplate } from '@entropic/types';
import { randomUUID } from 'crypto';

const FPS_TEMPLATE: Partial<GameDNA> = {
  genre: 'FPS',
  camera: 'Perspective3D',
  tone: 'Realistic',
  world_scale: 'SmallLevel',
  target_platforms: ['PC', 'Console'],
  physics_profile: 'SemiRealistic',
  max_players: 16,
  is_competitive: true,
  supports_coop: false,
  difficulty: 'Dynamic',
  monetization: 'PremiumBuy',
  target_audience: 'Mature',
  target_fps: 60,
  max_draw_distance: 2000,
  max_entities: 500,
  max_npc_count: 20,
  time_scale: 1.0,
  weather_enabled: false,
  seasons_enabled: false,
  day_night_cycle: false,
  persistent_world: false,
  npc_count: 10,
  ai_enabled: true,
  ai_difficulty_scaling: true,
  has_campaign: true,
  has_side_quests: false,
  dynamic_quests: false,
  tags: ['action', 'shooter', 'multiplayer'],
  custom_properties: {},
};

const RPG_TEMPLATE: Partial<GameDNA> = {
  genre: 'RPG',
  camera: 'Perspective3D',
  tone: 'Cinematic',
  world_scale: 'OpenWorld',
  target_platforms: ['PC', 'Console'],
  physics_profile: 'SemiRealistic',
  max_players: 4,
  is_competitive: false,
  supports_coop: true,
  difficulty: 'Dynamic',
  monetization: 'PremiumBuy',
  target_audience: 'Teen',
  target_fps: 60,
  max_draw_distance: 5000,
  max_entities: 1000,
  max_npc_count: 100,
  time_scale: 1.0,
  weather_enabled: true,
  seasons_enabled: true,
  day_night_cycle: true,
  persistent_world: true,
  npc_count: 50,
  ai_enabled: true,
  ai_difficulty_scaling: true,
  has_campaign: true,
  has_side_quests: true,
  dynamic_quests: true,
  tags: ['rpg', 'adventure', 'open-world'],
  custom_properties: {},
};

const CASUAL_TEMPLATE: Partial<GameDNA> = {
  genre: 'Casual',
  camera: 'Perspective2D',
  tone: 'Stylized',
  world_scale: 'TinyLevel',
  target_platforms: ['Mobile', 'PC'],
  physics_profile: 'Arcade',
  max_players: 1,
  is_competitive: false,
  supports_coop: false,
  difficulty: 'Easy',
  monetization: 'FreeToPlay',
  target_audience: 'Everyone',
  target_fps: 30,
  max_draw_distance: 500,
  max_entities: 50,
  max_npc_count: 5,
  time_scale: 1.0,
  weather_enabled: false,
  seasons_enabled: false,
  day_night_cycle: false,
  persistent_world: false,
  npc_count: 0,
  ai_enabled: false,
  ai_difficulty_scaling: false,
  has_campaign: false,
  has_side_quests: false,
  dynamic_quests: false,
  tags: ['casual', 'puzzle', 'family-friendly'],
  custom_properties: {},
};

const STRATEGY_TEMPLATE: Partial<GameDNA> = {
  genre: 'Strategy',
  camera: 'Isometric',
  tone: 'Realistic',
  world_scale: 'LargeLevel',
  target_platforms: ['PC'],
  physics_profile: 'SemiRealistic',
  max_players: 8,
  is_competitive: true,
  supports_coop: false,
  difficulty: 'Hard',
  monetization: 'PremiumBuy',
  target_audience: 'Everyone',
  target_fps: 60,
  max_draw_distance: 3000,
  max_entities: 2000,
  max_npc_count: 500,
  time_scale: 1.0,
  weather_enabled: true,
  seasons_enabled: false,
  day_night_cycle: false,
  persistent_world: false,
  npc_count: 100,
  ai_enabled: true,
  ai_difficulty_scaling: true,
  has_campaign: true,
  has_side_quests: false,
  dynamic_quests: false,
  tags: ['strategy', 'rts', 'competitive'],
  custom_properties: {},
};

export const TEMPLATES: Record<string, GameTemplate> = {
  fps: {
    name: 'First Person Shooter',
    description: 'Competitive multiplayer FPS with realistic graphics',
    config: FPS_TEMPLATE,
  },
  rpg: {
    name: 'Open World RPG',
    description: 'Epic open-world RPG with dynamic quests and co-op',
    config: RPG_TEMPLATE,
  },
  casual: {
    name: 'Casual Mobile Game',
    description: 'Simple, accessible mobile-first casual game',
    config: CASUAL_TEMPLATE,
  },
  strategy: {
    name: 'Real-Time Strategy',
    description: 'Competitive RTS with large-scale battles',
    config: STRATEGY_TEMPLATE,
  },
};

/**
 * Create a GameDNA object from a named template in the TEMPLATES registry.
 *
 * @param templateName - Key of the template to load from `TEMPLATES`
 * @returns A GameDNA object containing a generated `id`, default `name` ("New Game"), initial `version` (0.1.0), and the selected template's configuration
 * @throws Error if no template exists for `templateName`; the error message lists available template keys
 */
export function loadTemplate(templateName: string): GameDNA {
  const template = TEMPLATES[templateName];
  if (!template) {
    throw new Error(
      `Template '${templateName}' not found. Available: ${Object.keys(TEMPLATES).join(', ')}`
    );
  }

  return {
    id: randomUUID(),
    name: 'New Game',
    version: { major: 0, minor: 1, patch: 0 },
    ...template.config,
  } as GameDNA;
}