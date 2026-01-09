package config

import (
	"fmt"
	"os"
	"strconv"
	"strings"

	"gopkg.in/yaml.v3"
)

// Config represents the application configuration
type Config struct {
	Server   ServerConfig   `yaml:"server"`
	Database DatabaseConfig `yaml:"database"`
	Rust     RustConfig     `yaml:"rust"`
	Logging  LoggingConfig  `yaml:"logging"`
}

// ServerConfig contains server-related settings
type ServerConfig struct {
	GRPCPort int    `yaml:"grpc_port"`
	HTTPPort int    `yaml:"http_port"`
	Host     string `yaml:"host"`
}

// DatabaseConfig contains database-related settings
type DatabaseConfig struct {
	URL            string `yaml:"url"`
	MaxConnections int    `yaml:"max_connections"`
	SSLMode        string `yaml:"ssl_mode"`
	UseFallback    bool   `yaml:"use_fallback"` // Use in-memory if PostgreSQL unavailable
}

// RustConfig contains Rust FFI-related settings
type RustConfig struct {
	LibPath string `yaml:"lib_path"` // Path to compiled Rust library
	Enabled bool   `yaml:"enabled"`  // Enable/disable Rust validation
}

// LoggingConfig contains logging-related settings
type LoggingConfig struct {
	Level  string `yaml:"level"`  // debug, info, warn, error
	Format string `yaml:"format"` // json, console
}

// DefaultConfig returns a Config with sensible defaults
func DefaultConfig() *Config {
	return &Config{
		Server: ServerConfig{
			GRPCPort: 50051,
			HTTPPort: 8080,
			Host:     "0.0.0.0",
		},
		Database: DatabaseConfig{
			URL:            "postgres://entropic:entropic@localhost:5432/game_dna?sslmode=disable",
			MaxConnections: 25,
			SSLMode:        "disable",
			UseFallback:    true,
		},
		Rust: RustConfig{
			LibPath: "./lib/libentropic_dna_core.so",
			Enabled: false, // Disabled by default since Rust lib may not be available
		},
		Logging: LoggingConfig{
			Level:  "info",
			Format: "console",
		},
	}
}

// Load loads configuration from environment variables and optional config file
func Load() (*Config, error) {
	cfg := DefaultConfig()

	// Try to load from config file if specified
	if configPath := os.Getenv("CONFIG_FILE"); configPath != "" {
		data, err := os.ReadFile(configPath)
		if err != nil {
			return nil, fmt.Errorf("failed to read config file: %w", err)
		}

		if err := yaml.Unmarshal(data, cfg); err != nil {
			return nil, fmt.Errorf("failed to parse config file: %w", err)
		}
	}

	// Override with environment variables
	if dbURL := os.Getenv("DATABASE_URL"); dbURL != "" {
		cfg.Database.URL = dbURL
	}
	if grpcPort := os.Getenv("GRPC_PORT"); grpcPort != "" {
		if port, err := strconv.Atoi(grpcPort); err == nil {
			cfg.Server.GRPCPort = port
		}
	}
	if httpPort := os.Getenv("HTTP_PORT"); httpPort != "" {
		if port, err := strconv.Atoi(httpPort); err == nil {
			cfg.Server.HTTPPort = port
		}
	}
	if host := os.Getenv("SERVER_HOST"); host != "" {
		cfg.Server.Host = host
	}
	if libPath := os.Getenv("RUST_LIB_PATH"); libPath != "" {
		cfg.Rust.LibPath = libPath
	}
	if rustEnabled := os.Getenv("RUST_ENABLED"); rustEnabled != "" {
		cfg.Rust.Enabled = strings.ToLower(rustEnabled) == "true"
	}
	if logLevel := os.Getenv("LOG_LEVEL"); logLevel != "" {
		cfg.Logging.Level = logLevel
	}
	if logFormat := os.Getenv("LOG_FORMAT"); logFormat != "" {
		cfg.Logging.Format = logFormat
	}
	if useFallback := os.Getenv("DATABASE_USE_FALLBACK"); useFallback != "" {
		cfg.Database.UseFallback = strings.ToLower(useFallback) == "true"
	}

	return cfg, nil
}

// Validate validates the configuration
func (c *Config) Validate() error {
	if c.Server.GRPCPort <= 0 || c.Server.GRPCPort > 65535 {
		return fmt.Errorf("invalid gRPC port: %d", c.Server.GRPCPort)
	}
	if c.Server.HTTPPort <= 0 || c.Server.HTTPPort > 65535 {
		return fmt.Errorf("invalid HTTP port: %d", c.Server.HTTPPort)
	}
	if c.Server.Host == "" {
		return fmt.Errorf("server host cannot be empty")
	}
	if c.Database.MaxConnections <= 0 {
		return fmt.Errorf("max connections must be positive")
	}
	if c.Logging.Level == "" {
		c.Logging.Level = "info"
	}
	if c.Logging.Format == "" {
		c.Logging.Format = "console"
	}
	return nil
}
