package api

import (
	"context"
	"fmt"
	"net/http"

	pb "github.com/entropic-engine/entropic-dna-api/gen/proto/entropic/dna/v1"
	"github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
	"go.uber.org/zap"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

// RESTGateway provides an HTTP server that proxies to the gRPC server.
type RESTGateway struct {
	server *http.Server
	logger *zap.Logger
}

// NewRESTGateway creates a new REST gateway.
func NewRESTGateway(ctx context.Context, grpcAddr string, httpAddr string, logger *zap.Logger) (*RESTGateway, error) {
	mux := runtime.NewServeMux(
		runtime.WithErrorHandler(customHTTPError),
	)

	opts := []grpc.DialOption{grpc.WithTransportCredentials(insecure.NewCredentials())}
	if err := pb.RegisterGameDNAServiceHandlerFromEndpoint(ctx, mux, grpcAddr, opts); err != nil {
		return nil, fmt.Errorf("failed to register gateway: %w", err)
	}

	srv := &http.Server{
		Addr:    httpAddr,
		Handler: requestLoggingMiddleware(logger, mux),
	}

	return &RESTGateway{server: srv, logger: logger}, nil
}

// Start starts the HTTP server.
func (g *RESTGateway) Start() error {
	g.logger.Info("Starting REST gateway", zap.String("addr", g.server.Addr))
	return g.server.ListenAndServe()
}

// Shutdown gracefully shuts down the HTTP server.
func (g *RESTGateway) Shutdown(ctx context.Context) error {
	g.logger.Info("Shutting down REST gateway")
	return g.server.Shutdown(ctx)
}

func requestLoggingMiddleware(logger *zap.Logger, next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		logger.Info("HTTP request",
			zap.String("method", r.Method),
			zap.String("path", r.URL.Path),
			zap.String("remote", r.RemoteAddr),
		)
		next.ServeHTTP(w, r)
	})
}

func customHTTPError(ctx context.Context, mux *runtime.ServeMux, marshaler runtime.Marshaler, w http.ResponseWriter, r *http.Request, err error) {
	// Default grpc-gateway error handler already maps gRPC codes.
	runtime.DefaultHTTPErrorHandler(ctx, mux, marshaler, w, r, err)
}
