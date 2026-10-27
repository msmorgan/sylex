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
   - Decide what replaces the positional `levelBand` and `prototypeAlt` helpers, and record the
     choice here.
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
   - A dot constructor that has no reasonable bare word stays and is listed here with the
     reason.
   - A bare word may share its name with an existing declaration of another return type. For
     example, rounding `up : RoundMode` sits beside `primitive up : Delta Amount` through
     `(name, return_type)` dispatch. Pin that both `half up (powerOf …)` and
     `getsPt it (up 2) (up 2)` elaborate.
   - Rewrite the builtin's own dot forms for these sorts in the same change.

## Docs and tests

- Update `lean/CONTRACTS.md:143` (single-faced card syntax) and `lean/README.md:177`.
- Rewrite `deckmaste_semantics_v3/tests/{bundle,lower}.rs` assertions that assume
  `card <ident> : Spelled` or identifier lookup. Rewrite them to keep what they claim, not
  retire them.
- The existing 908 canon files must keep loading, so the old syntax is still accepted until
  `plugins-v3-canon-cleanup` removes it.

## Acceptance

- Every feature above has a Lean pin and a Rust test.
- The expansion baseline over `plugins_v3/canon` and `plugins_v3/builtin` is unchanged, apart
  from the builtin dot-form rewrite, which must also show zero changes.
- Gates: `cargo xtask lean check`. Run tests and clippy for `deckmaste_semantics_v3`,
  `deckmaste_lowering_v3` and `xtask`.
