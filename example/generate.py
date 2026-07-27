#!/usr/bin/env python3
"""Generates the following bindings from proto/example.proto:

  1. prost message types -> rust/protobuf-gen/src/gen/
  2. Java + Kotlin message types -> kotlin/generated/{java,kotlin}/
  3. godot-rosetta-rpc glue (Rust, implementing Clock) -> rust/game/src/generated/
  4. godot-rosetta-rpc glue (Kotlin, implementing GameService) -> kotlin/generated/

Requires `protoc`, `protoc-gen-prost` (`cargo install protoc-gen-prost`), and a
built `protoc-gen-rosetta-rpc` (built here on demand via `cargo build`) all
reachable.
"""

import os
import shutil
import subprocess
import sys
from pathlib import Path

EXAMPLE_PROJECT_ROOT = Path(__file__).parent
FRAMEWORK_ROOT = EXAMPLE_PROJECT_ROOT.parent
PROTO_ROOT = EXAMPLE_PROJECT_ROOT / "proto"

OUT_RUST_MESSAGES = EXAMPLE_PROJECT_ROOT / "rust/protobuf-gen/src/gen"
OUT_JAVA_MESSAGES = EXAMPLE_PROJECT_ROOT / "kotlin/generated/java"
OUT_KOTLIN_MESSAGES = EXAMPLE_PROJECT_ROOT / "kotlin/generated/kotlin"
OUT_RUST_RPC = EXAMPLE_PROJECT_ROOT / "rust/game/src/generated"
OUT_KOTLIN_RPC = EXAMPLE_PROJECT_ROOT / "kotlin/generated/rpc"

PROTOC = "protoc"

INSTALL_PY = FRAMEWORK_ROOT / "install/install.py"
UNINSTALL_PY = FRAMEWORK_ROOT / "install/uninstall.py"


def collect_proto_files() -> list[Path]:
    return sorted(PROTO_ROOT.rglob("*.proto"))


def clean(*dirs: Path) -> None:
    for d in dirs:
        if d.exists():
            shutil.rmtree(d)
        d.mkdir(parents=True, exist_ok=True)


def uninstall_rpc_runtime() -> None:
    subprocess.run(
        [sys.executable, str(UNINSTALL_PY), str(EXAMPLE_PROJECT_ROOT)],
        check=True,
    )


def install_rpc_runtime() -> None:
    subprocess.run(
        [sys.executable, str(INSTALL_PY), str(EXAMPLE_PROJECT_ROOT)],
        check=True,
    )


def build_rosetta_rpc_plugin() -> Path:
    subprocess.run(
        ["cargo", "build", "-p", "protoc-gen-rosetta-rpc"],
        cwd=FRAMEWORK_ROOT,
        check=True,
    )
    ext = "" if _is_unix() else ".exe"
    return FRAMEWORK_ROOT / f"target/debug/protoc-gen-rosetta-rpc{ext}"


def generate_rust_messages(files: list[Path]) -> None:
    plugin = shutil.which("protoc-gen-prost")
    if plugin is None:
        raise RuntimeError(
            "protoc-gen-prost not found on PATH. Install it with "
            "`cargo install protoc-gen-prost`."
        )
    subprocess.run(
        [
            PROTOC,
            f"--plugin=protoc-gen-prost={plugin}",
            f"--prost_out={OUT_RUST_MESSAGES}",
            "-I",
            str(PROTO_ROOT),
            *[str(f) for f in files],
        ],
        check=True,
    )


def generate_kotlin_messages(files: list[Path]) -> None:
    subprocess.run(
        [
            PROTOC,
            f"--java_out={OUT_JAVA_MESSAGES}",
            f"--kotlin_out={OUT_KOTLIN_MESSAGES}",
            "-I",
            str(PROTO_ROOT),
            *[str(f) for f in files],
        ],
        check=True,
    )


def generate_rosetta_rpc(plugin_bin: Path, files: list[Path]) -> None:
    subprocess.run(
        [
            PROTOC,
            f"--plugin=protoc-gen-rosetta-rpc={plugin_bin}",
            f"--rosetta-rpc_out=lang=rust,message_crate=protobuf_gen:{OUT_RUST_RPC}",
            "-I",
            str(PROTO_ROOT),
            *[str(f) for f in files],
        ],
        check=True,
    )
    subprocess.run(
        [
            PROTOC,
            f"--plugin=protoc-gen-rosetta-rpc={plugin_bin}",
            f"--rosetta-rpc_out=lang=kotlin:{OUT_KOTLIN_RPC}",
            "-I",
            str(PROTO_ROOT),
            *[str(f) for f in files],
        ],
        check=True,
    )

def _is_unix() -> bool:
    return os.name == 'posix'


def main() -> None:
    uninstall_rpc_runtime()
    clean(
        OUT_RUST_MESSAGES,
        OUT_JAVA_MESSAGES,
        OUT_KOTLIN_MESSAGES,
        OUT_RUST_RPC,
        OUT_KOTLIN_RPC,
    )

    files = collect_proto_files()
    print("Compiling .proto files:")
    for f in files:
        print("  ", f)

    plugin_bin = build_rosetta_rpc_plugin()
    generate_rust_messages(files)
    generate_kotlin_messages(files)
    generate_rosetta_rpc(plugin_bin, files)
    install_rpc_runtime()

    print("\nGenerated messages and RPC glue for the example project.")


if __name__ == "__main__":
    main()
