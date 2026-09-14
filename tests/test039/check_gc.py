#!/usr/bin/env python3
"""Linux/Unix bounded-memory reclamation check; run from any directory."""
import pathlib
import resource
import subprocess
import sys

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parent.parent
EXPECTED = b"allocation stress passed\n"
LIMIT = 128 * 1024 * 1024


def limit_memory():
    resource.setrlimit(resource.RLIMIT_AS, (LIMIT, LIMIT))


def check(command, **kwargs):
    result = subprocess.run(command, capture_output=True, timeout=120, **kwargs)
    if result.returncode != 0 or result.stdout != EXPECTED:
        raise RuntimeError(
            f"{command}: exit={result.returncode}, stdout={result.stdout!r}, "
            f"stderr={result.stderr[-2000:]!r}"
        )


if __name__ == "__main__":
    binary = pathlib.Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else ROOT / "jvm"
    subprocess.run(["javac", str(HERE / "Main.java")], check=True)
    # HotSpot reserves substantial virtual address space; constrain its Java
    # heap instead of applying the interpreter's process-address-space limit.
    check(["java", "-Xmx32m", "-cp", str(HERE), "Main"])
    check([str(binary), str(HERE / "Main.class")], preexec_fn=limit_memory)
    print("PASS: >3 GiB allocated with JVM process limited to 128 MiB")
