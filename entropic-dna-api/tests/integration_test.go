package tests

import (
	"context"
	"testing"

	pb "github.com/entropic-engine/entropic-dna-api/gen/proto/entropic/dna/v1"
	"github.com/entropic-engine/entropic-dna-api/internal/storage"
)

func TestMemoryStoreCRUD(t *testing.T) {
	ctx := context.Background()
	store := storage.NewMemoryStore()
	defer store.Close()

	// Create
	dna := &pb.GameDNA{
		Name:            "Test Game",
		Version:         "0.1.0",
		Genre:           "FPS",
		Camera:          "Perspective3D",
		TargetPlatforms: []string{"PC"},
		TargetFps:       60,
		TimeScale:       1.0,
	}

	created, err := store.Create(ctx, dna)
	if err != nil {
		t.Fatalf("Create failed: %v", err)
	}
	if created.Id == "" {
		t.Fatal("Created ID is empty")
	}
	if created.Name != "Test Game" {
		t.Errorf("Expected name 'Test Game', got '%s'", created.Name)
	}

	// Read
	read, err := store.Read(ctx, created.Id)
	if err != nil {
		t.Fatalf("Read failed: %v", err)
	}
	if read.Name != created.Name {
		t.Errorf("Expected name '%s', got '%s'", created.Name, read.Name)
	}

	// Update
	read.Genre = "RPG"
	updated, err := store.Update(ctx, read)
	if err != nil {
		t.Fatalf("Update failed: %v", err)
	}
	if updated.Genre != "RPG" {
		t.Errorf("Expected genre 'RPG', got '%s'", updated.Genre)
	}

	// List
	items, total, err := store.List(ctx, storage.ListFilters{}, storage.Pagination{Page: 1, PageSize: 10})
	if err != nil {
		t.Fatalf("List failed: %v", err)
	}
	if total != 1 {
		t.Errorf("Expected 1 item, got %d", total)
	}
	if len(items) != 1 {
		t.Errorf("Expected 1 item in list, got %d", len(items))
	}

	// Delete
	err = store.Delete(ctx, created.Id)
	if err != nil {
		t.Fatalf("Delete failed: %v", err)
	}

	// Verify deletion
	_, err = store.Read(ctx, created.Id)
	if err == nil {
		t.Error("Expected error reading deleted item, got nil")
	}
}

func TestMemoryStoreVersioning(t *testing.T) {
	ctx := context.Background()
	store := storage.NewMemoryStore()
	defer store.Close()

	dna := &pb.GameDNA{
		Name:            "Versioned Game",
		Version:         "0.1.0",
		Genre:           "FPS",
		TargetPlatforms: []string{"PC"},
		TargetFps:       60,
		TimeScale:       1.0,
	}

	created, err := store.Create(ctx, dna)
	if err != nil {
		t.Fatalf("Create failed: %v", err)
	}

	// Update to create version 2
	created.Genre = "RPG"
	_, err = store.Update(ctx, created)
	if err != nil {
		t.Fatalf("Update failed: %v", err)
	}

	// Get version history
	versions, err := store.GetVersionHistory(ctx, created.Id)
	if err != nil {
		t.Fatalf("GetVersionHistory failed: %v", err)
	}
	if len(versions) != 2 {
		t.Errorf("Expected 2 versions, got %d", len(versions))
	}

	// Rollback to version 1
	rolledBack, err := store.RollbackToVersion(ctx, created.Id, 1, "test")
	if err != nil {
		t.Fatalf("Rollback failed: %v", err)
	}
	if rolledBack.Genre != "FPS" {
		t.Errorf("Expected genre 'FPS' after rollback, got '%s'", rolledBack.Genre)
	}

	// Verify version count increased (rollback creates new version)
	versions, err = store.GetVersionHistory(ctx, created.Id)
	if err != nil {
		t.Fatalf("GetVersionHistory failed: %v", err)
	}
	if len(versions) != 3 {
		t.Errorf("Expected 3 versions after rollback, got %d", len(versions))
	}
}

func TestMemoryStorePublish(t *testing.T) {
	ctx := context.Background()
	store := storage.NewMemoryStore()
	defer store.Close()

	dna := &pb.GameDNA{
		Name:            "Publishable Game",
		Version:         "0.1.0",
		Genre:           "FPS",
		TargetPlatforms: []string{"PC"},
		TargetFps:       60,
		TimeScale:       1.0,
	}

	created, err := store.Create(ctx, dna)
	if err != nil {
		t.Fatalf("Create failed: %v", err)
	}

	// Publish (lock)
	published, err := store.PublishVersion(ctx, created.Id, "test")
	if err != nil {
		t.Fatalf("Publish failed: %v", err)
	}
	if !published.IsLocked {
		t.Error("Expected config to be locked after publish")
	}

	// Try to update locked config
	published.Genre = "RPG"
	_, err = store.Update(ctx, published)
	if err == nil {
		t.Error("Expected error updating locked config, got nil")
	}
}

func TestMemoryStoreClone(t *testing.T) {
	ctx := context.Background()
	store := storage.NewMemoryStore()
	defer store.Close()

	dna := &pb.GameDNA{
		Name:            "Original Game",
		Version:         "0.1.0",
		Genre:           "FPS",
		TargetPlatforms: []string{"PC"},
		TargetFps:       60,
		TimeScale:       1.0,
	}

	created, err := store.Create(ctx, dna)
	if err != nil {
		t.Fatalf("Create failed: %v", err)
	}

	// Clone
	cloned, err := store.Clone(ctx, created.Id, "Cloned Game", "test")
	if err != nil {
		t.Fatalf("Clone failed: %v", err)
	}

	if cloned.Id == created.Id {
		t.Error("Cloned ID should be different from original")
	}
	if cloned.Name != "Cloned Game" {
		t.Errorf("Expected name 'Cloned Game', got '%s'", cloned.Name)
	}
	if cloned.Genre != created.Genre {
		t.Errorf("Expected cloned genre to match original '%s', got '%s'", created.Genre, cloned.Genre)
	}
	if cloned.IsLocked {
		t.Error("Cloned config should not be locked")
	}

	// Verify both exist
	items, total, err := store.List(ctx, storage.ListFilters{}, storage.Pagination{Page: 1, PageSize: 10})
	if err != nil {
		t.Fatalf("List failed: %v", err)
	}
	if total != 2 {
		t.Errorf("Expected 2 items, got %d", total)
	}
	if len(items) != 2 {
		t.Errorf("Expected 2 items in list, got %d", len(items))
	}
}
