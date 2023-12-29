# Makefile for the go project.

.PHONEY: all format main test

main: format build test

all: clean main test

format:
	go fmt ./...

build: main.go
	mkdir -p build
	go build -o build/native-shell .

test:
	go test ./...

clean:
	test -d build && rm -r build
