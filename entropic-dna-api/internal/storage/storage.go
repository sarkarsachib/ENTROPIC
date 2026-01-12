package storage

import (
	"context"

	pb "github.com/entropic-engine/entropic-dna-api/gen/proto/entropic/dna/v1"
)

// ListFilters provides basic filtering for list calls.
type ListFilters struct {
	Tags       []string
	Genre      string
	NameFilter string
}

// Pagination provides pagination for list calls.
type Pagination struct {
	Page     int32
	PageSize int32
}

// VersionInfo represents a version snapshot.
type VersionInfo struct {
	VersionNum int64
	Checksum   string
	CreatedAt  string
	CreatedBy  string
	Data       *pb.GameDNA
}

// Store is the persistence interface for GameDNA.
type Store interface {
	Create(ctx context.Context, dna *pb.GameDNA) (*pb.GameDNA, error)
	Read(ctx context.Context, id string) (*pb.GameDNA, error)
	Update(ctx context.Context, dna *pb.GameDNA) (*pb.GameDNA, error)
	Delete(ctx context.Context, id string) error
	List(ctx context.Context, filters ListFilters, pagination Pagination) ([]*pb.GameDNA, int32, error)

	GetVersionHistory(ctx context.Context, configID string) ([]*VersionInfo, error)
	RollbackToVersion(ctx context.Context, configID string, versionNum int64, actor string) (*pb.GameDNA, error)
	PublishVersion(ctx context.Context, configID string, actor string) (*pb.GameDNA, error)
	Clone(ctx context.Context, id string, newName string, actor string) (*pb.GameDNA, error)

	Close()
}
