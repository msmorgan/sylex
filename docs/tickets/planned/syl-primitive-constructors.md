---
needs: []
---

# `.syl` names AST constructors only in primitives, capitalized

**Why.** A primitive either takes its constructor's Lean name (`primitive activated(…): Ability` is
`Ability.activated`) or names it with a leading dot (`primitive abilities(keyword: KeywordTerm):
AbilityLost = .term`), and a bare word anywhere may resolve straight to a constructor or enum word
of the expected sort (`docs/decisions/syl.md` §3, step 4: `stat(power)` reaches `Stat.power` with
no declaration). So the AST's constructor names live in Lean, cards and Templates use them, and an
identifier cannot be traced to what it builds. Implicit conversions hide the same thing.

**Design.**
- Every primitive names its constructor, capitalized, after its signature, letter for letter the
  Rust AST's variant name: `primitive activated(cost: Cost, action: Action, …): Ability =
  Activated { spelling = … }`. The name is required even when it only capitalizes the
  primitive's own name. The leading-dot form is gone.
- One primitive per constructor: a primitive is the one explicit mapping from a word to the AST.
- Capitalized identifiers name constructors and appear only in a primitive's constructor clause;
  elsewhere the only capitalized identifiers are sort names in signatures (`other: NounPhrase`,
  `Option(List(Cost))`). The loader refuses a capitalized identifier anywhere else. `lexical` and
  `spelling` are metadata fields, so the rule does not apply inside them.
- No bare constructor fallback: `syl.md` §3's step 4 is deleted, and every constructor or enum
  word used today without a primitive (`power`, `battlefield`, …) gets one (`primitive power:
  Stat = Power`), with a `spelling` where it has a printed word.
- No implicit conversions except `Option`: a value stands only at its own sort, and every wrap
  into another sort is a primitive written at the call site, retiring `syl.md` §3's direct sort
  atoms. Color words are primitives at `Color` only, and a `ColorOrColorless` or a mana symbol
  wraps one explicitly. Integer literals are values only at the integer sorts, and an `Amount` or
  a generic mana symbol wraps one explicitly. Mana-cost wrappers are Templates over primitive
  chains, as `colorless` already is.
- An `ability` declaration also declares its name at `KeywordLabel` (a second declaration, not a
  conversion), so a keyword with parameters is still named bare (`byKeyword(kicker)`); a term is
  written `theKeyword(kicker)`. Engine-defined keywords declared as primitives also get a
  `KeywordLabel`.
- Deeds are being made explicit in `.syl`; this ticket does not decide deed registration.
- Card files drop the frame-unpacking shorthand (`card adventurer({ normal = …, adventure = … })`)
  for `adventurer(normal = …, adventure = …)`.
- `Option` stays implicit: a value at an `Option(T)` position is a `T`, `some` is never written,
  and an omitted argument or field takes its declared default.

**Done when.**
- `.syl` follows the Design: every primitive names its capitalized constructor, capitalized
  identifiers appear nowhere else outside `lexical` and `spelling`, every constructor and enum
  word has a primitive, and the only implicit wrap left is `Option`.
- The owner has approved the wrapper words' names before the corpus is respelled.
- `sylex_syl` parses and formats the constructor clause, and the capitalization rule is enforced
  (the landing says where).
- `docs/decisions/syl.md` §1 to §3 state the rules: the primitive rule, the resolution steps
  without step 4, and no direct sort atoms.
- The change is renaming only: `lake exe dumpValues` gives byte-identical dumps before and after.
- `cargo xtask lean check`, `cargo xtask dsl fmt --check`, `cargo xtask dsl differential`, and
  `cargo test -p sylex_syl` with clippy pass.
- A capitalized identifier outside a constructor clause, a primitive without one, a bare word that
  names no declaration, and two primitives for one constructor are each refused, with a negative
  test apiece.

**Not in scope.** Declared implicit wraps (`syl-implicit-coercions`).

**Pointers.** `docs/decisions/syl.md` §1 to §3,
`crates/syl/`, `lean/SylGen/`, `crates/xtask/src/dsl.rs`.
