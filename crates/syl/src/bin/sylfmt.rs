//! `sylfmt`: the `.syl` formatter, as a small binary for `jj fix` and quick
//! manual use. Its flags are `sylex_syl::cli`'s, the same as
//! `cargo xtask dsl fmt`'s; `sylfmt --help` lists them.

use std::process::ExitCode;

fn main() -> ExitCode {
    sylex_syl::cli::main(std::env::args().skip(1), &[])
}
