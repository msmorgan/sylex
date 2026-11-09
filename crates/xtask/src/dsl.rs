//! `cargo xtask dsl`: `.syl` tooling. `fmt` lives in [`crate::dsl_fmt`] and
//! `check` in [`crate::dsl_check`].

use clap::Args;
use clap::Subcommand;

#[derive(Debug, Args)]
pub struct DslArgs {
    #[command(subcommand)]
    command: DslCmd,
}

#[derive(Debug, Subcommand)]
enum DslCmd {
    /// Check that every reference in the `.syl` bundles resolves by name,
    /// sort and arity.
    Check(crate::dsl_check::CheckArgs),
    /// Format `.syl` files (default: every `plugins` file) in place, check
    /// them, or format standard input.
    Fmt(crate::dsl_fmt::FmtArgs),
}

pub fn run(args: &DslArgs) -> anyhow::Result<()> {
    match &args.command {
        DslCmd::Check(args) => crate::dsl_check::run(args),
        DslCmd::Fmt(args) => crate::dsl_fmt::run(args),
    }
}
