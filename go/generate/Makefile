.PHONY: build

VERSION := `git describe --tags --always`
PKGS := `go list ./...`

build: gen fmt
	go build -ldflags "-X main.version=$(VERSION)" -o build/main cmd/generate/main.go

fmt:
	@for pkg in $(PKGS); do \
		echo $$pkg; \
		go fmt $$pkg; \
	done

gen:
	go generate internal/project/project.go

clean:
	rm -rf build
