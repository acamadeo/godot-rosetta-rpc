//! Smoke test wrapping around `example/tests/smoke_test.gd`. This primarily
//! sets up the smoke test, and is written in Rust so it can run in `cargo test`.
//!
//! Finds the appropriate `godot` binary to run the GDScript test. Ranked
//! preference:
//!  1. Godot-Kotlin (must be in PATH as `godot-kotlin`)
//!     - Compatible with Kotlin and GDExtension (Rust)
//!  2. Godot (must be in PATH as `godot` or `godot4`)
//!     - Compatible with GDExtension (Rust)

use std::env::{split_paths, var_os};
use std::fs::metadata;
use std::path::{Path, PathBuf};
use std::process::Command;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn example_dir() -> PathBuf {
    // manifest_dir is `<framework_root>/example/rust/game`.
    manifest_dir()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn framework_root() -> PathBuf {
    example_dir().parent().unwrap().to_path_buf()
}

/// Manual PATH scan for `name`.
fn find_on_path(name: &str) -> Option<PathBuf> {
    let path_var = var_os("PATH")?;
    split_paths(&path_var)
        .map(|dir| dir.join(name))
        .find(|candidate| candidate.is_file() && is_executable(candidate))
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    metadata(path)
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(_path: &Path) -> bool {
    true
}

fn run(command: &str) {
    let mut parts = command.split(' ');
    let program = parts.next().expect("empty command");
    let status = Command::new(program)
        .args(parts)
        .status()
        .unwrap_or_else(|e| panic!("failed to spawn {command}: {e}"));
    assert!(status.success(), "{command} failed");
}

/// Running `gradle` multiple times can be flaky. `godot-kotlin-jvm` plugin's
/// kspKotlin task often produces "Internal compiler error" / kspCaches
/// FileNotFoundException.
///
/// To reduce flakiness, run `gradle --stop`, delete the Gradle outputs and retry
/// the command up to 3 times total.
const GRADLE_MAX_ATTEMPTS: u32 = 3;

fn run_gradle(args: &[&str], cwd: &Path) {
    let command = format!("gradle {}", args.join(" "));
    for attempt in 1..=GRADLE_MAX_ATTEMPTS {
        let status = Command::new("gradle")
            .args(args)
            .current_dir(cwd)
            .status()
            .unwrap_or_else(|e| panic!("failed to spawn {command}: {e}"));
        if status.success() {
            return;
        }
        if attempt < GRADLE_MAX_ATTEMPTS {
            eprintln!(
                "{command}: failed (attempt {attempt}/{GRADLE_MAX_ATTEMPTS}), retrying after \
                 `gradle --stop` (known kspKotlin daemon flake)"
            );
            run("gradle --stop");
            run("rm -rf bin build .gradle .kotlin");
        }
    }
    panic!("{command} failed {GRADLE_MAX_ATTEMPTS} times in a row (not the known flake)");
}

#[test]
fn godot_smoke_test() {
    let godot_bin = find_on_path("godot4").or_else(|| find_on_path("godot"));
    let Some(godot_bin) = godot_bin else {
        eprintln!("godot_smoke_test: skipping — no godot4/godot binary on PATH.");
        return;
    };
    let godot_kotlin_bin = find_on_path("godot-kotlin");

    let framework_root = framework_root();
    let example_dir = example_dir();

    // Build protoc plugin that generates Rust/Kotlin service bindings.
    run(&format!(
        "cargo build -p protoc-gen-rosetta-rpc --manifest-path {}",
        framework_root.join("Cargo.toml").display()
    ));

    // Set up the example/ directory.
    run(&format!(
        "python3 {}",
        example_dir.join("generate.py").display()
    ));

    // Build example/ Rust source.
    run(&format!(
        "cargo build -p game --manifest-path {}",
        framework_root.join("Cargo.toml").display()
    ));

    let bin_to_use = match &godot_kotlin_bin {
        Some(bin) => {
            eprintln!(
                "godot_smoke_test: using '{}' — Kotlin integration will be exercised in-process.",
                bin.display()
            );
            // Build example/ Kotlin source.
            run_gradle(&["build"], &example_dir);
            bin
        }
        None => {
            eprintln!(
                "godot_smoke_test: using '{}' (no godot-kotlin on PATH — Kotlin integration check will be skipped).",
                godot_bin.display()
            );
            &godot_bin
        }
    };

    run(&format!(
        "{} --headless --path {} --script res://tests/smoke_test.gd",
        bin_to_use.display(),
        example_dir.display()
    ));
}
