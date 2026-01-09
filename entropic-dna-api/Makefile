.PHONY: help build run test clean proto docker migrate

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: ## Build the server binary
	@echo "Building server..."
	@export PATH=$$PATH:/usr/local/go/bin && go build -o bin/server ./cmd/server

run: ## Run the server
	@echo "Running server..."
	@export PATH=$$PATH:/usr/local/go/bin && go run ./cmd/server/main.go

test: ## Run tests
	@echo "Running tests..."
	@export PATH=$$PATH:/usr/local/go/bin && go test -v ./...

clean: ## Clean build artifacts
	@echo "Cleaning..."
	@rm -rf bin/
	@rm -rf gen/

proto: ## Generate protobuf code
	@echo "Generating protobuf code..."
	@export PATH=$$PATH:/usr/local/go/bin:$$(go env GOPATH)/bin && cd proto && buf generate

docker-build: ## Build Docker image
	@echo "Building Docker image..."
	@docker build -t entropic-dna-api:latest -f docker/Dockerfile .

docker-up: ## Start services with docker-compose
	@echo "Starting services..."
	@docker-compose up -d

docker-down: ## Stop services with docker-compose
	@echo "Stopping services..."
	@docker-compose down

docker-logs: ## Show docker-compose logs
	@docker-compose logs -f

deps: ## Install Go dependencies
	@export PATH=$$PATH:/usr/local/go/bin && go mod tidy && go mod download

install-tools: ## Install required tools
	@echo "Installing tools..."
	@export PATH=$$PATH:/usr/local/go/bin && \
	go install github.com/bufbuild/buf/cmd/buf@v1.28.0 && \
	go install google.golang.org/protobuf/cmd/protoc-gen-go@v1.32.0 && \
	go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@v1.3.0 && \
	go install github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-grpc-gateway@v2.19.1 && \
	go install github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-openapiv2@v2.19.1

lint: ## Run linter
	@export PATH=$$PATH:/usr/local/go/bin && \
	if command -v golangci-lint >/dev/null 2>&1; then \
		golangci-lint run; \
	else \
		echo "golangci-lint not installed, skipping..."; \
	fi

.DEFAULT_GOAL := help
