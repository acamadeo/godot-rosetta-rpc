#!/usr/bin/env python3
"""
Installs `RpcRuntime.gd` into a target Godot project and registers it as an
autoload in that project's `project.godot`.
"""

import argparse
import re
import shutil
from pathlib import Path

from util import read_autoload_value, AUTOLOAD_KEY

# Path within this codebase of the file to copy over.
FRAMEWORK_ROOT = Path(__file__).resolve().parent.parent
RPC_RUNTIME_SOURCE = FRAMEWORK_ROOT / "godot" / "RpcRuntime.gd"


def install(target_project: Path, dest_dir: str | None) -> None:
    if not RPC_RUNTIME_SOURCE.is_file():
        raise FileNotFoundError(f"RpcRuntime.gd not found at {RPC_RUNTIME_SOURCE}")

    project_godot = target_project / "project.godot"
    if not project_godot.is_file():
        raise FileNotFoundError(f"no project.godot found at {project_godot}")

    resolved_dest_dir = _resolve_dest_dir(project_godot, target_project, dest_dir)
    dest_subdir = target_project / resolved_dest_dir
    dest_subdir.mkdir(parents=True, exist_ok=True)
    dest_file = dest_subdir / "RpcRuntime.gd"

    # Idempotent across a changed --dest-dir: remove any previously installed
    # copy elsewhere in the project before writing the new one.
    _remove_previous_copy(target_project, project_godot)
    shutil.copyfile(RPC_RUNTIME_SOURCE, dest_file)
    print(f"Installed RpcRuntime.gd -> {dest_file}")

    autoload_path = dest_file.relative_to(target_project).as_posix()
    _set_autoload(project_godot, AUTOLOAD_KEY, f"*res://{autoload_path}")
    print(f"Registered autoload '{AUTOLOAD_KEY}' in {project_godot}")


def _resolve_dest_dir(
    project_godot: Path, target_project: Path, dest_dir: str | None
) -> str:
    """
    Resolves where to place `RpcRuntime.gd`. If `--dest-dir` is specified, place
    it there. Otherwise, place it wherever it's already installed (per
    `project.godot`).

    This means running install.py with no arguments simply refreshes the file
    in-place, instead of installing it in at the project root.
    """
    if dest_dir is not None:
        return dest_dir
    existing = read_autoload_value(project_godot, AUTOLOAD_KEY)
    if existing is not None:
        res_path = existing.lstrip("*")
        if res_path.startswith("res://"):
            existing_file = target_project / res_path[len("res://") :]
            return existing_file.parent.relative_to(target_project).as_posix()
    return "."


def _remove_previous_copy(target_project: Path, project_godot: Path) -> None:
    existing = read_autoload_value(project_godot, AUTOLOAD_KEY)
    if existing is None:
        return
    res_path = existing.lstrip("*")
    if not res_path.startswith("res://"):
        return
    previous_file = target_project / res_path[len("res://") :]
    if previous_file.is_file():
        previous_file.unlink()


def _set_autoload(project_godot: Path, key: str, value: str) -> None:
    """
    Inserts or replaces `key="value"` inside the `[autoload]` section.

    `configparser` is not used as `configparser.write()` drops `;`-comments and 
    blank-line layout. Also avoids adding dependencies.
    """
    lines = project_godot.read_text().splitlines(keepends=True)
    entry = f'{key}="{value}"\n'
    key_pattern = re.compile(rf"^{re.escape(key)}=")

    autoload_idx = next(
        (i for i, l in enumerate(lines) if l.strip() == "[autoload]"), None
    )

    # Create [autoload] section if not present.
    if autoload_idx is None:
        if lines and not lines[-1].endswith("\n"):
            lines[-1] += "\n"
        lines += ["\n[autoload]\n\n", entry]
        project_godot.write_text("".join(lines))
        return

    section_end = next(
        (
            i
            for i in range(autoload_idx + 1, len(lines))
            if lines[i].strip().startswith("[") and lines[i].strip() != "[autoload]"
        ),
        len(lines),
    )
    existing_idx = next(
        (
            i
            for i in range(autoload_idx + 1, section_end)
            if key_pattern.match(lines[i].strip())
        ),
        None,
    )

    if existing_idx is not None:
        lines[existing_idx] = entry
    else:
        # Insert right after the section's last non-blank line, in order to
        # preserve any gap between this section and the next one.
        last_content_idx = next(
            (i for i in range(section_end - 1, autoload_idx, -1) if lines[i].strip()),
            autoload_idx,
        )
        lines.insert(last_content_idx + 1, entry)

    project_godot.write_text("".join(lines))


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "target_project", type=Path, help="path to the target Godot project"
    )
    parser.add_argument(
        "--dest-dir",
        default=None,
        help=(
            "directory (relative to target_project) to place RpcRuntime.gd in "
            "(default: wherever it's already installed, or the project root "
            "for a fresh install)"
        ),
    )
    args = parser.parse_args()
    install(args.target_project.resolve(), args.dest_dir)


if __name__ == "__main__":
    main()
