package api

import (
    "context"
    "fmt"

    pb "github.com/entropic-engine/entropic-dna-api/gen/proto/entropic/dna/v1"
    "github.com/entropic-engine/entropic-dna-api/internal/ffi"
    "github.com/entropic-engine/entropic-dna-api/internal/storage"
    "go.uber.org/zap"
)

// GameDNAServiceServer implements the gRPC service.
type GameDNAServiceServer struct {
    pb.UnimplementedGameDNAServiceServer
    store  storage.Store
    rust   *ffi.RustFFI
    logger *zap.Logger
}

// NewGameDNAServiceServer creates a new gRPC service server.
func NewGameDNAServiceServer(store storage.Store, rust *ffi.RustFFI, logger *zap.Logger) *GameDNAServiceServer {
    return &GameDNAServiceServer{
        store:  store,
        rust:   rust,
        logger: logger,
    }
}

// CreateGameDNA creates a new game configuration.
func (s *GameDNAServiceServer) CreateGameDNA(ctx context.Context, req *pb.CreateGameDNARequest) (*pb.GameDNAResponse, error) {
    s.logger.Info("Creating game DNA", zap.String("name", req.GameDna.Name))

    // Validate the configuration
    validationResp, err := s.rust.ValidateGameDNA(req.GameDna)
    if err != nil {
        s.logger.Error("Validation error", zap.Error(err))
        return nil, fmt.Errorf("validation error: %w", err)
    }
    if !validationResp.IsValid {
        s.logger.Warn("Validation failed for create", zap.Int("errors", len(validationResp.Errors)))
        return nil, fmt.Errorf("validation failed: %d errors", len(validationResp.Errors))
    }

    // Calculate checksum
    checksum, err := s.rust.CalculateChecksum(req.GameDna)
    if err != nil {
        s.logger.Error("Failed to calculate checksum", zap.Error(err))
        return nil, fmt.Errorf("failed to calculate checksum: %w", err)
    }
    req.GameDna.Checksum = checksum

    // Store the configuration
    created, err := s.store.Create(ctx, req.GameDna)
    if err != nil {
        s.logger.Error("Failed to create game DNA", zap.Error(err))
        return nil, fmt.Errorf("failed to create game DNA: %w", err)
    }

    s.logger.Info("Game DNA created", zap.String("id", created.Id))

    return &pb.GameDNAResponse{
        GameDna: created,
        Message: "Game DNA created successfully",
    }, nil
}

// GetGameDNA retrieves a game configuration by ID.
func (s *GameDNAServiceServer) GetGameDNA(ctx context.Context, req *pb.GetGameDNARequest) (*pb.GameDNAResponse, error) {
    s.logger.Info("Getting game DNA", zap.String("id", req.Id))

    dna, err := s.store.Read(ctx, req.Id)
    if err != nil {
        s.logger.Error("Failed to read game DNA", zap.Error(err))
        return nil, fmt.Errorf("failed to read game DNA: %w", err)
    }

    return &pb.GameDNAResponse{
        GameDna: dna,
        Message: "Game DNA retrieved successfully",
    }, nil
}

// ListGameDNA lists all game configurations with filtering and pagination.
func (s *GameDNAServiceServer) ListGameDNA(ctx context.Context, req *pb.ListGameDNARequest) (*pb.ListGameDNAResponse, error) {
    s.logger.Info("Listing game DNAs", zap.Int32("page", req.Page))

    filters := storage.ListFilters{
        Tags:       req.Tags,
        Genre:      req.Genre,
        NameFilter: req.NameFilter,
    }

    pagination := storage.Pagination{
        Page:     req.Page,
        PageSize: req.PageSize,
    }

    items, total, err := s.store.List(ctx, filters, pagination)
    if err != nil {
        s.logger.Error("Failed to list game DNAs", zap.Error(err))
        return nil, fmt.Errorf("failed to list game DNAs: %w", err)
    }

    pageSize := req.PageSize
    if pageSize == 0 {
        pageSize = 10
    }
    page := req.Page
    if page == 0 {
        page = 1
    }
    totalPages := (total + pageSize - 1) / pageSize

    return &pb.ListGameDNAResponse{
        Items: items,
        Pagination: &pb.PaginationInfo{
            Page:       page,
            PageSize:   pageSize,
            Total:      total,
            TotalPages: totalPages,
        },
    }, nil
}

