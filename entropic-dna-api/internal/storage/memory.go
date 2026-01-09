package storage

import (
    "context"
    "fmt"
    "strings"
    "sync"
    "time"

    "github.com/google/uuid"
    pb "github.com/entropic-engine/entropic-dna-api/gen/proto/entropic/dna/v1"
)

// MemoryStore is an in-memory implementation of the Store interface.
type MemoryStore struct {
    mu       sync.RWMutex
    configs  map[string]*pb.GameDNA
    versions map[string][]*VersionInfo
}

// deepCopyGameDNA creates a deep copy of a GameDNA protobuf message
func deepCopyGameDNA(src *pb.GameDNA) *pb.GameDNA {
    if src == nil {
        return nil
    }
    dst := &pb.GameDNA{
        Id:                  src.Id,
        Name:                src.Name,
        Version:             src.Version,
        CreatedAt:           src.CreatedAt,
        LastModified:        src.LastModified,
        CreatedBy:           src.CreatedBy,
        Checksum:            src.Checksum,
        IsLocked:            src.IsLocked,
        Genre:               src.Genre,
        Camera:              src.Camera,
        Tone:                src.Tone,
        WorldScale:          src.WorldScale,
        TargetPlatforms:     append([]string{}, src.TargetPlatforms...),
        PhysicsProfile:      src.PhysicsProfile,
        MaxPlayers:          src.MaxPlayers,
        IsCompetitive:       src.IsCompetitive,
        SupportsCoop:        src.SupportsCoop,
        Difficulty:          src.Difficulty,
        Monetization:        src.Monetization,
        TargetAudience:      src.TargetAudience,
        EsrbRating:          src.EsrbRating,
        TargetFps:           src.TargetFps,
        MaxDrawDistance:     src.MaxDrawDistance,
        MaxEntities:         src.MaxEntities,
        MaxNpcCount:         src.MaxNpcCount,
        TimeScale:           src.TimeScale,
        WeatherEnabled:      src.WeatherEnabled,
        SeasonsEnabled:      src.SeasonsEnabled,
        DayNightCycle:       src.DayNightCycle,
        PersistentWorld:     src.PersistentWorld,
        NpcCount:            src.NpcCount,
        AiEnabled:           src.AiEnabled,
        AiDifficultyScaling: src.AiDifficultyScaling,
        HasCampaign:         src.HasCampaign,
        HasSideQuests:       src.HasSideQuests,
        DynamicQuests:       src.DynamicQuests,
        Tags:                append([]string{}, src.Tags...),
        CustomProperties:    make(map[string]string),
    }
    for k, v := range src.CustomProperties {
        dst.CustomProperties[k] = v
    }
    return dst
}

// NewMemoryStore creates a new in-memory storage backend.
func NewMemoryStore() *MemoryStore {
    return &MemoryStore{
        configs:  make(map[string]*pb.GameDNA),
        versions: make(map[string][]*VersionInfo),
    }
}

// Create creates a new GameDNA configuration.
func (m *MemoryStore) Create(ctx context.Context, dna *pb.GameDNA) (*pb.GameDNA, error) {
    m.mu.Lock()
    defer m.mu.Unlock()

    if dna.Id == "" {
        dna.Id = uuid.New().String()
    }

    if dna.CreatedAt == "" {
        dna.CreatedAt = time.Now().Format(time.RFC3339)
    }
    if dna.LastModified == "" {
        dna.LastModified = time.Now().Format(time.RFC3339)
    }
    if dna.Version == "" {
        dna.Version = "0.1.0"
    }

    m.configs[dna.Id] = dna

    // Create initial version snapshot
    m.versions[dna.Id] = []*VersionInfo{
        {
            VersionNum: 1,
            Checksum:   dna.Checksum,
            CreatedAt:  dna.CreatedAt,
            CreatedBy:  dna.CreatedBy,
            Data:       deepCopyGameDNA(dna),
        },
    }

    return dna, nil
}

// Read retrieves a GameDNA configuration by ID.
func (m *MemoryStore) Read(ctx context.Context, id string) (*pb.GameDNA, error) {
    m.mu.RLock()
    defer m.mu.RUnlock()

    dna, exists := m.configs[id]
    if !exists {
        return nil, fmt.Errorf("config not found: %s", id)
    }

    return dna, nil
}

