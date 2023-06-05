.PHONY: all test

all:
	cargo build

test:
	./tests.py

clean:
	rm -rf target
