# jsr / ret regression

Run `python3 tests.py 026` from the repository root. The runner invokes
`generate_class.py` instead of `javac` for this test. Modern Java compilers do
not emit `jsr` or `ret`; the generator writes a version-49 (Java 5) `Main.class`
that the reference JVM can still verify and execute.

The main method initializes an integer local to zero, calls the same subroutine
twice with `jsr`, and prints `2`. The subroutine saves the return address with
`astore_2`, increments the local, and returns through `ret`. Using two call sites
checks that each invocation returns to its own continuation.

Inspect the instructions with `javap -c tests/test026/Main.class` after running
the test. Do not commit the generated `.class` file.
