---
needs: [plugins-v3-syl-conversion]
---
# `plugins_v3` as a `.syl` data language: delete the Lean authoring machinery

Standard constraints apply. Third of three tickets (after `plugins-v3-syl-generator` and
`plugins-v3-syl-conversion`). The data language itself is specified in
`docs/decisions/semantics-v3.md` §4.4–§4.5. Once the conversion lands, nothing in `plugins_v3`
is elaborated by the authoring machinery; this ticket deletes it.

## Why

The `plugins_v3` DSL was written in Lean syntax but no longer behaved like Lean. `load_bundle`,
expected-type dispatch, scoped `term_elab` hooks on `ident`/`app`/`structInst`, `AuthoringContext`
threaded through 2,073 declarations, `primitive_boundary%`, and `Authoring.inspect` /
`Authoring.Form` / `onlyAuthored` all existed to police an open term language into behaving like a
closed data language: more than 1,000 lines of `Authoring.lean`, plus a 1,272-line proof file
testing the policing. `plugins_v2` was a data language whose files were practically their own
AST; `.syl` returns to that simplicity.

## Change

- **Delete** `AuthoringContext`, `Authoring.inspect`, `Authoring.Form`, `onlyAuthored`,
  `primitive_boundary%`, `splice`/`CallerSplice`, the scoped `term_elab` hooks, every authoring
  coercion instance (§4.4 "No coercions"), and the `inCaller` opt-out handling.
- **Rewrite or retire** `Proofs/Authoring.lean` and `Proofs/CallerScope.lean` to test the
  generator, naming each retirement. In particular:
  - `incompatibleLocalAllowsWord` is rewritten to assert that a parameter used at the wrong sort
    is a generator error;
  - the `up` pin is retired as moot: a body-less primitive is its constructor's binding.
- **Mention-order pin** (from the deleted `lean-v3-mention-order-follows-constructors`): a Template
  that passes one `NounPhrase` as `conjunction`'s `parts` and as its `subject` gives the subject the
  first mention.
- **Zero qualified names** (from `plugins-v3-kept-qualified-sites`), and no dependency-scan
  workarounds left in `lean/CONTRACTS.md` or a README.

## Acceptance

- `cargo xtask lean check` passes; the Lean dump shows 0 differences against the one saved after
  `plugins-v3-syl-conversion`.
- The test that the Rust and Lake parsers agree belongs to `semantics-rust-rewrite`, where the
  second parser appears.
