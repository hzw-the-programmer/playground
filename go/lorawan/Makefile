.PHONY: lint test

PKGS := $(shell go list ./...)

test: lint
	go test -cover -v ./...

lint:
	go fmt $(PKGS)
	golint $(PKGS)
	go vet $(PKGS)
