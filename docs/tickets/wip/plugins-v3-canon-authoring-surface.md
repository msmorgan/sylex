---
needs: [plugins-v3-builtin-cleanup, plugins-v3-primitive-template-boundary]
---
# Authoring surface for canon cards: anonymous `card :=`, `cardNamed`, record forms and bare words

Standard constraints apply. This ticket builds the features that `plugins-v3-canon-cleanup` will
use to rewrite the 908 canon cards. It does not rewrite them. The Rust reader
(`deckmaste_semantics_v3`) parses the Lean source itself, so every feature needs a matching Lean
elaborator and Rust reader/lowering path, plus a test on each side. Develop test-first.

## Features

1. **Anonymous `card := <term>`.**
   - The declaration key is the printed name. For multi-part forms, join the part names with
     ` // `:
     - `front`/`back` for `transforming` and `modalDfc`.
     - `left`/`right` for `split` and `sharedLineSplit`.
     - `normal`/`adventure` for `adventurer`.
     - `normal`/`alternative` for `flip`.
   - `leveler` and `prototype` use `inner`'s name.
   - A card with no name is an elaboration error. `Characteristics.name` is `Option String`.
   - Key handling must not split the key on `.`. Rust `bare()` uses `rsplit('.')`.
   - Generate `ok_<key>` theorems under `Name.mkSimple`.
2. **`cardNamed "<name>"`.** This is a term elaborator that looks a card up by key, plus a way
   for proofs to reach its `ok_` theorem.
   - Test-only cards in `lean/Semantics/Proofs/Authoring.lean` (`testCard`, `bundleCard`,
     `authoredSurface`, `authoredSpree`) and the inline cards in
     `crates/xtask/tests/suite/plugins_v3_declarations.rs` move to the new syntax, each with a
     distinct test name such as `name := "Test Card"`.
3. **Record forms for `Card`.**
   - `card := { name := … }` must elaborate to a single-faced card. A `Coe` cannot do this: Lean
     does not apply coercions to a structure instance whose expected type is an inductive. It
     needs a `structInst` elaborator like `elabCharacteristicBundleStruct`
     (`Authoring.lean:979`).
   - The same applies to `{ … }` in a `CardFace` position.
   - Multi-part constructors take braced named fields:
     - `transforming { front, back }`
     - `modalDfc { front, back }`
     - `split { left, right }`
     - `sharedLineSplit { shared, left, right }`, whose halves are `SharedLineHalf`, not
       `CardFace`.
     - `adventurer { normal, adventure }`
     - `flip { normal, alternative }`
     - `leveler { inner, bands }`
     - `prototype { inner, alternative }`, where `alternative` is a `PrototypeFrame`.
   - Use existing structure literals for `LevelBand` (`{ range := between 1 2, power := 2,
     toughness := 3, text := [...] }`) and `PrototypeFrame` (`{ cost := [1, blue, blue],
     power := 2, toughness := 1 }`). Keep the `levelBand` and `prototypeAlt` template files
     until canon cleanup migrates their callers; no builtin declaration keys are removed.
   - Remove `body` and `singleFacedCard` from the `card` surface in `Authoring.lean`,
     `reader.rs`, `bundle.rs` and `lower/mod.rs`.
4. **Numerals for stats.** Add `OfNat (Option Amount) n`, so that `power := 3` works.
   `Characteristics.power`, `toughness`, `loyalty` and `defense` are `Option Amount`; there is
   no `PrintedStat`. `Authoring.inspect` must read through it to the `Amount.lit` leaf
   (`lean/CONTRACTS.md`). Add the Rust numeral to `ManaSymbol` coercion too; Lean already has
   `OfNat ManaSymbol`.
5. **Default `copyCharacteristics`'s `typesAdded` to `false`.**
6. **Bare words.** Add bare words for every sort currently written as a dot constructor in
   `plugins_v3/canon/cards`. Run `rg -o '(^|[ (\[,])\.[a-z]\w*' plugins_v3/canon/cards` for the
   current list of 137; it includes card types, colors, supertypes, comparators, turn parts,
   roundings and quality ops.
   - Keep `.some` on `ColorSpec`: bare `some` is the prelude Option constructor, so
     `some []` for colorless would collide with and obscure that meaning.
   - A bare word may share its name with an existing declaration of another return type. For
     example, rounding `up : RoundMode` sits beside `primitive up : Delta Amount` through
     `(name, return_type)` dispatch. Pin that both `half up (powerOf …)` and
     `getsPt it (up 2) (up 2)` elaborate.
   - Rewrite the builtin's own dot forms for these sorts in the same change. Keep
     `zone .exile none` in `exileZone`: the untyped dependency scan would otherwise create
     a false cycle with the `exile` action, which depends on `exileZone`.
     Match-pattern dots remain to distinguish constructors from binders. Internal and
     semantic-expression constructors, mana constructors, and the qualified
     `Semantics.QualityOp.sets` computation argument retain their existing spelling.

## Docs and tests

- Update `lean/CONTRACTS.md:143` (single-faced card syntax) and `lean/README.md:177`.
- Rewrite `deckmaste_semantics_v3/tests/{bundle,lower}.rs` assertions that assume
  `card <ident> : Spelled` or identifier lookup. Rewrite them to keep what they claim, not
  retire them.
- The existing 908 canon files must keep loading, so the old syntax is still accepted until
  `plugins-v3-canon-cleanup` removes it.

## Acceptance

- Every feature above has a Lean pin and a Rust test.
- All existing expansion samples remain unchanged, including the builtin dot-form rewrite.
  Only `copyCharacteristics : List CopyExcept` gains samples for default omission.
- Gates: `cargo xtask lean check`. Run tests and clippy for `deckmaste_semantics_v3`,
  `deckmaste_lowering_v3` and `xtask`.

## Expansion baseline workflow

Save before editing; this single invocation includes builtin and canon, with canon
expanded over builtin:

```bash
cargo xtask expansions --out docs/evidence/plugins-v3-expansions/plugins-v3-canon-authoring-surface/before
```

After the change, require exactly one reviewed difference, zero renames and exit 0:

```bash
cargo xtask expansions --compare docs/evidence/plugins-v3-expansions/plugins-v3-canon-authoring-surface/before --expect 'copyCharacteristics : List CopyExcept'
```

Baseline hardening landed after this ticket was drafted. `sample_invocations` now samples
omission of every defaulted parameter. Feature 5 therefore adds omission samples for
`typesAdded : Bool := false`; each must match the corresponding explicit `false` sample.
No existing sample may change, and the other 2,980 declarations must have zero differences.
The saved baseline is immutable; do not regenerate it. No rename map is needed.

The saved format is `before/snapshot.json` (version 2). Save commands refuse an
existing snapshot unless `--force` is deliberately added; unrelated nonempty
directories are always refused. Comparison writes nothing unless `--out` is also
given. Review the sampling summary for constructor omissions and capped products.
