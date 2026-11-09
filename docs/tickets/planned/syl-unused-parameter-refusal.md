---
needs: []
---

# Refuse a parameter its declaration never reads

## Why

A declaration with a body (`template`, `action`, `ability`, `ability_word`) may declare a
parameter its body never reads with `$name`, and `cargo xtask dsl check` accepts it. A parameter
nothing reads is a bug: the caller's argument is dropped silently.

## Design

- A parameter that its declaration's block never reads is refused, on every declaration kind with
  a body. The one exception is `deed_action` (`syl-explicit-deeds`), whose object parameter can
  name what received the Deed without being read.

## Done when

- `cargo xtask dsl check` refuses an unread parameter on each kind with a body, with a negative
  test for each kind.
- Every unread parameter in `plugins/` is read, or removed with its arguments at every call site;
  the landing lists each one and which it was.
- Declaration order stays the printed order (`syl.md` §5).
- `docs/decisions/syl.md` states the rule.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

- `crates/xtask/src/dsl_check.rs` (`Resolver::body`), `crates/xtask/src/dsl_check/schema.rs`
  (the declaration kinds)
- `docs/decisions/syl.md` §1 and §2
