package storage

import (
    "context"
    "database/sql"
    "encoding/json"
    "fmt"
    "time"

    "github.com/google/uuid"
    "github.com/lib/pq"
    pb "github.com/entropic-engine/entropic-dna-api/gen/proto/entropic/dna/v1"
)

// PostgresStore is a PostgreSQL implementation of the Store interface.
type PostgresStore struct {
    db *sql.DB
}

// DB returns the underlying database connection for migrations.
func (p *PostgresStore) DB() *sql.DB {
    return p.db
}

// NewPostgresStore creates a new PostgreSQL storage backend.
func NewPostgresStore(connectionURL string) (*PostgresStore, error) {
    db, err := sql.Open("postgres", connectionURL)
    if err != nil {
        return nil, fmt.Errorf("failed to open database connection: %w", err)
    }

    if err := db.Ping(); err != nil {
        return nil, fmt.Errorf("failed to ping database: %w", err)
    }

    db.SetMaxOpenConns(25)
    db.SetMaxIdleConns(25)
    db.SetConnMaxLifetime(5 * time.Minute)

    return &PostgresStore{db: db}, nil
}

// Create creates a new GameDNA configuration.
func (p *PostgresStore) Create(ctx context.Context, dna *pb.GameDNA) (*pb.GameDNA, error) {
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

    dataJSON, err := json.Marshal(dna)
    if err != nil {
        return nil, fmt.Errorf("failed to marshal game DNA: %w", err)
    }

    query := `
        INSERT INTO game_dna_configs (id, name, version, data, checksum, is_locked, created_at, updated_at, created_by, tags)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id
    `

    createdAt, _ := time.Parse(time.RFC3339, dna.CreatedAt)
    updatedAt, _ := time.Parse(time.RFC3339, dna.LastModified)

    err = p.db.QueryRowContext(
        ctx, query,
        dna.Id, dna.Name, dna.Version, string(dataJSON), dna.Checksum, dna.IsLocked,
        createdAt, updatedAt, dna.CreatedBy, pq.Array(dna.Tags),
    ).Scan(&dna.Id)
    if err != nil {
        return nil, fmt.Errorf("failed to create game DNA: %w", err)
    }

    // Create initial version snapshot
    versionQuery := `
        INSERT INTO game_dna_versions (config_id, version_num, data, checksum, created_at, created_by)
        VALUES ($1, 1, $2, $3, $4, $5)
    `
    _, err = p.db.ExecContext(ctx, versionQuery, dna.Id, string(dataJSON), dna.Checksum, createdAt, dna.CreatedBy)
    if err != nil {
        return nil, fmt.Errorf("failed to create version snapshot: %w", err)
    }

    return dna, nil
}

// Read retrieves a GameDNA configuration by ID.
func (p *PostgresStore) Read(ctx context.Context, id string) (*pb.GameDNA, error) {
    query := `
        SELECT data FROM game_dna_configs WHERE id = $1
    `

    var dataJSON string
    err := p.db.QueryRowContext(ctx, query, id).Scan(&dataJSON)
    if err == sql.ErrNoRows {
        return nil, fmt.Errorf("config not found: %s", id)
    }
    if err != nil {
        return nil, fmt.Errorf("failed to read game DNA: %w", err)
    }

    var dna pb.GameDNA
    if err := json.Unmarshal([]byte(dataJSON), &dna); err != nil {
        return nil, fmt.Errorf("failed to unmarshal game DNA: %w", err)
    }

    return &dna, nil
}