// Update updates an existing GameDNA configuration.
func (m *MemoryStore) Update(ctx context.Context, dna *pb.GameDNA) (*pb.GameDNA, error) {
    m.mu.Lock()
    defer m.mu.Unlock()

    existing, exists := m.configs[dna.Id]
    if !exists {
        return nil, fmt.Errorf("config not found: %s", dna.Id)
    }

    if existing.IsLocked {
        return nil, fmt.Errorf("config is locked: %s", dna.Id)
    }

    dna.LastModified = time.Now().Format(time.RFC3339)
    m.configs[dna.Id] = dna

    // Create new version snapshot
    nextVersion := int64(len(m.versions[dna.Id]) + 1)
    m.versions[dna.Id] = append(m.versions[dna.Id], &VersionInfo{
        VersionNum: nextVersion,
        Checksum:   dna.Checksum,
        CreatedAt:  dna.LastModified,
        CreatedBy:  dna.CreatedBy,
        Data:       deepCopyGameDNA(dna),
    })

    return dna, nil
}

// Delete removes a GameDNA configuration.
func (m *MemoryStore) Delete(ctx context.Context, id string) error {
    m.mu.Lock()
    defer m.mu.Unlock()

    if _, exists := m.configs[id]; !exists {
        return fmt.Errorf("config not found: %s", id)
    }

    delete(m.configs, id)
    delete(m.versions, id)

    return nil
}

// List retrieves all GameDNA configurations with filtering and pagination.
func (m *MemoryStore) List(ctx context.Context, filters ListFilters, pagination Pagination) ([]*pb.GameDNA, int32, error) {
    m.mu.RLock()
    defer m.mu.RUnlock()

    var result []*pb.GameDNA

    for _, dna := range m.configs {
        // Apply filters
        if filters.Genre != "" && dna.Genre != filters.Genre {
            continue
        }
        if filters.NameFilter != "" && !strings.Contains(strings.ToLower(dna.Name), strings.ToLower(filters.NameFilter)) {
            continue
        }
        if len(filters.Tags) > 0 {
            hasAllTags := true
            for _, tag := range filters.Tags {
                found := false
                for _, dnaTag := range dna.Tags {
                    if dnaTag == tag {
                        found = true
                        break
                    }
                }
                if !found {
                    hasAllTags = false
                    break
                }
            }
            if !hasAllTags {
                continue
            }
        }

        result = append(result, dna)
    }

    total := int32(len(result))

    // Apply pagination
    if pagination.PageSize == 0 {
        pagination.PageSize = 10
    }
    if pagination.Page == 0 {
        pagination.Page = 1
    }

    start := (pagination.Page - 1) * pagination.PageSize
    end := start + pagination.PageSize

    if start >= int32(len(result)) {
        return []*pb.GameDNA{}, total, nil
    }
    if end > int32(len(result)) {
        end = int32(len(result))
    }

    return result[start:end], total, nil
}

// GetVersionHistory retrieves the version history for a configuration.
func (m *MemoryStore) GetVersionHistory(ctx context.Context, configID string) ([]*VersionInfo, error) {
    m.mu.RLock()
    defer m.mu.RUnlock()

    versions, exists := m.versions[configID]
    if !exists {
        return nil, fmt.Errorf("config not found: %s", configID)
    }

    return versions, nil
}

// RollbackToVersion rolls back a configuration to a previous version.
func (m *MemoryStore) RollbackToVersion(ctx context.Context, configID string, versionNum int64, actor string) (*pb.GameDNA, error) {
    m.mu.Lock()
    defer m.mu.Unlock()

    versions, exists := m.versions[configID]
    if !exists {
        return nil, fmt.Errorf("config not found: %s", configID)
    }

    var targetVersion *VersionInfo
    for _, v := range versions {
        if v.VersionNum == versionNum {
            targetVersion = v
            break
        }
    }

    if targetVersion == nil {
        return nil, fmt.Errorf("version not found: %d", versionNum)
    }

    // Deep copy the version data and create new current config
    rolledBack := deepCopyGameDNA(targetVersion.Data)
    rolledBack.LastModified = time.Now().Format(time.RFC3339)
    if actor != "" {
        rolledBack.CreatedBy = actor
    }

    m.configs[configID] = rolledBack

    // Add rollback as a new version
    nextVersion := int64(len(versions) + 1)
    m.versions[configID] = append(versions, &VersionInfo{
        VersionNum: nextVersion,
        Checksum:   rolledBack.Checksum,
        CreatedAt:  rolledBack.LastModified,
        CreatedBy:  actor,
        Data:       deepCopyGameDNA(rolledBack),
    })

    return rolledBack, nil
}

