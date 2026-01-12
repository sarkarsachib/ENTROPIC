-- +migrate Up
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS game_dna_configs (
  id UUID PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  version VARCHAR(20) NOT NULL,
  data JSONB NOT NULL,
  checksum VARCHAR(64) NOT NULL,
  is_locked BOOLEAN DEFAULT FALSE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  created_by VARCHAR(255),
  tags TEXT[],
  UNIQUE(name, version)
);

CREATE TABLE IF NOT EXISTS game_dna_versions (
  id SERIAL PRIMARY KEY,
  config_id UUID REFERENCES game_dna_configs(id) ON DELETE CASCADE,
  version_num INT NOT NULL,
  data JSONB NOT NULL,
  checksum VARCHAR(64),
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  created_by VARCHAR(255),
  UNIQUE(config_id, version_num)
);

CREATE INDEX IF NOT EXISTS idx_game_dna_name ON game_dna_configs(name);
CREATE INDEX IF NOT EXISTS idx_game_dna_tags ON game_dna_configs USING GIN(tags);

-- +migrate Down
DROP TABLE IF EXISTS game_dna_versions;
DROP TABLE IF EXISTS game_dna_configs;
