//! `sylfmt`: the `.syl` formatter, as a small binary for `jj fix` and quick
//! manual use. Its flags are `deckmaste_dsl::cli`'s, the same as
//! `cargo xtask dsl fmt`'s; `sylfmt --help` lists them.

use std::process::ExitCode;

fn main() -> ExitCode {
    deckmaste_dsl::cli::main(std::env::args().skip(1), &[])
}