// PublishVersion locks a configuration and creates an immutable snapshot.
func (m *MemoryStore) PublishVersion(ctx context.Context, configID string, actor string) (*pb.GameDNA, error) {
    m.mu.Lock()
    defer m.mu.Unlock()

    dna, exists := m.configs[configID]
    if !exists {
        return nil, fmt.Errorf("config not found: %s", configID)
    }

    if dna.IsLocked {
        return nil, fmt.Errorf("config is already locked: %s", configID)
    }

    dna.IsLocked = true
    dna.LastModified = time.Now().Format(time.RFC3339)
    if actor != "" {
        dna.CreatedBy = actor
    }

    m.configs[configID] = dna

    return dna, nil
}

// Clone creates a new configuration based on an existing one.
func (m *MemoryStore) Clone(ctx context.Context, id string, newName string, actor string) (*pb.GameDNA, error) {
    m.mu.Lock()
    defer m.mu.Unlock()

    original, exists := m.configs[id]
    if !exists {
        return nil, fmt.Errorf("config not found: %s", id)
    }

    // Create a deep copy
    cloned := &pb.GameDNA{
        Id:                  uuid.New().String(),
        Name:                newName,
        Version:             original.Version,
        CreatedAt:           time.Now().Format(time.RFC3339),
        LastModified:        time.Now().Format(time.RFC3339),
        CreatedBy:           actor,
        Checksum:            "",
        IsLocked:            false,
        Genre:               original.Genre,
        Camera:              original.Camera,
        Tone:                original.Tone,
        WorldScale:          original.WorldScale,
        TargetPlatforms:     append([]string{}, original.TargetPlatforms...),
        PhysicsProfile:      original.PhysicsProfile,
        MaxPlayers:          original.MaxPlayers,
        IsCompetitive:       original.IsCompetitive,
        SupportsCoop:        original.SupportsCoop,
        Difficulty:          original.Difficulty,
        Monetization:        original.Monetization,
        TargetAudience:      original.TargetAudience,
        EsrbRating:          original.EsrbRating,
        TargetFps:           original.TargetFps,
        MaxDrawDistance:     original.MaxDrawDistance,
        MaxEntities:         original.MaxEntities,
        MaxNpcCount:         original.MaxNpcCount,
        TimeScale:           original.TimeScale,
        WeatherEnabled:      original.WeatherEnabled,
        SeasonsEnabled:      original.SeasonsEnabled,
        DayNightCycle:       original.DayNightCycle,
        PersistentWorld:     original.PersistentWorld,
        NpcCount:            original.NpcCount,
        AiEnabled:           original.AiEnabled,
        AiDifficultyScaling: original.AiDifficultyScaling,
        HasCampaign:         original.HasCampaign,
        HasSideQuests:       original.HasSideQuests,
        DynamicQuests:       original.DynamicQuests,
        Tags:                append([]string{}, original.Tags...),
        CustomProperties:    make(map[string]string),
    }

    // Deep copy custom properties
    for k, v := range original.CustomProperties {
        cloned.CustomProperties[k] = v
    }

    m.configs[cloned.Id] = cloned

    // Create initial version snapshot
    m.versions[cloned.Id] = []*VersionInfo{
        {
            VersionNum: 1,
            Checksum:   cloned.Checksum,
            CreatedAt:  cloned.CreatedAt,
            CreatedBy:  actor,
            Data:       cloned,
        },
    }

    return cloned, nil
}

// Close closes the storage backend (no-op for memory storage).
func (m *MemoryStore) Close() {
    // No-op for in-memory storage
}
