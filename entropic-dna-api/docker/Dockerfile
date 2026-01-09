# Stage 1: Build Go binary
FROM golang:1.22-alpine AS builder
WORKDIR /app
COPY . .
RUN go mod download
RUN go build -o server cmd/server/main.go

# Stage 2: Runtime
FROM alpine:latest
RUN apk add --no-cache ca-certificates postgresql-client
COPY --from=builder /app/server /usr/local/bin/
COPY --from=builder /app/internal/storage/migrations /app/migrations
COPY --from=builder /app/gen/openapi /app/openapi

EXPOSE 50051 8080
CMD ["server"]
