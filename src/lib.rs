#![forbid(unsafe_code)]
// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//
// mylangiser library — progressive-disclosure interface generation from
// complex APIs via My-Lang. This crate provides:
//
// - `manifest` — Parse and validate mylangiser.toml manifests
// - `codegen`  — Score complexity, assign levels, generate layered wrappers
// - `abi`      — Core type definitions (DisclosureLevel, ComplexityScore, etc.)

pub mod abi;
pub mod codegen;
pub mod manifest;

pub use abi::{APIEndpoint, ComplexityScore, DisclosureLevel, LayeredWrapper, SmartDefault};
pub use manifest::{load_manifest, validate, Manifest};

/// Convenience function: load a manifest, validate it, and generate all
/// layered wrapper code in the specified output directory.
///
/// # Errors
/// Returns an error if the manifest is invalid or file I/O fails.
pub fn generate(manifest_path: &str, output_dir: &str) -> anyhow::Result<()> {
    let m = load_manifest(manifest_path)?;
    validate(&m)?;
    codegen::generate_all(&m, output_dir)
}
