---
needs: [syl-single-face-card-call]
---

# Template parameters are bindings

## Why

`docs/decisions/syl.md` §4 binds a Template parameter only when it is a NounPhrase used more than
once, so a Template that uses a parameter of another sort twice (`getsBoth`'s delta, `definesPt`'s
amount) reads its argument twice after expansion: a value the source writes once becomes two reads
in the tree. Three texts state the parameter rule differently: `syl.md` §4 (NounPhrase only),
`docs/cleanup/07-rust-ast.md` (the expander never tags first or later mentions) and
`docs/cleanup/08-drt.md` (the binding rule for Definition parameters only).

Written against the start of step 6 (`syl-single-face-card-call`).

## Design

- Every parameter of a Template or a Definition, at any sort, is a binding: its argument is read
  once, at the parameter's first use in read order, and each later use in the body re-mentions
  that read, never reading the argument again. Templates and Definitions bind parameters alike.
- A Template has no Reference Scope and no Deed, so its body's references reach the call site's
  referents; that is what sets it apart from a Definition. A Reference Scope on Templates would
  hide the call site from the pronoun Templates' anaphors and from the `when`/`whenever`/`at`
  Templates' bodies, and give a phrasing a scope the rules do not make (`semantics.md` §3).
- The expander keeps every call and parameter boundary and tags no mention; reference resolution
  decides a parameter's first use by read order and resolves its later uses to that binding.
- Sorts are already checked per slot, with nothing expanded: `cargo xtask dsl check` checks each
  argument against its parameter's declared sort at the call, and each `$name` against the slot it
  stands in, in every declaration kind (`a_parameter_at_another_sort_is_refused`).

## Done when

- `syl.md` §4 states the binding rule for every parameter of any sort and the per-slot sort check
  as it exists; `docs/cleanup/07-rust-ast.md`, `docs/cleanup/08-drt.md`, `semantics.md` §3 and the
  glossary's **Template** entry agree with it.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- One parameter used in two `or` arms (`objectsOfPlayer`): `syl-binding-across-or-arms`.

## Pointers

`docs/decisions/syl.md` §4, `docs/decisions/semantics.md` §2.3 and §3,
`docs/cleanup/07-rust-ast.md`, `docs/cleanup/08-drt.md`, `crates/xtask/src/dsl_check.rs`.
