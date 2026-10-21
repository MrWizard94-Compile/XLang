#!/usr/bin/env python3
"""Compile every top-level examples/*.ae file and run the grant-free ones.

Uses the aether binary already built in target/debug or target/release.
M14 host I/O and M21 foreign weaves are compiled and not run: they refuse
without an explicit operator grant. The M8 host pilot uses pure fixtures and
is run.
"""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXAMPLES = ROOT / "examples"

# Source markers for programs that need --grant-read/--grant-write or --grant-lib.
GRANT_MARKERS = (
    "foreign weave",
    "host weave read_text",
    "host weave write_text",
)


def needs_host_grant(source: str) -> bool:
    return any(marker in source for marker in GRANT_MARKERS)


def aether_binary() -> Path:
    name = "aether.exe" if sys.platform == "win32" else "aether"
    candidates = [
        ROOT / "target" / "debug" / name,
        ROOT / "target" / "release" / name,
    ]
    for path in candidates:
        if path.is_file():
            return path
    searched = ", ".join(str(path) for path in candidates)
    raise SystemExit(f"built aether CLI not found ({searched})")


def run_aether(binary: Path, args: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [str(binary), *args],
        cwd=ROOT,
        text=True,
        capture_output=True,
        check=False,
    )


def show_failure(label: str, result: subprocess.CompletedProcess[str]) -> None:
    print(f"{label} (exit {result.returncode})", file=sys.stderr)
    if result.stdout:
        print(result.stdout, file=sys.stderr)
    if result.stderr:
        print(result.stderr, file=sys.stderr)


def main() -> int:
    binary = aether_binary()
    sources = sorted(path for path in EXAMPLES.glob("*.ae") if path.is_file())
    if not sources:
        print(f"no examples/*.ae under {EXAMPLES}", file=sys.stderr)
        return 1

    out_dir = ROOT / "target" / "examples-smoke"
    out_dir.mkdir(parents=True, exist_ok=True)

    compiled = 0
    ran = 0
    skipped = 0
    failed = False
    for source in sources:
        artifact = out_dir / f"{source.stem}.aeth"
        compiled_result = run_aether(
            binary,
            ["compile", str(source), "--output", str(artifact)],
        )
        if compiled_result.returncode != 0:
            show_failure(f"compile failed: {source.name}", compiled_result)
            failed = True
            continue
        compiled += 1

        text = source.read_text(encoding="utf-8")
        if needs_host_grant(text):
            print(f"compiled, skipped run (needs a host grant): {source.name}")
            skipped += 1
            continue

        ran_result = run_aether(binary, ["run", str(artifact)])
        if ran_result.returncode != 0:
            show_failure(f"run failed: {source.name}", ran_result)
            failed = True
            continue
        ran += 1
        print(f"ok {source.name}")

    print(
        f"examples smoke: {compiled} compiled, {ran} ran, "
        f"{skipped} grant-skipped, {len(sources)} total"
    )
    if failed or compiled != len(sources):
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
