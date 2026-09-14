.PHONY: all test

all:
	cargo build --release

test:
	./tests.py

clean:
	rm -rf target
