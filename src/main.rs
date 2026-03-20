// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//
// mylangiser CLI — Generate progressive-disclosure interfaces from complex APIs via My-Lang

use anyhow::Result;
use clap::{Parser, Subcommand};

mod codegen;
mod manifest;

/// mylangiser — Generate progressive-disclosure interfaces from complex APIs via My-Lang
#[derive(Parser)]
#[command(name = "mylangiser", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialise a new mylangiser.toml manifest.
    Init { #[arg(short, long, default_value = ".")] path: String },
    /// Validate a mylangiser.toml manifest.
    Validate { #[arg(short, long, default_value = "mylangiser.toml")] manifest: String },
    /// Generate My-Lang wrapper, Zig FFI bridge, and C headers.
    Generate {
        #[arg(short, long, default_value = "mylangiser.toml")] manifest: String,
        #[arg(short, long, default_value = "generated/mylangiser")] output: String,
    },
    /// Build the generated artifacts.
    Build { #[arg(short, long, default_value = "mylangiser.toml")] manifest: String, #[arg(long)] release: bool },
    /// Run the workload.
    Run {
        #[arg(short, long, default_value = "mylangiser.toml")] manifest: String,
        #[arg(trailing_var_arg = true)] args: Vec<String>,
    },
    /// Show manifest information.
    Info { #[arg(short, long, default_value = "mylangiser.toml")] manifest: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { path } => { manifest::init_manifest(&path)?; }
        Commands::Validate { manifest } => { let m = manifest::load_manifest(&manifest)?; manifest::validate(&m)?; println!("Valid: {}", m.workload.name); }
        Commands::Generate { manifest, output } => { let m = manifest::load_manifest(&manifest)?; manifest::validate(&m)?; codegen::generate_all(&m, &output)?; }
        Commands::Build { manifest, release } => { let m = manifest::load_manifest(&manifest)?; codegen::build(&m, release)?; }
        Commands::Run { manifest, args } => { let m = manifest::load_manifest(&manifest)?; codegen::run(&m, &args)?; }
        Commands::Info { manifest } => { let m = manifest::load_manifest(&manifest)?; manifest::print_info(&m); }
    }
    Ok(())
}
