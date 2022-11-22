

clean:
	rm -rf target/go-sample

build:
	CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -o target/go-sample cmd/sample/main.go