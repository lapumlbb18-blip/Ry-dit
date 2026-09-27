#!/usr/bin/env python3
"""Parsea la salida cruda de `cargo test --workspace --no-fail-fast` a YAML.

Uso: python3 parse_cargo_test.py raw-test-output.txt test-report.yaml --exit-code 0
"""
import re
import sys
import subprocess
from datetime import datetime, timezone

RESULT_RE = re.compile(
    r"^test result:\s*(?P<status>ok|FAILED)\.\s*"
    r"(?P<passed>\d+) passed;\s*(?P<failed>\d+) failed;\s*"
    r"(?P<ignored>\d+) ignored;\s*(?P<measured>\d+) measured;\s*"
    r"(?P<filtered>\d+) filtered out;\s*finished in (?P<dur>[\d.]+)s"
)
TEST_RE = re.compile(r"^test\s+(?P<name>.+?)\s+\.\.\.\s+(?P<status>ok|FAILED|ignored)$")
RUN_RE = re.compile(
    r"^\s+Running\s+(?P<target>.+?)\s+\((?P<path>target/debug/deps/(?P<stem>[^-]+)-[0-9a-f]+)\)$"
)
DOC_RE = re.compile(r"^\s+Doc-tests\s+(?P<name>\S+)\s*$")


def yq(s):
    """Cita un string como YAML double-quoted."""
    out = s.replace("\\", "\\\\").replace('"', '\\"')
    out = out.replace("\n", "\\n").replace("\t", "\\t")
    return '"' + out + '"'


def emit(obj, indent=0, lines=None):
    """Emite YAML anidado (dict/list/scalar) sin dependencias externas."""
    if lines is None:
        lines = []
    pad = "  " * indent
    if isinstance(obj, dict):
        if not obj:
            lines.append(pad + "{}")
        for k, v in obj.items():
            if isinstance(v, (dict, list)) and v:
                lines.append(f"{pad}{k}:")
                emit(v, indent + 1, lines)
            elif isinstance(v, (dict, list)):
                lines.append(f"{pad}{k}: " + ("{}" if isinstance(v, dict) else "[]"))
            elif isinstance(v, bool):
                lines.append(f"{pad}{k}: {'true' if v else 'false'}")
            elif isinstance(v, (int, float)):
                lines.append(f"{pad}{k}: {v}")
            elif v is None:
                lines.append(f"{pad}{k}: null")
            else:
                lines.append(f"{pad}{k}: {yq(str(v))}")
    elif isinstance(obj, list):
        for item in obj:
            if isinstance(item, dict):
                first = True
                for k, v in item.items():
                    prefix = pad + "- " if first else pad + "  "
                    first = False
                    if isinstance(v, (dict, list)) and v:
                        lines.append(f"{prefix}{k}:")
                        emit(v, indent + 2, lines)
                    elif isinstance(v, (dict, list)):
                        lines.append(f"{prefix}{k}: " + ("{}" if isinstance(v, dict) else "[]"))
                    elif isinstance(v, bool):
                        lines.append(f"{prefix}{k}: {'true' if v else 'false'}")
                    elif isinstance(v, (int, float)):
                        lines.append(f"{prefix}{k}: {v}")
                    elif v is None:
                        lines.append(f"{prefix}{k}: null")
                    else:
                        lines.append(f"{prefix}{k}: {yq(str(v))}")
            else:
                lines.append(f"{pad}- {yq(str(item))}")
    else:
        lines.append(pad + yq(str(obj)))
    return lines


def main():
    raw_path = sys.argv[1]
    out_path = sys.argv[2]
    exit_code = 0
    if "--exit-code" in sys.argv:
        exit_code = int(sys.argv[sys.argv.index("--exit-code") + 1])

    suites = []
    cur = None
    failures = []
    warnings = 0
    compilation_errors = []

    with open(raw_path, encoding="utf-8", errors="replace") as f:
        for line in f:
            line = line.rstrip("\n")
            if line.startswith("warning:"):
                warnings += 1
            if line.startswith("error"):
                compilation_errors.append(line.strip())

            m = RUN_RE.match(line)
            if m:
                cur = {
                    "name": m.group("stem"),
                    "kind": "unittest" if "unittests" in m.group("target") else "integration",
                    "target": m.group("target"),
                    "tests": [],
                    "passed": 0, "failed": 0, "ignored": 0, "duration_seconds": 0.0,
                }
                suites.append(cur)
                continue

            m = DOC_RE.match(line)
            if m:
                cur = {
                    "name": m.group("name"),
                    "kind": "doctest",
                    "target": "rustdoc",
                    "tests": [],
                    "passed": 0, "failed": 0, "ignored": 0, "duration_seconds": 0.0,
                }
                suites.append(cur)
                continue

            m = TEST_RE.match(line)
            if m and cur is not None:
                status = m.group("status")
                entry = {"name": m.group("name").strip(), "status": status}
                cur["tests"].append(entry)
                if status == "ok":
                    cur["passed"] += 1
                elif status == "FAILED":
                    cur["failed"] += 1
                    failures.append({"suite": cur["name"], "test": entry["name"]})
                else:
                    cur["ignored"] += 1
                continue

            m = RESULT_RE.match(line)
            if m and cur is not None:
                cur["duration_seconds"] = float(m.group("dur"))
                continue

    suites = [s for s in suites if s["tests"]]
    total_passed = sum(s["passed"] for s in suites)
    total_failed = sum(s["failed"] for s in suites)
    total_ignored = sum(s["ignored"] for s in suites)
    total_tests = total_passed + total_failed + total_ignored
    total_dur = round(sum(s["duration_seconds"] for s in suites), 2)

    try:
        now = datetime.now().astimezone().isoformat(timespec="seconds")
    except Exception:
        now = datetime.now(timezone.utc).isoformat(timespec="seconds")

    report = {
        "meta": {
            "tool": "cargo test",
            "command": "cargo test --workspace --no-fail-fast -j 4",
            "workspace": "/data/data/com.termux/files/home/Ry-dit",
            "date": now,
            "exit_code": exit_code,
            "raw_log": "raw-test-output.txt",
        },
        "summary": {
            "status": "passed" if (exit_code == 0 and not failures) else "failed",
            "suites": len(suites),
            "tests": total_tests,
            "passed": total_passed,
            "failed": total_failed,
            "ignored": total_ignored,
            "duration_seconds": total_dur,
            "warnings": warnings,
            "compilation_errors": compilation_errors,
        },
        "failures": failures,
        "suites": suites,
    }

    lines = emit(report)
    with open(out_path, "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")
    print(f"OK: {out_path} ({len(suites)} suites, {total_tests} tests, "
          f"{total_passed} passed, {total_failed} failed)")


if __name__ == "__main__":
    main()
