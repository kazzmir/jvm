.PHONY: all test

all:
	cargo build

test:
	./tests.py