// UpdateGameDNA updates an existing game configuration.
func (s *GameDNAServiceServer) UpdateGameDNA(ctx context.Context, req *pb.UpdateGameDNARequest) (*pb.GameDNAResponse, error) {
    s.logger.Info("Updating game DNA", zap.String("id", req.Id))

    // Ensure ID matches
    req.GameDna.Id = req.Id

    // Validate the configuration
    validationResp, err := s.rust.ValidateGameDNA(req.GameDna)
    if err != nil {
        s.logger.Error("Validation error", zap.Error(err))
        return nil, fmt.Errorf("validation error: %w", err)
    }
    if !validationResp.IsValid {
        s.logger.Warn("Validation failed for update", zap.Int("errors", len(validationResp.Errors)))
        return nil, fmt.Errorf("validation failed: %d errors", len(validationResp.Errors))
    }

    // Calculate new checksum
    checksum, err := s.rust.CalculateChecksum(req.GameDna)
    if err != nil {
        s.logger.Error("Failed to calculate checksum", zap.Error(err))
        return nil, fmt.Errorf("failed to calculate checksum: %w", err)
    }
    req.GameDna.Checksum = checksum

    // Update the configuration
    updated, err := s.store.Update(ctx, req.GameDna)
    if err != nil {
        s.logger.Error("Failed to update game DNA", zap.Error(err))
        return nil, fmt.Errorf("failed to update game DNA: %w", err)
    }

    s.logger.Info("Game DNA updated", zap.String("id", updated.Id))

    return &pb.GameDNAResponse{
        GameDna: updated,
        Message: "Game DNA updated successfully",
    }, nil
}

// DeleteGameDNA deletes a game configuration.
func (s *GameDNAServiceServer) DeleteGameDNA(ctx context.Context, req *pb.DeleteGameDNARequest) (*pb.DeleteGameDNAResponse, error) {
    s.logger.Info("Deleting game DNA", zap.String("id", req.Id))

    err := s.store.Delete(ctx, req.Id)
    if err != nil {
        s.logger.Error("Failed to delete game DNA", zap.Error(err))
        return nil, fmt.Errorf("failed to delete game DNA: %w", err)
    }

    s.logger.Info("Game DNA deleted", zap.String("id", req.Id))

    return &pb.DeleteGameDNAResponse{
        Success: true,
        Message: "Game DNA deleted successfully",
    }, nil
}

// ValidateGameDNA validates a game configuration without saving it.
func (s *GameDNAServiceServer) ValidateGameDNA(ctx context.Context, req *pb.ValidateGameDNARequest) (*pb.ValidationResponse, error) {
    var dna *pb.GameDNA
    if req.GetGameDna() != nil {
        dna = req.GetGameDna()
        if req.GetId() != "" {
            dna.Id = req.GetId()
        }
    } else if req.GetId() != "" {
        stored, err := s.store.Read(ctx, req.GetId())
        if err != nil {
            return nil, fmt.Errorf("failed to load stored config for validation: %w", err)
        }
        dna = stored
    } else {
        return nil, fmt.Errorf("either id or game_dna must be provided")
    }

    name := dna.GetName()
    if name == "" {
        name = dna.GetId()
    }
    s.logger.Info("Validating game DNA", zap.String("name", name), zap.String("id", dna.GetId()))

    validationResp, err := s.rust.ValidateGameDNA(dna)
    if err != nil {
        s.logger.Error("Validation error", zap.Error(err))
        return nil, fmt.Errorf("validation error: %w", err)
    }

    s.logger.Info("Validation complete",
        zap.Bool("valid", validationResp.IsValid),
        zap.Int("errors", len(validationResp.Errors)),
        zap.Int("warnings", len(validationResp.Warnings)),
    )

    return validationResp, nil
}

