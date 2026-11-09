---
needs: []
---

# Required arguments are positional, defaulted arguments are named

## Why

`cargo xtask dsl check` binds a call's arguments in written order: a named argument binds its
parameter, a positional one the first parameter not yet bound (`Resolver::call`,
`crates/xtask/src/dsl_check.rs`). The corpus mixes both ways freely: required arguments passed by
name, and optional ones passed positionally, many of them verbs overloaded across valencies by a
defaulted parameter. `docs/decisions/syl.md` §2 says "argument order carries no meaning", so
nothing tells a reader which parameter a positional argument fills.

## Design

- A required argument is passed positionally and binds the next unbound required parameter, in
  declaration order, even when an optional parameter is declared before it.
- A defaulted argument is passed by name.
- A named argument whose written value equals the parameter's declared default is refused.

## Done when

- `cargo xtask dsl check` binds positional arguments only to required parameters, refuses a named
  required argument, a positional defaulted one, and a named argument equal to its declared
  default; each refusal has a negative test.
- Every call in `plugins/` follows the rule, with no named argument equal to its default.
- The respelled corpus means the same: a one-off script, kept in `docs/evidence/`, checks that
  every call binds the same parameter-to-value map before and after (counting an omitted default
  as its default), and the landing reports the call count it compared.
- `docs/decisions/syl.md` §2 states the rule in place of "argument order carries no meaning".
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

- `crates/xtask/src/dsl_check.rs` (`Resolver::call`)
- `docs/decisions/syl.md` §2 and §3
