
.DEFAULT_GOAL := test-all

.PHONY: test-all
test-all:
	go test ./...

# usage: make test N=2
N :=
.PHONY: test
test:
	go test ./$(N)_*
