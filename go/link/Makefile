build:
	@#go build -ldflags '-X main.version=1.0' main.go
	@#go build -ldflags "-X main.version=`git rev-parse HEAD`" main.go
	#go build -ldflags "-X 'main.version=1.0 (Beta)'" main.go
	go build -ldflags "-X 'main.version=`date`'" main.go
