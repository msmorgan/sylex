//! `cargo xtask dsl fmt`: format `.syl` files with `sylex_syl`'s printer.
//!
//! The command line is `sylex_syl::cli`'s, the same as the `sylfmt`
//! binary's (`cargo xtask dsl fmt --help` lists it); this command only
//! supplies the default path, every `plugins_v3/**/*.syl` file.

use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Args;

#[derive(Debug, Args)]
#[command(disable_help_flag = true)]
pub struct FmtArgs {
    /// The formatter's flags and paths; `--help` lists them.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

pub fn run(args: &FmtArgs) -> anyhow::Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()?;
    let default_paths: [PathBuf; 1] = [root.join("plugins_v3")];
    if sylex_syl::cli::main(args.args.iter().cloned(), &default_paths) == ExitCode::SUCCESS {
        Ok(())
    } else {
        anyhow::bail!("dsl fmt failed")
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;

    #[derive(Debug, Parser)]
    struct Cli {
        #[command(flatten)]
        args: FmtArgs,
    }

    #[test]
    fn every_argument_reaches_the_shared_command_line() {
        let args = [
            "--check",
            "--width",
            "80",
            "--help",
            "--record-spaces",
            "a.syl",
        ];
        let cli = Cli::try_parse_from(std::iter::once("fmt").chain(args)).unwrap();
        assert_eq!(cli.args.args, args);
    }
}
