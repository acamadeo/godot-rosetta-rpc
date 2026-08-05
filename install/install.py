#!/usr/bin/env python3
"""
Installs `RpcRuntime.gd` into a target Godot project and registers it as an
autoload in that project's `project.godot`. Also writes the search roots
RpcRuntime.gd uses to discover the Kotlin/C# runtimes into a `[rosetta_rpc]`
section of `project.godot` (see --kotlin-gdj-root/--csharp-rpc-root).
"""

import argparse
import shutil
from pathlib import Path

from util import read_autoload_value, set_config_value, AUTOLOAD_KEY

# Path within this codebase of the file to copy over.
FRAMEWORK_ROOT = Path(__file__).resolve().parent.parent
RPC_RUNTIME_SOURCE = FRAMEWORK_ROOT / "godot" / "RpcRuntime.gd"

DEFAULT_KOTLIN_GDJ_ROOT = "kotlin/gdj"
DEFAULT_CSHARP_RPC_ROOT = "csharp/generated/rpc"


def install(
    target_project: Path,
    dest_dir: str | None,
    csharp_rpc_root: str,
    kotlin_gdj_root: str,
) -> None:
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
    set_config_value(project_godot, "autoload", AUTOLOAD_KEY, f"*res://{autoload_path}")
    print(f"Registered autoload '{AUTOLOAD_KEY}' in {project_godot}")

    set_config_value(project_godot, "rosetta_rpc", "csharp_rpc_root", f"res://{csharp_rpc_root}")
    set_config_value(project_godot, "rosetta_rpc", "kotlin_gdj_root", f"res://{kotlin_gdj_root}")
    print(f"Set rosetta_rpc.csharp_rpc_root = \"res://{csharp_rpc_root}\" in {project_godot}")
    print(f"Set rosetta_rpc.kotlin_gdj_root = \"res://{kotlin_gdj_root}\" in {project_godot}")


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
    existing = read_autoload_value(project_godot, "autoload", AUTOLOAD_KEY)
    if existing is not None:
        res_path = existing.lstrip("*")
        if res_path.startswith("res://"):
            existing_file = target_project / res_path[len("res://") :]
            return existing_file.parent.relative_to(target_project).as_posix()
    return "."


def _remove_previous_copy(target_project: Path, project_godot: Path) -> None:
    existing = read_autoload_value(project_godot, "autoload", AUTOLOAD_KEY)
    if existing is None:
        return
    res_path = existing.lstrip("*")
    if not res_path.startswith("res://"):
        return
    previous_file = target_project / res_path[len("res://") :]
    if previous_file.is_file():
        previous_file.unlink()


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
    parser.add_argument(
        "--csharp-rpc-root",
        default=DEFAULT_CSHARP_RPC_ROOT,
        help=(
            "directory (relative to target_project) where RpcRuntime.gd should "
            f"recursively search for CSharpRuntime.cs (default: {DEFAULT_CSHARP_RPC_ROOT})"
        ),
    )
    parser.add_argument(
        "--kotlin-gdj-root",
        default=DEFAULT_KOTLIN_GDJ_ROOT,
        help=(
            "directory (relative to target_project) where RpcRuntime.gd should "
            f"recursively search for KotlinRuntime.gdj (default: {DEFAULT_KOTLIN_GDJ_ROOT})"
        ),
    )
    args = parser.parse_args()
    install(
        args.target_project.resolve(),
        args.dest_dir,
        args.csharp_rpc_root,
        args.kotlin_gdj_root,
    )


if __name__ == "__main__":
    main()
