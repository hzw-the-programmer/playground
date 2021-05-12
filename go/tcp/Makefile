.PHONY: build clean test
PKGS := $(shell go list ./... | grep -v /vendor/)
VERSION := $(shell git describe --always | sed -e "s/^v//")

build:
	@echo "Compiling source"
	@for pkg in $(PKGS); do \
		go fmt $$pkg; \
	done
	@mkdir -p build
	go build $(GO_EXTRA_BUILD_ARGS) -ldflags "-s -w -X main.version=$(VERSION)" -o build/iotserver cmd/iotserver/main.go

clean:
	@echo "Cleaning up workspace"
	@rm -rf build

test:
	@echo "Running tests"
	@rm -f coverage.out
	@for pkg in $(PKGS); do \
		go fmt $$pkg; \
		golint $$pkg; \
	done
	@go vet $(PKGS)
	@go test -cover -v $(PKGS) --coverprofile coverage.out
