---
needs: [lean-generated-canon-modules, lean-elaborated-value-dump, plugins-v3-first-order-templates, plugins-v3-syl-surface-syntax]
---
# `plugins_v3` as a `.syl` data language whose files are their own AST

Standard constraints apply. The abstract model and dispatch rules are settled below and recorded in
`docs/decisions/semantics-v3.md` §§4.4–4.5; `plugins-v3-syl-surface-syntax` settles the concrete
surface punctuation before implementation.

## Why

The `plugins_v3` DSL is written in Lean syntax, but it no longer behaves like Lean. `load_bundle`,
expected-type dispatch, scoped `term_elab` hooks on `ident`/`app`/`structInst`, `AuthoringContext`
threaded through 2,073 declarations, `primitive_boundary%`, and `Authoring.inspect` / `Authoring.Form`
/ `onlyAuthored` all exist to police an open term language into behaving like a closed data
language. Together that is more than 1,000 lines of `Authoring.lean`, plus a 1,272-line proof file
testing the policing.

`plugins_v2` was a data language, and its files were practically their own AST. Return to that
simplicity.

## Settled (owner, 2026-10-28..2026-10-30)

- **Lean is a workbench and verifier, never in the hot path of card compilation.** The Rust
  pipeline must parse the DSL itself. It is being rewritten as `semantics` (parser and AST),
  `discourse` (DRT referent resolution, in Rust) and `lowering` (to core); see
  `semantics-rust-rewrite`.
- **Host and `.syl` file extension.** `plugins_v3` files are standalone `.syl` data files
  (`plugins_v3/**/*.syl`), not Lean source files. They are parsed natively by Rust (`semantics`)
  and by the Lake generator in `lean/lakefile.lean`, which emits gitignored `.lean` modules under
  `lean/.lake/build/gen/` containing plain Lean `def`s. Zero custom Lean `term_elab` hooks or
  authoring macros remain. One grammar specification in `docs/decisions/semantics-v3.md` §4.4
  governs both parsers; `plugins-v3-syl-surface-syntax` settles its concrete tokens.
- **Mentions follow constructor order.** `firstMention` evaluates a parameter's argument once and
  records its referents; `laterMention` reads them back. Which occurrence is first is decided after
  expansion, in the order the checker evaluates the expanded constructor's fields. Authors write
  arguments in printed-card order, and a declaration's parameter order carries no meaning, so
  primitive and Template signatures may follow the printed phrase (12 primitives violated the old
  parameter-order rule).
- **No `inCaller` opt-out.** Definitions always wrap every `CallerScope` parameter. `search` and
  `searchZonesOf` drop their `inCaller for_` opt-outs, so `for_` is bound like any parameter;
  `madness`'s scope-escape `inCaller` is an ordinary call. Expansion changes this causes are
  reviewed for correctness, not frozen (see Oracle).
- **No evaluation in plugin files.** No higher-order parameters, `match`, `if`, `fun`, or
  `List`/`Option` functions; `plugins-v3-first-order-templates` removed the existing sites
  (including `chooseModesOver`, `splice` and `CallerSplice`), along with `have`, tuples and `⟨…⟩`.
  The grammar has no local bindings and no bottom-up type inference.
- **Abstract grammar, no coercions, no domain lists, no leading dots.**
  - The 5 expression productions are:
    1. literals (string `"..."`, integer, `true`/`false`);
    2. list literals (`[...]`, valid at expected sort `List T` or `Option (List T)`);
    3. record blocks;
    4. calls with positional and named arguments;
    5. bare words (0-argument identifiers).
  - **All coercions are deleted, simplicity first (owner, 2026-10-30).** That includes
    `NounPhrase → Subject` and the 12 expression-level `Coe`s (`actor`, `hasSubtype`, `perform`,
    `mana`, `written`, `printedKind`, `printedName`, `holds`, `static`, `span`, `mode`,
    `Characteristics → CharacteristicBundle`). Every wrapper is written explicitly in `.syl`;
    coercions may be reintroduced deliberately once the language is simple enough to refactor.
  - **All 5 domain-list coercions are deleted** (`sequentially`, `and`, `conjunction`, `compound`
    and `mana` are always written explicitly; `[...]` is only `List T`).
  - **No leading dots** (`.foo` becomes bare `foo`), and **no polymorphic `{α : Type}` binders**
    (`plugins_v3/builtin/templates/noun/bracketed.lean` is deleted).
