package storage

import "errors"

var (
	// ErrNotFound indicates the requested entity does not exist.
	ErrNotFound = errors.New("not found")
	// ErrLocked indicates the config is locked (immutable).
	ErrLocked = errors.New("locked")
	// ErrConflict indicates a constraint violation (e.g., unique name+version).
	ErrConflict = errors.New("conflict")
)
