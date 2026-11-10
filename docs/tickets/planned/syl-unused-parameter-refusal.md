---
needs: [syl-craft-materials, syl-companion-pregame-rule, syl-bodiless-keywords, cleanup-step6-surface]
---

# Refuse a parameter its declaration never reads

## Why

A declaration with a body (`template`, `action`, `ability`, `ability_word`) may declare a
parameter its body never reads with `$name`, and `cargo xtask dsl check` accepts it. A parameter
nothing reads is a bug: the caller's argument is dropped silently.

Three declarations have an unread parameter today: `craft`'s `cost`, `companion`'s `condition`
and `toxic`'s `amount`. `syl-craft-materials` and `syl-companion-pregame-rule` rewrite the first
two, and land first.

Written against the surface (`cleanup-step6-surface`).

## Design

- A parameter that its declaration's block never reads is refused, on every declaration kind with
  a body. `deed_action` (`syl-explicit-deeds`) is exempt.

## Open questions

- `toxic`'s `amount` is read only through the keyword's number: the rule row `toxicPoison.syl`
  reads `totalToxicValue`, which sums `keywordNumber(toxic)` [CR#702.164b,702.164c], and toxic's
  own body is empty. Does the refusal count that as reading `amount`, or does toxic's rule move so
  that its own declaration reads `amount`?

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
