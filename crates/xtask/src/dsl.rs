//! `cargo xtask dsl`: `.syl` tooling. `fmt` lives in [`crate::dsl_fmt`],
//! `check` in [`crate::dsl_check`] and `expand` in [`crate::dsl_expand`].

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
    /// Print the expansion of one canon card or one term.
    Expand(crate::dsl_expand::ExpandArgs),
    /// Format `.syl` files (default: every `plugins` file) in place, check
    /// them, or format standard input.
    Fmt(crate::dsl_fmt::FmtArgs),
}

pub fn run(args: &DslArgs) -> anyhow::Result<()> {
    match &args.command {
        DslCmd::Check(args) => crate::dsl_check::run(args),
        DslCmd::Expand(args) => crate::dsl_expand::run(args),
        DslCmd::Fmt(args) => crate::dsl_fmt::run(args),
    }
}
