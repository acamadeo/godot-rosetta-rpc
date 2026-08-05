//! Golden-file test: runs the real `protoc` binary against the built
//! plugin for both languages, and diffs the output against checked-in
//! `tests/golden/{rust,kotlin}/`. Fully Godot-free — plain `cargo test`.
//!
//! Re-baseline after an intentional template change with:
//! `UPDATE_GOLDEN=1 cargo test -p protoc-gen-rosetta-rpc golden`

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn run_protoc(lang: &str, extra_params: &str, out_dir: &Path) {
    let plugin_bin = env!("CARGO_BIN_EXE_protoc-gen-rosetta-rpc");
    let fixtures_dir = manifest_dir().join("tests/fixtures");
    let params = if extra_params.is_empty() {
        format!("lang={lang}")
    } else {
        format!("lang={lang},{extra_params}")
    };

    let status = Command::new("protoc")
        .arg(format!("--plugin=protoc-gen-rosetta-rpc={plugin_bin}"))
        .arg(format!("--rosetta-rpc_out={params}:{}", out_dir.display()))
        .arg("-I")
        .arg(&fixtures_dir)
        .arg(fixtures_dir.join("example.proto"))
        .status()
        .expect("failed to spawn protoc — is it installed and on PATH?");

    assert!(
        status.success(),
        "protoc exited with failure for lang={lang}"
    );
}

fn collect_files(root: &Path) -> BTreeSet<PathBuf> {
    let mut out = BTreeSet::new();
    if !root.exists() {
        return out;
    }
    for entry in walkdir(root) {
        out.insert(entry.strip_prefix(root).unwrap().to_path_buf());
    }
    out
}

fn walkdir(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                files.push(path);
            }
        }
    }
    files
}

fn assert_matches_golden(lang: &str, extra_params: &str) {
    let temp = tempfile::tempdir().unwrap();
    run_protoc(lang, extra_params, temp.path());

    let golden_dir = manifest_dir().join("tests/golden").join(lang);

    if std::env::var("UPDATE_GOLDEN").is_ok() {
        if golden_dir.exists() {
            std::fs::remove_dir_all(&golden_dir).unwrap();
        }
        copy_dir(temp.path(), &golden_dir);
        return;
    }

    let actual = collect_files(temp.path());
    let expected = collect_files(&golden_dir);
    assert_eq!(
        actual, expected,
        "generated file set for lang={lang} doesn't match tests/golden/{lang} \
         (run with UPDATE_GOLDEN=1 to re-baseline if this is intentional)"
    );

    for rel_path in &expected {
        let actual_content = std::fs::read_to_string(temp.path().join(rel_path)).unwrap();
        let expected_content = std::fs::read_to_string(golden_dir.join(rel_path)).unwrap();
        assert_eq!(
            actual_content,
            expected_content,
            "generated content for lang={lang}, file {} doesn't match golden \
             (run with UPDATE_GOLDEN=1 to re-baseline if this is intentional)",
            rel_path.display(),
        );
    }
}

fn copy_dir(src: &Path, dst: &Path) {
    std::fs::create_dir_all(dst).unwrap();
    for entry in std::fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());
        if path.is_dir() {
            copy_dir(&path, &dest_path);
        } else {
            std::fs::copy(&path, &dest_path).unwrap();
        }
    }
}

#[test]
fn rust_output_matches_golden() {
    // Every service always gets a full set of glue (Service, Descriptors,
    // Client, Adapter) in every language now — whether a language actually
    // implements a given service is a runtime decision (see
    // ServiceImplementations in bootstrap.rs.jinja), not a generation-time
    // one.
    assert_matches_golden("rust", "message_crate=protobuf_gen");
}

#[test]
fn csharp_output_matches_golden() {
    assert_matches_golden("csharp", "");
}

#[test]
fn kotlin_output_matches_golden() {
    assert_matches_golden("kotlin", "");
}