- **Deterministic `O(1)` dispatch and `Option` auto-wrapping.**
  - Every node is resolved top-down by `(bare_name, expected_sort)` with zero backtracking, in
    priority order:
    1. parameter of the enclosing declaration (0 args; declared sort `T`, or `U` when
       `T = Option U`);
    2. `none` when `T = Option U`;
    3. declaration `(head, T)`, then `(head, U)` when `T = Option U` (the exact sort wins);
    4. constructor / closed enum word of `T` (or `U` when `T = Option U`).

    A declaration `(head, T)` that shares its name with a constructor or enum word of `T` is a
    load error.
  - **`Option U` auto-wrapping:** Presence of a `U` wraps in `some`; omission of an argument
    or record field declared with a default (including `none`) takes that default; `none` remains available as an explicit default or
    argument; `some` is never written.
  - **Direct sort atoms:** integer literals at `Nat`, `Int`, `Amount` (`Amount.lit n`) and
    `ManaSymbol` (`generic n`); color words (`white`, `blue`, `black`, `red`, `green`) at `Color`,
    `ColorOrColorless` and `ManaSymbol`; `ability` definition names at `Ability`, `KeywordLabel`
    and `KeywordTerm`; `action` definition names at `Action` and `Deed`.
  - Supersedes the deleted `lean-v3-overloaded-primitive-typeclasses` (`kknnrqss`): all `Has*`
    typeclasses and all qualified names are deleted.
- **Expansion.** Templates expand to constructor trees by first-order substitution, and
  Definitions (`action`, `ability`) expand to `ownScope` with `inCaller` on every `CallerScope`
  parameter. First and later mentions are inserted after expansion by one walk in constructor field
  order.
- **`Authoring.Form` is deleted; cleave is backburnered (owner, 2026-10-30).** Delete
  `Authoring.Form`, `Authoring.inspect`, `onlyAuthored`, and
  `plugins_v3/builtin/templates/noun/bracketed.lean`, with the `bracketed` pins in
  `Proofs/TextChanges.lean` (retired: they assert a template that no longer exists; name each).
  Cleave is not expected to work through templates until `plugins-v3-cleave-bracketed-text`
  returns `bracketed` in the new language. Rewrite `untapStepCardsUseUntapDuring` in `Proofs/UntapStep.lean` to assert on the
  expanded `.card` AST directly.
- **The dependency graph and builtin modules.** Do not do file discovery or dependency
  sorting inside a Lean elaborator.
  - The Lake generator from `lean-generated-canon-modules` (a `lakefile.lean` target behind the
    `Module.inputFacet` override, ending with `setTrace .nil`) parses `plugins_v3/builtin/**/*.syl`
    and `plugins_v3/canon/**/*.syl` in the `.syl` grammar. It writes gitignored Lean modules with
    real `import`s.
  - One module per component makes module names change as the builtin changes. That also requires
    overriding `LeanLib.modulesFacet`, so the library awaits the generator before listing its
    modules. Add a case for it to the generator's regression test.
  - Each strongly connected component of the possibly over-approximate dependency graph becomes
    one module, so a false edge only merges two modules instead of failing.
  - Delete `load_bundle`, `topoSortBundle`, `referencedIdents`, the directory walk and the
    `builtinBundle` input. Lake then builds the builtin in parallel and rebuilds only the
    dependents of an edited declaration.
  - The `exileZone` (`zone .exile none`) and `devourEntry` (`Action.devour`) workarounds disappear,
    and no qualification is ever needed.
- **The existing definitions are converted by script**: a purpose-built script, or a final
  emitter run from the current `deckmaste_semantics_v3`. They are not converted by hand.

## Implementation, once settled

- **Oracle.** Save the Lean dump (`lean-elaborated-value-dump`) before converting. Afterwards it
  must show 0 differences under the conversion's rename map, except the changes the rulings
  above deliberately make (the `search`/`searchZonesOf` opt-out); each such difference is reviewed
  for correctness and the oracle is updated, not the ruling bent to fit it. The Rust `xtask expansions` cannot
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
- **Conformance.** A differential test parses the same corpus with the Rust parser and the Lake
  generator and requires identical trees.
- **Folded in from earlier tickets:**
  - from `plugins-v3-kept-qualified-sites`: zero qualified names, and no dependency-scan
    workarounds left in CONTRACTS.md or README.md;
  - the mention-order pin from the deleted `lean-v3-mention-order-follows-constructors`.

  For the mention-order pin: a Template that passes one NounPhrase as `conjunction`'s `parts` and
  as its `subject` gives the subject the first mention.
- Additive `lean-v3-*` tickets that land before the conversion are carried over by the script.
