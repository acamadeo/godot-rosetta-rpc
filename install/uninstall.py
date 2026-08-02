#!/usr/bin/env python3
"""
Reverses install.py: deletes RpcRuntime.gd from a target Godot project,
removes its autoload registration from that project's project.godot, and
removes the [rosetta_rpc] section.
"""

import argparse
import re
from pathlib import Path

from util import read_autoload_value, remove_config_section, AUTOLOAD_KEY


# Deletes `RpcRuntime.gd` file (wherever specified in `project.godot`), and
# removes the autoload entry from `project.godot`.
def uninstall(target_project: Path) -> None:
    project_godot = target_project / "project.godot"
    if not project_godot.is_file():
        print(f"No project.godot found at {project_godot}; nothing to do.")
        return

    existing = read_autoload_value(project_godot, "autoload", AUTOLOAD_KEY)
    if existing is None:
        print(f"No '{AUTOLOAD_KEY}' autoload found in {project_godot}; nothing to do.")
        return

    res_path = existing.lstrip("*")
    if res_path.startswith("res://"):
        installed_file = target_project / res_path[len("res://"):]
        if installed_file.is_file():
            installed_file.unlink()
            print(f"Removed {installed_file}")

    _remove_autoload(project_godot, AUTOLOAD_KEY)
    print(f"Removed autoload '{AUTOLOAD_KEY}' from {project_godot}")

    remove_config_section(project_godot, "rosetta_rpc")
    print(f"Removed [rosetta_rpc] section from {project_godot}")



def _remove_autoload(project_godot: Path, key: str) -> None:
    """Deletes `key="..."` line from the `[autoload]` section."""
    lines = project_godot.read_text().splitlines(keepends=True)
    key_pattern = re.compile(rf'^{re.escape(key)}=')

    autoload_idx = next(
        (i for i, l in enumerate(lines) if l.strip() == "[autoload]"), None
    )
    if autoload_idx is None:
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
        del lines[existing_idx]

    project_godot.write_text("".join(lines))


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("target_project", type=Path, help="path to the target Godot project")
    args = parser.parse_args()
    uninstall(args.target_project.resolve())


if __name__ == "__main__":
    main()
