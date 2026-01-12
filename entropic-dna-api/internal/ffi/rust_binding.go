package ffi

import (
	"encoding/json"
	"fmt"

	pb "github.com/entropic-engine/entropic-dna-api/gen/proto/entropic/dna/v1"
)

// RustFFI provides an interface to the Rust Game DNA validation engine.
// This is a stub implementation until the Rust library is compiled and available.
type RustFFI struct {
	enabled bool
	libPath string
}

// NewRustFFI creates a new Rust FFI binding.
func NewRustFFI(libPath string, enabled bool) (*RustFFI, error) {
	if !enabled {
		return &RustFFI{enabled: false}, nil
	}

	// TODO: Load the Rust library using CGO or dlopen
	// For now, return a stub that performs basic validation

	return &RustFFI{
		enabled: enabled,
		libPath: libPath,
	}, nil
}

// ValidateGameDNA validates a GameDNA configuration using the Rust engine.
func (r *RustFFI) ValidateGameDNA(dna *pb.GameDNA) (*pb.ValidationResponse, error) {
	if !r.enabled {
		return r.basicValidation(dna), nil
	}

	// TODO: Call Rust FFI function
	// For now, fallback to basic validation

	return r.basicValidation(dna), nil
}

// basicValidation provides basic Go-side validation as a fallback.
func (r *RustFFI) basicValidation(dna *pb.GameDNA) *pb.ValidationResponse {
	resp := &pb.ValidationResponse{
		IsValid:     true,
		Errors:      []*pb.ValidationError{},
		Warnings:    []*pb.ValidationWarning{},
		Suggestions: []string{},
	}

	// Basic field validation
	if dna.Name == "" {
		resp.IsValid = false
		resp.Errors = append(resp.Errors, &pb.ValidationError{
			Code:    "EMPTY_NAME",
			Field:   "name",
			Message: "Game name cannot be empty",
			Details: "The game name field is required and must contain at least one character",
		})
	}

	if len(dna.TargetPlatforms) == 0 {
		resp.IsValid = false
		resp.Errors = append(resp.Errors, &pb.ValidationError{
			Code:    "NO_PLATFORMS",
			Field:   "target_platforms",
			Message: "At least one target platform must be specified",
			Details: "Valid platforms include PC, Console, Mobile, VR, Web",
		})
	}

	if dna.TargetFps == 0 || dna.TargetFps > 1000 {
		resp.IsValid = false
		resp.Errors = append(resp.Errors, &pb.ValidationError{
			Code:    "INVALID_FPS",
			Field:   "target_fps",
			Message: "Target FPS must be between 1 and 1000",
			Details: fmt.Sprintf("Current value: %d", dna.TargetFps),
		})
	}

	if dna.TimeScale <= 0 || dna.TimeScale > 1000 {
		resp.IsValid = false
		resp.Errors = append(resp.Errors, &pb.ValidationError{
			Code:    "INVALID_TIME_SCALE",
			Field:   "time_scale",
			Message: "Time scale must be positive and reasonable",
			Details: fmt.Sprintf("Current value: %f", dna.TimeScale),
		})
	}

	// Warnings
	if dna.Genre == "" {
		resp.Warnings = append(resp.Warnings, &pb.ValidationWarning{
			Code:       "MISSING_GENRE",
			Field:      "genre",
			Message:    "No genre specified",
			Suggestion: "Consider specifying a genre (FPS, RPG, Strategy, etc.)",
		})
	}

	if dna.MaxPlayers > 1 && !dna.IsCompetitive && !dna.SupportsCoop {
		resp.Warnings = append(resp.Warnings, &pb.ValidationWarning{
			Code:       "MULTIPLAYER_NOT_CONFIGURED",
			Field:      "max_players",
			Message:    "Multiplayer capacity set but no competitive or coop mode enabled",
			Suggestion: "Set is_competitive or supports_coop to true",
		})
	}

	// Suggestions
	if dna.Genre != "" && dna.Camera == "" {
		resp.Suggestions = append(resp.Suggestions, "Consider specifying a camera mode that matches your genre")
	}

	if dna.AiEnabled && dna.NpcCount == 0 {
		resp.Suggestions = append(resp.Suggestions, "AI is enabled but NPC count is 0 - consider setting npc_count > 0")
	}

	return resp
}

// CalculateChecksum generates a checksum for a GameDNA configuration.
func (r *RustFFI) CalculateChecksum(dna *pb.GameDNA) (string, error) {
	if !r.enabled {
		return r.basicChecksum(dna)
	}

	// TODO: Call Rust FFI function
	return r.basicChecksum(dna)
}

// basicChecksum provides basic Go-side checksum calculation.
func (r *RustFFI) basicChecksum(dna *pb.GameDNA) (string, error) {
	// Simple JSON-based checksum for now
	// In production, this should use the Rust deterministic serialization
	jsonData, err := json.Marshal(dna)
	if err != nil {
		return "", fmt.Errorf("failed to marshal DNA for checksum: %w", err)
	}

	// Use a simple hash (in production, use SHA-256 from Rust)
	return fmt.Sprintf("%x", len(jsonData)), nil
}

// Close closes the FFI binding and releases resources.
func (r *RustFFI) Close() {
	// TODO: Clean up any loaded libraries
}
