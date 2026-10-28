---
needs: [lean-build-speed, lean-elaborated-value-dump]
---
# [design] `plugins_v3` as a data language whose files are their own AST

Standard constraints apply. This ticket needs a design dialogue and an amendment to
`docs/decisions/semantics-v3.md` before implementation.

## Why

The `plugins_v3` DSL is written in Lean syntax, but it no longer behaves like Lean. `load_bundle`,
expected-type dispatch, scoped `term_elab` hooks on `ident`/`app`/`structInst`, `AuthoringContext`
threaded through 2,073 declarations, `primitive_boundary%`, and `Authoring.inspect` / `Authoring.Form`
/ `onlyAuthored` all exist to police an open term language into behaving like a closed data
language. Together that is more than 1,000 lines of `Authoring.lean`, plus a 1,272-line proof file
testing the policing.

`plugins_v2` was a data language, and its files were practically their own AST. Return to that
simplicity.

## Settled (owner, 2026-10-28)

- **Lean is a workbench and verifier, never in the hot path of card compilation.** The Rust
  pipeline must parse the DSL itself. It is being rewritten as `semantics` (parser and AST),
  `discourse` (DRT referent resolution, in Rust) and `lowering` (to core); see
  `semantics-rust-rewrite`.
- **Mentions follow constructor order.** Whether an occurrence is a first or later mention is
  decided in the order the constructor arguments it reaches are read, after expansion. A
  declaration's parameter order carries no meaning, so primitive and Template signatures may
  follow the printed phrase. This replaces the parameter-order rule in §4.5, which 12 primitives
  violate.
- **No higher-order parameters.** Delete `chooseModesOver`, `splice` and `CallerSplice`. Both
  users, Call Damage Control and Wild Shape, write their modes out as printed.
- **The existing definitions are converted by script**: a purpose-built script, or a final
  emitter run from the current `deckmaste_semantics_v3`. They are not converted by hand.

## Questions for the design dialogue

1. **Host.** Is the data language still parsed by Lean at all, as a closed `declare_syntax_cat`
   grammar, or is it a standalone format? A standalone format would be parsed natively by Rust and
   loaded into Lean through a generated or `load_bundle` step. Either way, one grammar document is
   the spec for both parsers.
2. **Grammar.** The productions are:
   - calls with positional and named arguments;
   - literals;
   - domain lists;
   - record blocks;
   - closed enum words;
   - the existing domain coercions.

   Settle which productions exist, and how a bare word, a declaration call and a parameter are
   told apart without Lean's term elaborator.
3. **Dispatch.** A name is resolved by its bare name together with the expected sort. Settle how
   that works, how `Option` unwraps, and how list literals map per sort.
   `lean-v3-overloaded-primitive-typeclasses` and `plugins-v3-authoring-loose-ends` item 4 are
   folded in here.
4. **Expansion.** Templates expand to constructor trees, and Definitions (`action`, `keyword`)
   expand to `ownScope` with `inCaller`. First and later mentions are inserted after expansion,
   following the ruling above.
5. **What replaces `Authoring.Form`.** Only Lean self-tests use it; Rust has no use for it. Either
   build it from the parsed tree for the two call-tree pins (`Proofs/TextChanges.lean`,
   `Proofs/UntapStep.lean`), or retire those pins in favour of tests on the parse tree.
6. **The dependency graph.** Build typed dependency edges, so that `exileZone`'s
   `zone .exile none` and `devourEntry`'s `Action.devour` workarounds disappear and no
   qualification is ever needed.

## Implementation, once settled

- **Oracle.** Save the Lean dump (`lean-elaborated-value-dump`) before converting. Afterwards it
  must show 0 differences under the conversion's rename map. The Rust `xtask expansions` cannot
  read the converted files, so it is no oracle here.
- **Delete the old Rust crates in this same change, right after the conversion.** The old Rust
  parser cannot read the converted files, so these would fail `cargo test` from the moment the
  files change:
  - `deckmaste_semantics_v3` and `deckmaste_lowering_v3`, including their tests
    (`tests/bundle.rs`, `tests/lower.rs`);
  - `xtask expansions`;
  - `crates/xtask/tests/suite/plugins_v3_declarations.rs`.

  If the conversion uses the old crate as its emitter, run it first, then delete. Delete any
  ticket text that still names these crates as live.
- **Delete** `AuthoringContext`, `Authoring.inspect`, `Authoring.Form` (unless question 5 keeps a
  parse-tree form), `onlyAuthored`, `primitive_boundary%`, `splice`/`CallerSplice`, and the scoped
  `term_elab` hooks. Rewrite or retire `Proofs/Authoring.lean` and `Proofs/CallerScope.lean` to
  test the new loader, naming each retirement.
- **Folded in from earlier tickets:**
  - from `plugins-v3-kept-qualified-sites`: zero qualified names, and no dependency-scan
    workarounds left in CONTRACTS.md or README.md;
  - from `plugins-v3-authoring-loose-ends`: Lean refuses a wrongly typed local loudly, and the
    `up` pin can fail;
  - the mention-order pin from the deleted `lean-v3-mention-order-follows-constructors`.

  For the mention-order pin: a Template that passes one NounPhrase as `conjunction`'s `parts` and
  as its `subject` gives the subject the first mention.
- Additive `lean-v3-*` tickets that land before the conversion are carried over by the script.
