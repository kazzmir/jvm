# Working with this repository

## Project layout

This is a small, incomplete JVM bytecode interpreter written in Rust (edition 2021), not a full Java runtime.

- `src/jvm/data.rs`: class-file parsing, constant pool, method descriptors, code attributes, and exception tables.
- `src/jvm/exec.rs`: opcode constants, runtime values, frames, interpreter dispatch, class loading, exception propagation, and minimal native Java classes.
- `src/jvm/exec/tests.rs`: Rust unit tests for stack manipulation.
- `src/jvm/exec/gc.rs`: single-threaded tracing cycle collector over weak allocation registrations; `gc/tests.rs` verifies reclamation and root preservation.
- `src/main.rs`: `jvm` CLI entry point.
- `src/jimage/`: separate `jimage` binary/tooling.
- `tests.py`: compares this interpreter's stdout against the installed Java JVM.
- `tests/testNNN/Main.java`: numbered Java integration tests. Nested classes are compiled alongside `Main`.

## Build and test

Run commands from the repository root. Required tools: Rust/Cargo, Python 3, and a JDK providing `javac`, `java`, and `javap`.

```sh
cargo build --bin jvm
# On a fresh checkout, if ./jvm does not exist:
ln -s target/debug/jvm jvm
python3 tests.py 021       # one test: tests/test021
python3 tests.py           # all Java tests
cargo test                # Rust unit tests
javap -c tests/test021/Main.class
git diff --check
```

- The test runner invokes `./jvm`, not `cargo run`. In the current workspace, `./jvm` is a symlink to `target/debug/jvm`; rebuilding updates it automatically. Check before replacing an existing executable/link.
- `make` builds both binaries. `make test` only invokes the Python runner; it does **not** rebuild Rust first.
- The runner recompiles Java sources when the corresponding top-level `.class` is absent or older. If nested class files are missing/stale, explicitly run `javac tests/testNNN/Main.java`.
- **Do not trust exit status alone:** `tests.py` prints failures but does not exit nonzero for output mismatches. The JVM CLI also prints interpreter errors to stdout without setting a failure exit code. Inspect the test results.
- Output comparison is byte-for-byte. Formatting, trailing newlines, and extra stdout matter. Debug logging uses stderr and is disabled by default. Pass `-v` to the JVM CLI (`./jvm -v tests/test001/Main.class`) to enable `debug!` output in either debug or release builds.

## Adding bytecode tests

1. Follow the requested scope: if asked only to create a test, create it and report the current failure; implement instructions when asked to make it pass.
2. Add `tests/testNNN/Main.java` using the next requested number. Keep tests small and deterministic, with observable output checked against Java.
3. Compile and inspect with `javap -c`. Java source syntax does not guarantee a particular opcode: constant folding and local-variable allocation affect emitted bytecode.
4. Prefer runtime local variables for conversion/arithmetic tests, rather than constant expressions the compiler can fold away.
5. Use simple supported constants when testing unrelated operations. Arbitrary float/double literals can require constant-pool support that is not yet implemented.
6. Print chars numerically when testing `caload`/`castore` or `i2c`, to avoid conflating those instructions with character printing.
7. Identify all missing prerequisite instructions, not just the requested opcode. For example, casts may also require additional local loads/stores, and comparisons require conditional branches.
8. After implementation, rebuild, run the selected test, run the full Java suite and `cargo test`, and check the diff.

Naming details:
- There is no JVM `f2f` instruction; a float-to-float cast is a no-op.
- Float/double comparisons are `fcmpl`/`fcmpg` and `dcmpl`/`dcmpg`, not a single `fcmp`/`dcmp` opcode.
- `test020` intentionally has 11,000 expanded increments to force `javac` to emit `goto_w`. Do not replace the expanded body with a compact runtime loop: that removes the long jump.

## Interpreter implementation conventions

- Add named opcode constants in `opcodes`; avoid raw opcode numbers in dispatch.
- Use the `array_types` constants and `array_types::is_supported()` for `newarray`; do not reintroduce magic type numbers such as 4, 7, or 8.
- Follow the existing style and keep changes focused. Avoid formatting the entire large interpreter file as part of a small instruction change.
- Pop binary operands in JVM order: right operand first, then left. This matters for subtraction, division, remainder, and comparisons.
- `RuntimeValue::Int` stores an `i64`, but Java ints are signed **32-bit** values. Apply the appropriate width, truncation, sign extension, and zero extension for each instruction.
- Java `char` is unsigned 16-bit, including surrogate values; use `u16`, not Rust `char`. Byte loads sign-extend; char loads zero-extend. Boolean array stores retain the low bit.
- Float operations should use `f32`, doubles `f64`. Float-to-integer conversions truncate toward zero, saturate overflow, and convert NaN to zero. Comparison variants differ in their NaN result. Preserve signed zero and infinity behavior.
- Objects and arrays use shared `Rc<RefCell<...>>` references. Duplicating/loading/returning a reference must preserve identity, not deep-copy its contents.
- GC runs at bytecode boundaries after 16 MiB of estimated allocation pressure (not process RSS). It marks globals, current locals/operand stack, suspended caller snapshots, pending exceptions, and held monitors. Unreachable objects/reference arrays have their outgoing edges cleared to reclaim cycles through reference counting. Array payload capacities and estimated object overhead drive the budget; this is not a hard heap limit or an allocation-failure retry mechanism.
- Keep newly exposed object/reference-array values registered with the heap and account for new array payloads. Calls that can execute Java must retain caller roots via `gc::CallRoots`; never collect while Rust-only temporary references are unrooted. Runtime teardown also clears remaining cycles.
- Run `python3 tests/test039/check_gc.py` to verify >3 GiB of cyclic allocation under a 128 MiB process limit. Check the `./jvm` symlink and rebuild its actual target (debug or release).
- Doubles and longs occupy **two JVM slots**, but one operand-stack `Vec` entry. Double locals reserve two entries. Stack-manipulation instructions must distinguish category-1 and category-2 values; `duplicate_two_slots` and its tests demonstrate this.
- Branch offsets are signed and relative to the instruction's starting PC. `goto_w` uses a signed 32-bit offset. Do not narrow the PC to `i16`, even for short branches in large methods.
- New array value variants must also be handled by debugging, `arraylength`, reference assignability/checkcasts, and `areturn` as appropriate. Check bounds and operand types rather than introducing panics.
- Bytecode methods must execute against their defining class's constant pool, not the caller's. The CLI loader follows available class references relative to the entry class's classpath root.
- Exceptions currently propagate through `RuntimeConst::pending_exception`. Handler lookup uses the throwing/invoking instruction's original PC, half-open protected ranges, superclass matching, and the first matching handler. Entering a handler clears the stack and pushes the exception.

## Limitations and workspace care

- Native Java classes are minimal stubs. Class-file support, method resolution, verification, interfaces, null handling, array exceptions, and JVM semantics are incomplete. Do not assume support just because the reference JVM accepts a program.
- Method maps currently use names rather than full name/descriptor keys; native printing dispatches on runtime value types. Category-2 argument/local placement and Java-compatible numeric formatting also need care beyond existing tests.
- Use the JVM specification and Java output to validate new semantics. Add targeted unit tests for stack forms and edge cases where Java source alone cannot easily produce the desired bytecode.
- Inspect `git status` before editing. Preserve unrelated user changes and untracked files.
- Build outputs, executable symlinks, `.class` files, `modules`, and `Cargo.lock` may already be present or untracked. Do not delete or stage them indiscriminately; submit source/documentation changes relevant to the task.