// Update updates an existing GameDNA configuration.
func (p *PostgresStore) Update(ctx context.Context, dna *pb.GameDNA) (*pb.GameDNA, error) {
    // Check if exists and not locked
    var isLocked bool
    checkQuery := `SELECT is_locked FROM game_dna_configs WHERE id = $1`
    err := p.db.QueryRowContext(ctx, checkQuery, dna.Id).Scan(&isLocked)
    if err == sql.ErrNoRows {
        return nil, fmt.Errorf("config not found: %s", dna.Id)
    }
    if err != nil {
        return nil, fmt.Errorf("failed to check config: %w", err)
    }
    if isLocked {
        return nil, fmt.Errorf("config is locked: %s", dna.Id)
    }

    dna.LastModified = time.Now().Format(time.RFC3339)

    dataJSON, err := json.Marshal(dna)
    if err != nil {
        return nil, fmt.Errorf("failed to marshal game DNA: %w", err)
    }

    updateQuery := `
        UPDATE game_dna_configs
        SET data = $1, checksum = $2, updated_at = $3, tags = $4, name = $5, version = $6
        WHERE id = $7
    `

    updatedAt, _ := time.Parse(time.RFC3339, dna.LastModified)

    _, err = p.db.ExecContext(
        ctx, updateQuery,
        string(dataJSON), dna.Checksum, updatedAt, pq.Array(dna.Tags), dna.Name, dna.Version, dna.Id,
    )
    if err != nil {
        return nil, fmt.Errorf("failed to update game DNA: %w", err)
    }

    // Create new version snapshot
    versionCountQuery := `SELECT COALESCE(MAX(version_num), 0) FROM game_dna_versions WHERE config_id = $1`
    var maxVersion int64
    err = p.db.QueryRowContext(ctx, versionCountQuery, dna.Id).Scan(&maxVersion)
    if err != nil {
        return nil, fmt.Errorf("failed to get version count: %w", err)
    }

    nextVersion := maxVersion + 1
    versionQuery := `
        INSERT INTO game_dna_versions (config_id, version_num, data, checksum, created_at, created_by)
        VALUES ($1, $2, $3, $4, $5, $6)
    `
    _, err = p.db.ExecContext(ctx, versionQuery, dna.Id, nextVersion, string(dataJSON), dna.Checksum, updatedAt, dna.CreatedBy)
    if err != nil {
        return nil, fmt.Errorf("failed to create version snapshot: %w", err)
    }

    return dna, nil
}

// Delete removes a GameDNA configuration.
func (p *PostgresStore) Delete(ctx context.Context, id string) error {
    query := `DELETE FROM game_dna_configs WHERE id = $1`
    result, err := p.db.ExecContext(ctx, query, id)
    if err != nil {
        return fmt.Errorf("failed to delete game DNA: %w", err)
    }

    rows, err := result.RowsAffected()
    if err != nil {
        return fmt.Errorf("failed to get affected rows: %w", err)
    }
    if rows == 0 {
        return fmt.Errorf("config not found: %s", id)
    }

    return nil
}

// List retrieves all GameDNA configurations with filtering and pagination.
func (p *PostgresStore) List(ctx context.Context, filters ListFilters, pagination Pagination) ([]*pb.GameDNA, int32, error) {
    if pagination.PageSize == 0 {
        pagination.PageSize = 10
    }
    if pagination.Page == 0 {
        pagination.Page = 1
    }

    whereClause := "WHERE 1=1"
    args := []interface{}{}
    argCount := 1

    if filters.Genre != "" {
        whereClause += fmt.Sprintf(" AND data->>'genre' = $%d", argCount)
        args = append(args, filters.Genre)
        argCount++
    }

    if filters.NameFilter != "" {
        whereClause += fmt.Sprintf(" AND LOWER(name) LIKE LOWER($%d)", argCount)
        args = append(args, "%"+filters.NameFilter+"%")
        argCount++
    }

    if len(filters.Tags) > 0 {
        whereClause += fmt.Sprintf(" AND tags @> $%d", argCount)
        args = append(args, pq.Array(filters.Tags))
        argCount++
    }

    // Count total
    countQuery := "SELECT COUNT(*) FROM game_dna_configs " + whereClause
    var total int32
    err := p.db.QueryRowContext(ctx, countQuery, args...).Scan(&total)
    if err != nil {
        return nil, 0, fmt.Errorf("failed to count configs: %w", err)
    }

    // Get paginated results
    offset := (pagination.Page - 1) * pagination.PageSize
    query := fmt.Sprintf(`
        SELECT data FROM game_dna_configs
        %s
        ORDER BY created_at DESC
        LIMIT $%d OFFSET $%d
    `, whereClause, argCount, argCount+1)
    args = append(args, pagination.PageSize, offset)

    rows, err := p.db.QueryContext(ctx, query, args...)
    if err != nil {
        return nil, 0, fmt.Errorf("failed to list game DNAs: %w", err)
    }
    defer rows.Close()

    var result []*pb.GameDNA
    for rows.Next() {
        var dataJSON string
        if err := rows.Scan(&dataJSON); err != nil {
            return nil, 0, fmt.Errorf("failed to scan row: %w", err)
        }

        var dna pb.GameDNA
        if err := json.Unmarshal([]byte(dataJSON), &dna); err != nil {
            return nil, 0, fmt.Errorf("failed to unmarshal game DNA: %w", err)
        }

        result = append(result, &dna)
    }

    if err := rows.Err(); err != nil {
        return nil, 0, fmt.Errorf("row iteration error: %w", err)
    }

    return result, total, nil
}