// PublishGameDNA locks a game configuration and creates an immutable snapshot.
func (s *GameDNAServiceServer) PublishGameDNA(ctx context.Context, req *pb.PublishGameDNARequest) (*pb.PublishedGameDNAResponse, error) {
    s.logger.Info("Publishing game DNA", zap.String("id", req.Id))

    published, err := s.store.PublishVersion(ctx, req.Id, "system")
    if err != nil {
        s.logger.Error("Failed to publish game DNA", zap.Error(err))
        return nil, fmt.Errorf("failed to publish game DNA: %w", err)
    }

    s.logger.Info("Game DNA published", zap.String("id", published.Id), zap.String("checksum", published.Checksum))

    return &pb.PublishedGameDNAResponse{
        GameDna:  published,
        Checksum: published.Checksum,
        Message:  "Game DNA published and locked successfully",
    }, nil
}

// GetVersionHistory retrieves the version history for a game configuration.
func (s *GameDNAServiceServer) GetVersionHistory(ctx context.Context, req *pb.GetVersionHistoryRequest) (*pb.VersionHistoryResponse, error) {
    s.logger.Info("Getting version history", zap.String("config_id", req.ConfigId))

    versions, err := s.store.GetVersionHistory(ctx, req.ConfigId)
    if err != nil {
        s.logger.Error("Failed to get version history", zap.Error(err))
        return nil, fmt.Errorf("failed to get version history: %w", err)
    }

    var pbVersions []*pb.VersionInfo
    for _, v := range versions {
        pbVersions = append(pbVersions, &pb.VersionInfo{
            VersionNum: v.VersionNum,
            Checksum:   v.Checksum,
            CreatedAt:  v.CreatedAt,
            CreatedBy:  v.CreatedBy,
            Data:       v.Data,
        })
    }

    s.logger.Info("Version history retrieved", zap.Int("count", len(pbVersions)))

    return &pb.VersionHistoryResponse{
        Versions: pbVersions,
    }, nil
}

// RollbackToVersion rolls back a game configuration to a previous version.
func (s *GameDNAServiceServer) RollbackToVersion(ctx context.Context, req *pb.RollbackToVersionRequest) (*pb.GameDNAResponse, error) {
    s.logger.Info("Rolling back to version",
        zap.String("config_id", req.ConfigId),
        zap.Int64("version", req.VersionNum),
    )

    rolled, err := s.store.RollbackToVersion(ctx, req.ConfigId, req.VersionNum, "system")
    if err != nil {
        s.logger.Error("Failed to rollback version", zap.Error(err))
        return nil, fmt.Errorf("failed to rollback version: %w", err)
    }

    s.logger.Info("Rolled back successfully", zap.String("id", rolled.Id))

    return &pb.GameDNAResponse{
        GameDna: rolled,
        Message: fmt.Sprintf("Rolled back to version %d successfully", req.VersionNum),
    }, nil
}

// CloneGameDNA creates a copy of an existing game configuration.
func (s *GameDNAServiceServer) CloneGameDNA(ctx context.Context, req *pb.CloneGameDNARequest) (*pb.GameDNAResponse, error) {
    s.logger.Info("Cloning game DNA",
        zap.String("id", req.Id),
        zap.String("new_name", req.NewName),
    )

    cloned, err := s.store.Clone(ctx, req.Id, req.NewName, "system")
    if err != nil {
        s.logger.Error("Failed to clone game DNA", zap.Error(err))
        return nil, fmt.Errorf("failed to clone game DNA: %w", err)
    }

    s.logger.Info("Game DNA cloned", zap.String("original_id", req.Id), zap.String("cloned_id", cloned.Id))

    return &pb.GameDNAResponse{
        GameDna: cloned,
        Message: "Game DNA cloned successfully",
    }, nil
}
