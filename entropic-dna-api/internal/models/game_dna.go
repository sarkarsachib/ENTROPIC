package models

import (
	"time"

	pb "github.com/entropic-engine/entropic-dna-api/gen/proto/entropic/dna/v1"
)

// GameDNA represents a game configuration in the database
type GameDNA struct {
	ID             string
	Name           string
	Version        string
	CreatedAt      time.Time
	LastModified   time.Time
	CreatedBy      string
	Checksum       string
	IsLocked       bool
	Data           string // JSON representation
	Tags           []string
	Genre          string
	TargetPlatforms []string
}

// GameDNAVersion represents a version snapshot in version history
type GameDNAVersion struct {
	ID         int64
	ConfigID   string
	VersionNum int64
	Data       string // JSON representation
	Checksum   string
	CreatedAt  time.Time
	CreatedBy  string
}

// ToProto converts the model to protobuf representation
func (g *GameDNA) ToProto() (*pb.GameDNA, error) {
	return &pb.GameDNA{
		Id:           g.ID,
		Name:         g.Name,
		Version:      g.Version,
		CreatedAt:    g.CreatedAt.Format(time.RFC3339),
		LastModified: g.LastModified.Format(time.RFC3339),
		CreatedBy:    g.CreatedBy,
		Checksum:     g.Checksum,
		IsLocked:     g.IsLocked,
		Genre:        g.Genre,
		Tags:         g.Tags,
		TargetPlatforms: g.TargetPlatforms,
	}, nil
}

// FromProto creates a model from protobuf representation
func FromProto(pb *pb.GameDNA) (*GameDNA, error) {
	createdAt, _ := time.Parse(time.RFC3339, pb.CreatedAt)
	lastModified, _ := time.Parse(time.RFC3339, pb.LastModified)
	
	if createdAt.IsZero() {
		createdAt = time.Now()
	}
	if lastModified.IsZero() {
		lastModified = time.Now()
	}

	return &GameDNA{
		ID:              pb.Id,
		Name:            pb.Name,
		Version:         pb.Version,
		CreatedAt:       createdAt,
		LastModified:    lastModified,
		CreatedBy:       pb.CreatedBy,
		Checksum:        pb.Checksum,
		IsLocked:        pb.IsLocked,
		Genre:           pb.Genre,
		Tags:            pb.Tags,
		TargetPlatforms: pb.TargetPlatforms,
	}, nil
}
