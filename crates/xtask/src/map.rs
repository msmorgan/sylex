//! `cargo xtask map` — on-demand "bearings" dumps of current code shape, so
//! an agent can regenerate what a corpus of `pub enum`s currently looks like
//! instead of re-reading the source wholesale.
//!
//!  - `map enums [DIR]` — every `pub enum` under a Rust source tree (default
//!    `crates/deckmaste_core/src`), with each variant's shape and first doc
//!    line.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use clap::Args;
use clap::Subcommand;
use quote::ToTokens;
use syn::spanned::Spanned;

#[derive(Debug, Args)]
pub struct MapArgs {
    #[command(subcommand)]
    command: MapCmd,
}

#[derive(Debug, Subcommand)]
enum MapCmd {
    /// Dump every `pub enum` under a Rust source tree.
    Enums(EnumsArgs),
}

#[derive(Debug, Args)]
pub struct EnumsArgs {
    /// Directory to walk recursively for `.rs` files. Defaults to
    /// `crates/deckmaste_core/src`.
    dir: Option<PathBuf>,
}

/// # Errors
/// If the target directory is missing, unreadable, or a `.rs` file fails to
/// parse.
pub fn run(args: &MapArgs) -> anyhow::Result<()> {
    match &args.command {
        MapCmd::Enums(a) => run_enums(a),
    }
}

// ---------------------------------------------------------------------
// `map enums`
// ---------------------------------------------------------------------

/// # Errors
/// If DIR isn't a directory, a `.rs` file can't be read, or fails to parse.
fn run_enums(args: &EnumsArgs) -> anyhow::Result<()> {
    let dir = args.dir.clone().unwrap_or_else(|| {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/deckmaste_core/src")
    });
    anyhow::ensure!(dir.is_dir(), "not a directory: {}", dir.display());

    let mut rs_files = Vec::new();
    collect_rs_files(&dir, &mut rs_files).with_context(|| format!("walking {}", dir.display()))?;
    rs_files.sort();

    for path in &rs_files {
        let src =
            fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let file = syn::parse_file(&src).with_context(|| format!("parsing {}", path.display()))?;
        let mut enums = Vec::new();
        collect_pub_enums(&file.items, &mut enums);
        for item_enum in enums {
            print_enum(path, item_enum);
        }
    }

    Ok(())
}

/// Recursively walk `dir` collecting `.rs` file paths.
pub(crate) fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) -> anyhow::Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("reading dir {}", dir.display()))? {
        let entry = entry.with_context(|| format!("reading an entry of {}", dir.display()))?;
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out)?;
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
    Ok(())
}

/// Recursively walk a file's items (descending into inline `mod { .. }`
/// blocks) collecting `pub enum` items, in source order.
pub(crate) fn collect_pub_enums<'a>(items: &'a [syn::Item], out: &mut Vec<&'a syn::ItemEnum>) {
    for item in items {
        match item {
            syn::Item::Enum(item_enum) if matches!(item_enum.vis, syn::Visibility::Public(_)) => {
                out.push(item_enum);
            }
            syn::Item::Mod(item_mod) => {
                if let Some((_, inner_items)) = &item_mod.content {
                    collect_pub_enums(inner_items, out);
                }
            }
            _ => {}
        }
    }
}

fn print_enum(path: &Path, item: &syn::ItemEnum) {
    let line = item.span().start().line;
    println!(
        "## {} — {}:{line} ({} variants)",
        item.ident,
        path.display(),
        item.variants.len()
    );
    if let Some(doc) = first_doc_line(&item.attrs) {
        println!("{doc}");
    }
    for variant in &item.variants {
        let name = &variant.ident;
        let shape = variant_shape(&variant.fields);
        match first_doc_line(&variant.attrs) {
            Some(doc) => println!("- {name}{shape} — {doc}"),
            None => println!("- {name}{shape}"),
        }
    }
    println!();
}

/// `(<field types>)` for a tuple variant, `{ <name>: <type>, … }` for a
/// struct variant, empty for a unit variant.
fn variant_shape(fields: &syn::Fields) -> String {
    match fields {
        syn::Fields::Unit => String::new(),
        syn::Fields::Unnamed(unnamed) => {
            let types: Vec<String> = unnamed
                .unnamed
                .iter()
                .map(|field| field.ty.to_token_stream().to_string())
                .collect();
            format!("({})", types.join(", "))
        }
        syn::Fields::Named(named) => {
            let fields: Vec<String> = named
                .named
                .iter()
                .map(|field| {
                    // Named fields always carry an ident; `unwrap_or` covers
                    // the syntactically-unreachable case rather than panic.
                    let name = field
                        .ident
                        .as_ref()
                        .map_or_else(|| "_".to_string(), std::string::ToString::to_string);
                    let ty = field.ty.to_token_stream().to_string();
                    format!("{name}: {ty}")
                })
                .collect();
            format!("{{ {} }}", fields.join(", "))
        }
    }
}

/// The first non-blank `///` doc-comment line attached to `attrs`, if any.
fn first_doc_line(attrs: &[syn::Attribute]) -> Option<String> {
    attrs.iter().find_map(|attr| {
        if !attr.path().is_ident("doc") {
            return None;
        }
        let syn::Meta::NameValue(name_value) = &attr.meta else {
            return None;
        };
        let syn::Expr::Lit(expr_lit) = &name_value.value else {
            return None;
        };
        let syn::Lit::Str(lit_str) = &expr_lit.lit else {
            return None;
        };
        let text = lit_str.value();
        let trimmed = text.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}