// GetVersionHistory retrieves the version history for a configuration.
func (p *PostgresStore) GetVersionHistory(ctx context.Context, configID string) ([]*VersionInfo, error) {
    query := `
        SELECT version_num, checksum, created_at, created_by, data
        FROM game_dna_versions
        WHERE config_id = $1
        ORDER BY version_num DESC
    `

    rows, err := p.db.QueryContext(ctx, query, configID)
    if err != nil {
        return nil, fmt.Errorf("failed to query version history: %w", err)
    }
    defer rows.Close()

    var versions []*VersionInfo
    for rows.Next() {
        var v VersionInfo
        var dataJSON string
        var createdAt time.Time

        if err := rows.Scan(&v.VersionNum, &v.Checksum, &createdAt, &v.CreatedBy, &dataJSON); err != nil {
            return nil, fmt.Errorf("failed to scan version row: %w", err)
        }

        v.CreatedAt = createdAt.Format(time.RFC3339)

        var dna pb.GameDNA
        if err := json.Unmarshal([]byte(dataJSON), &dna); err != nil {
            return nil, fmt.Errorf("failed to unmarshal game DNA: %w", err)
        }
        v.Data = &dna

        versions = append(versions, &v)
    }

    if err := rows.Err(); err != nil {
        return nil, fmt.Errorf("row iteration error: %w", err)
    }

    return versions, nil
}

// RollbackToVersion rolls back a configuration to a previous version.
func (p *PostgresStore) RollbackToVersion(ctx context.Context, configID string, versionNum int64, actor string) (*pb.GameDNA, error) {
    query := `
        SELECT data FROM game_dna_versions
        WHERE config_id = $1 AND version_num = $2
    `

    var dataJSON string
    err := p.db.QueryRowContext(ctx, query, configID, versionNum).Scan(&dataJSON)
    if err == sql.ErrNoRows {
        return nil, fmt.Errorf("version not found: %d", versionNum)
    }
    if err != nil {
        return nil, fmt.Errorf("failed to read version: %w", err)
    }

    var dna pb.GameDNA
    if err := json.Unmarshal([]byte(dataJSON), &dna); err != nil {
        return nil, fmt.Errorf("failed to unmarshal game DNA: %w", err)
    }

    // Update with new timestamp and actor
    dna.LastModified = time.Now().Format(time.RFC3339)
    if actor != "" {
        dna.CreatedBy = actor
    }

    // Update the main config
    return p.Update(ctx, &dna)
}

// PublishVersion locks a configuration and creates an immutable snapshot.
func (p *PostgresStore) PublishVersion(ctx context.Context, configID string, actor string) (*pb.GameDNA, error) {
    // Get current config
    dna, err := p.Read(ctx, configID)
    if err != nil {
        return nil, err
    }

    if dna.IsLocked {
        return nil, fmt.Errorf("config is already locked: %s", configID)
    }

    // Lock the config
    dna.IsLocked = true
    dna.LastModified = time.Now().Format(time.RFC3339)
    if actor != "" {
        dna.CreatedBy = actor
    }

    dataJSON, err := json.Marshal(dna)
    if err != nil {
        return nil, fmt.Errorf("failed to marshal game DNA: %w", err)
    }

    updateQuery := `
        UPDATE game_dna_configs
        SET is_locked = true, data = $1, updated_at = $2
        WHERE id = $3
    `

    updatedAt, _ := time.Parse(time.RFC3339, dna.LastModified)
    _, err = p.db.ExecContext(ctx, updateQuery, string(dataJSON), updatedAt, configID)
    if err != nil {
        return nil, fmt.Errorf("failed to publish config: %w", err)
    }

    return dna, nil
}

// Clone creates a new configuration based on an existing one.
func (p *PostgresStore) Clone(ctx context.Context, id string, newName string, actor string) (*pb.GameDNA, error) {
    original, err := p.Read(ctx, id)
    if err != nil {
        return nil, err
    }

    // Create a new ID and reset metadata
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

    return p.Create(ctx, cloned)
}

// Close closes the database connection.
func (p *PostgresStore) Close() {
    if p.db != nil {
        p.db.Close()
    }
}
