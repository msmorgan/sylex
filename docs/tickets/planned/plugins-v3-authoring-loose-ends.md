---
needs: [plugins-v3-expansion-baseline-recursive-samples]
---
# Loose ends from the overnight `plugins-v3-*` chain

Standard constraints apply. These are small, independent defects found reviewing the chain
`plugins-v3-card-class` … `plugins-v3-canon-cleanup` (2026-10-28). Each one can be landed as
its own change.

1. **Finish the builtin bare-word rewrite.**
   - `plugins-v3-canon-authoring-surface` required "Rewrite the builtin's own dot forms for these
     sorts". Its done ticket was then edited to exempt a broad class of them instead.
   - Builtin still writes the following dot forms, which canon now writes bare (counts as of
     2026-10-28): `.up` 19 (`exalted`, `bushido`, `doublePower*`, `tripleToughness`), `.down` 4,
     `.stat` 24 (e.g. `card_type/battle.lean`), `.type`/`.subtype` 29 (`templates/noun/*`),
     `.playerStat` 3, `.counter` 1 and `.byKeyword` 6.
   - Rewrite every site where the bare word resolves. A site that cannot is kept only with its
     reason written beside it. A declaration that cannot write its own name bare (the
     `creature` template) is one such reason.
   - Expansion baseline: 0 differences.
2. **Delete `templates/ability/levelBand.lean` and `prototypeAlt.lean`.** They were kept only
   "until canon cleanup migrates their callers". Their callers are migrated, and both files
   still use `stat`.
3. **Make `cardClass` required on `card_type`.**
   - `cardClass` defaults to `none`, so a permanent type that leaves it out silently has no Card
     Class.
   - Require it in Lean `card_type` authoring and in the Rust registry reader. `none` must be
     written explicitly (kindred [CR#308.1], dungeon [CR#309.2c]).
   - Negative test on both sides.
4. **Settle the `some`-wrapping list coercion.**
   - `plugins-v3-primitive-template-boundary` (`qovownsn`) added a coercion to `Authoring.lean`
     that wraps a list literal in `some` when the expected type is `Option X`, for a hard-coded
     set of five sorts (`Action`, `Predicate`, `Condition`, `StaticSpec`, `Cost`).
   - It is untested and documented nowhere. Either generalize it to every sort that has a
     domain-list elaborator, or document why the set is closed.
   - Pin it in Lean, mirror it in Rust if Rust lacks it, and test both.
5. **Make Lean and Rust agree on wrongly typed locals.**
   - Lean `elabClosedWord` lets a constructor word win over a local of an incompatible type,
     pinned as `incompatibleLocalAllowsWord`. A misused local silently becomes a different
     well-typed term.
   - Rust `nullary_word` (`lower/mod.rs`) errors instead, except for a three-sort allowlist.
   - Choose the loud behaviour, an elaboration error, on both sides unless a canon card needs
     otherwise. Rewrite `incompatibleLocalAllowsWord` to assert it.
6. **Make the `up` pin able to fail.**
   - `deltasKeepTheirDeclaration` compares against `Delta.up 2`, and the Rust test compares
     `up 2` with `.up 2`. Neither can tell the `primitive up` declaration from the constructor.
   - Assert that the elaborated term's head is the primitive's constant (Lean) or the
     declaration key (Rust).
7. **Stop the canon filename test from re-implementing `card_filename`.**
   `live_canon_cards_elaborate_with_builtin` (`crates/deckmaste_semantics_v3/tests/lower.rs`)
   re-implements `deckmaste_core::plugin::card_filename`. Use the real one (a dev-dependency) and
   assert NFC names.
8. **Move the dispatch bullet out of the ADR.**
   - `docs/decisions/semantics-v3.md` §4.5 "Dependency and dispatch rules" describes
     implementation mechanics, sitting directly under an owner ruling.
   - Move it to `lean/CONTRACTS.md` (bundle loading) and leave a one-line pointer in §4.5.
   - `lean-v3-mention-order-follows-constructors` owns the neighbouring parameter-order bullet.

## Acceptance

- Each item lands with its stated test or pin.
- The expansion baselines show 0 differences, except item 2's two removed keys.
- Gates: `cargo xtask lean check`. Run `cargo test` and clippy for `deckmaste_semantics_v3`,
  `deckmaste_lowering_v3` and `xtask`.
