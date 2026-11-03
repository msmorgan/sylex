---
needs: []
---
# `.syl` names AST constructors only in primitives, capitalized

Standard constraints apply. Step 0 of the semantics-v3 rewrite: `.syl` carries no trace of Lean's
naming. Today a primitive either takes its constructor's Lean name (`primitive activated(…):
Ability` is `Ability.activated`) or names it with a leading dot (`primitive abilities(keyword:
KeywordTerm): AbilityLost = .term`), and a bare word anywhere may resolve straight to a constructor
or closed enum word of the expected sort (`docs/decisions/semantics-v3.md` §4.5 item 2, step 4;
`stat(power)` reaches `Stat.power` with no declaration). The AST's constructor names therefore live
in Lean, and cards, templates and Definitions use them.

## Change

- **Every primitive names its constructor, capitalized,** after its signature:
  `primitive activated(cost: Cost, action: Action, …): Ability = Activated { spelling = … }`,
  `primitive abilities(keyword: KeywordTerm): AbilityLost = Term { spelling = … }`. The name is
  required even when it only capitalizes the primitive's own name. The leading-dot form is gone.
- **Capitalized identifiers name constructors, and appear only there.** Outside a primitive's
  constructor clause, the only capitalized identifiers in `.syl` are sort names in signatures
  (`other: NounPhrase`, `Option(List(Cost))`). Cards, templates, Definitions and rules never write a
  constructor; they reach every constructor through its primitive. The loader refuses a capitalized
  identifier anywhere else.
- **No bare constructor fallback.** Step 4 of §4.5 item 2 is deleted: every constructor or enum word
  a declaration uses today without a primitive (`power`, `battlefield`, …) gets a primitive file,
  `primitive power: Stat = Power`, with a `spelling` where it has a printed word. Many of these are
  one-line renames; that is intended.
- **One primitive per constructor** stays the rule; a primitive is the one explicit mapping from a
  word to the AST.
- **The Lake generator maps the capitalized name to Lean's constructor** (lowercasing the first
  letter, and quoting the words that collide with Lean as it already does). No Lean name, no
  trailing underscore and no leading dot remains in `.syl`.
- `deckmaste_dsl` parses and formats the constructor clause, and enforces the capitalization rule
  (or `semantics` does, at load; say which).
- Update §4.4 (bare words, the leading dot) and §4.5 (the primitive rule, names, dispatch steps)
  of `docs/decisions/semantics-v3.md`.

- **Color words get primitives too:** `white`, `blue`, `black`, `red` and `green` are declared at
  each sort they build (`Color`, `ColorOrColorless`, `ManaSymbol`), like any other constructor.
  Integer literals stay literals, and a Definition name still stands for its keyword at
  `Ability`, `KeywordLabel`, `KeywordTerm` and `Deed` (§4.5's direct sort atoms, less the colors).

## Tests

- Renaming only: `lake exe dumpValues` gives byte-identical value dumps before and after.
- `cargo xtask lean check`; `cargo xtask dsl fmt --check`; `cargo xtask dsl differential`;
  `cargo test -p deckmaste_dsl` and clippy.
- A capitalized identifier outside a primitive's constructor clause, a primitive without one, a bare
  word that names no declaration, and two primitives for one constructor are each refused, with a
  negative test apiece.
