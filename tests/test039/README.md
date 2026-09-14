# Cyclic garbage reclamation stress test

`Main.java` creates 100,000 objects with 8,192-element int arrays: over 3 GiB
of cumulative array payload. Every object has a strong `self = this` reference.
A ring retains only 64 recent objects (~2 MiB), plus one long-lived anchor.
Replacing a ring entry leaves an unreachable self-cycle that retains its array.
The program never breaks these cycles. Plain reference counting cannot reclaim
them; tracing or another cycle-collection mechanism is required.

Checks verify the self-references and both ends of retained arrays, including
the anchor, to catch premature reclamation/corruption of reachable objects.

Run the bounded-memory assertion (Linux/Unix):

```sh
cargo build --bin jvm
python3 tests/test039/check_gc.py
# Optional alternative binary:
python3 tests/test039/check_gc.py target/release/jvm
```

The helper compiles the fixture, checks Java with a 32 MiB heap, and checks the
interpreter under a 128 MiB address-space limit and a 120-second timeout. It
checks both exit status and exact stdout, since interpreter errors can return
exit status zero.

The interpreter's tracing cycle collector should pass this test. The ordinary
`python3 tests.py 039` output regression also works, but the memory-limited
helper is the reclamation assertion: a leaking implementation could consume
over 3 GiB and still print success without the limit.

This tests bounded memory under cyclic allocation churn, not weak references,
finalizers, or exact collection timing. It deliberately does not call System.gc().
