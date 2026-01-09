package main

import (
	"context"
	"fmt"
	"net"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/entropic-engine/entropic-dna-api/internal/api"
	"github.com/entropic-engine/entropic-dna-api/internal/config"
	"github.com/entropic-engine/entropic-dna-api/internal/ffi"
	"github.com/entropic-engine/entropic-dna-api/internal/storage"
	pb "github.com/entropic-engine/entropic-dna-api/gen/proto/entropic/dna/v1"
	"go.uber.org/zap"
	"google.golang.org/grpc"
	"google.golang.org/grpc/reflection"
)

func main() {
	if err := run(); err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}

func run() error {
	// Load configuration
	cfg, err := config.Load()
	if err != nil {
		return fmt.Errorf("failed to load config: %w", err)
	}

	if err := cfg.Validate(); err != nil {
		return fmt.Errorf("invalid config: %w", err)
	}

	// Initialize logger
	logger, err := initLogger(cfg.Logging)
	if err != nil {
		return fmt.Errorf("failed to init logger: %w", err)
	}
	defer logger.Sync()

	logger.Info("Starting Entropic DNA API Server",
		zap.String("grpc_addr", fmt.Sprintf("%s:%d", cfg.Server.Host, cfg.Server.GRPCPort)),
		zap.String("http_addr", fmt.Sprintf("%s:%d", cfg.Server.Host, cfg.Server.HTTPPort)),
	)

	// Initialize storage
	var store storage.Store
	if cfg.Database.URL != "" && cfg.Database.URL != "memory" {
		logger.Info("Connecting to PostgreSQL", zap.String("url", cfg.Database.URL))
		pgStore, err := storage.NewPostgresStore(cfg.Database.URL)
		if err != nil {
			if cfg.Database.UseFallback {
				logger.Warn("Failed to connect to PostgreSQL, falling back to memory storage", zap.Error(err))
				store = storage.NewMemoryStore()
			} else {
				return fmt.Errorf("failed to connect to database: %w", err)
			}
		} else {
			// Run migrations
			logger.Info("Running database migrations")
			ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
			if err := storage.Migrate(ctx, pgStore.DB()); err != nil {
				cancel()
				return fmt.Errorf("failed to run migrations: %w", err)
			}
			cancel()
			store = pgStore
		}
	} else {
		logger.Info("Using in-memory storage")
		store = storage.NewMemoryStore()
	}
	defer store.Close()

	// Initialize Rust FFI
	logger.Info("Initializing Rust FFI", zap.String("lib_path", cfg.Rust.LibPath), zap.Bool("enabled", cfg.Rust.Enabled))
	rust, err := ffi.NewRustFFI(cfg.Rust.LibPath, cfg.Rust.Enabled)
	if err != nil {
		return fmt.Errorf("failed to init Rust FFI: %w", err)
	}
	defer rust.Close()

	// Create gRPC server
	grpcServer := grpc.NewServer()
	svcServer := api.NewGameDNAServiceServer(store, rust, logger)
	pb.RegisterGameDNAServiceServer(grpcServer, svcServer)
	reflection.Register(grpcServer)

	// Start gRPC server
	grpcAddr := fmt.Sprintf("%s:%d", cfg.Server.Host, cfg.Server.GRPCPort)
	lis, err := net.Listen("tcp", grpcAddr)
	if err != nil {
		return fmt.Errorf("failed to listen on %s: %w", grpcAddr, err)
	}

	go func() {
		logger.Info("gRPC server listening", zap.String("addr", grpcAddr))
		if err := grpcServer.Serve(lis); err != nil {
			logger.Error("gRPC server error", zap.Error(err))
		}
	}()

	// Start REST gateway
	ctx := context.Background()
	httpAddr := fmt.Sprintf("%s:%d", cfg.Server.Host, cfg.Server.HTTPPort)
	gateway, err := api.NewRESTGateway(ctx, grpcAddr, httpAddr, logger)
	if err != nil {
		return fmt.Errorf("failed to create REST gateway: %w", err)
	}

	go func() {
		logger.Info("REST gateway listening", zap.String("addr", httpAddr))
		if err := gateway.Start(); err != nil && err != context.Canceled {
			logger.Error("REST gateway error", zap.Error(err))
		}
	}()

	// Wait for shutdown signal
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, os.Interrupt, syscall.SIGTERM)
	<-sigChan

	logger.Info("Shutting down gracefully...")

	// Graceful shutdown
	shutdownCtx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	if err := gateway.Shutdown(shutdownCtx); err != nil {
		logger.Error("Error shutting down REST gateway", zap.Error(err))
	}

	grpcServer.GracefulStop()

	logger.Info("Shutdown complete")
	return nil
}

func initLogger(cfg config.LoggingConfig) (*zap.Logger, error) {
	var logConfig zap.Config

	if cfg.Format == "json" {
		logConfig = zap.NewProductionConfig()
	} else {
		logConfig = zap.NewDevelopmentConfig()
	}

	switch cfg.Level {
	case "debug":
		logConfig.Level = zap.NewAtomicLevelAt(zap.DebugLevel)
	case "info":
		logConfig.Level = zap.NewAtomicLevelAt(zap.InfoLevel)
	case "warn":
		logConfig.Level = zap.NewAtomicLevelAt(zap.WarnLevel)
	case "error":
		logConfig.Level = zap.NewAtomicLevelAt(zap.ErrorLevel)
	default:
		logConfig.Level = zap.NewAtomicLevelAt(zap.InfoLevel)
	}

	return logConfig.Build()
}
