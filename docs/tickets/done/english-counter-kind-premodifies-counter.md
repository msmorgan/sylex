---
needs: []
---

# A counter's kind is a modifier before the noun "counter"

## Why

Every counter kind is one compound-noun lexeme, "+1/+1 counter", "charge counter", "flying
counter", built from its `counter` declaration's `lexical` lemma
(`crates/vocabulary/src/lib.rs`, the `SharedHeadCompound` class). *Travel Preparations*'s "a
+1/+1 counter" is a single `counter:+1/+1` leaf with no structure. Eight in-scope cards share one
"counter" across several kinds: *Denry Klin, Editor in Chief* ("a +1/+1, first strike, or
vigilance counter"), *Frankenstein's Monster*, *Me, the Immortal*, *Assaultron Dominator*,
*Reluctant Role Model*, *Owen Grady, Raptor Trainer*, *T-45 Power Armor* and *The Night of the
Doctor*. The grammar reads those through `WordPartCoordination`, whose conjuncts are a `Measure`
(`MeasureBase`), a `KeywordPhrase` (`KeywordBase`) or the last whole compound (`WholeCompound`):
the parts are words the language has anyway, and a `.syl` term has no form for a coordination of
word parts.

## Design

- "Counter" is a noun, and a kind is a premodifier of it: a slash measure ("+1/+1"), a keyword
  ("flying", "first strike") or a kind name ("charge", "loyalty", "time"). A kind name is a word
  of its own, declared by its `counter` declaration, whose lemma is the name without "counter".
- A coordination of kinds is an ordinary coordination of premodifiers under one head.
- A license feature (`docs/design/english.md` §3) restricts the kinds: "counter" is the one noun
  a kind premodifies, and it takes a measure, a keyword or a declared kind name there and no other
  noun premodifier; a kind name stands only there, so no other noun takes it ("charge creature"
  has no Reading) and it never heads a nominal.

## Done when

- "a +1/+1 counter", "a charge counter" and "a flying counter" each parse as a premodifier and
  the head noun "counter", with one Reading where they have one today, and the eight cards above
  parse as a coordination of premodifiers.
- No `counter:*` compound lexeme, `SharedHeadCompound` class or `MeasuredCompound` surface
  structure remains, and `WordPartCoordination`, `MeasureBase`, `KeywordBase` and
  `WholeCompound` are deleted unless another wording still needs them, which the landing names.
- Every `counter` declaration in `plugins/builtin/definitions/counter/` carries the lemma the
  design gives it; a keyword counter's or measure counter's declaration that then only repeats
  its keyword or measure is listed for `syl-counters-noun-phrase`.
- `english-measure-nominal-compound-part` is rewritten against the new productions or closed.
- The census shows which Readings changed on the supported corpus and why; `cargo xtask english
  strata --check`, `cargo xtask dsl check`, `cargo test` and `cargo clippy --tests -- -D warnings`
  pass for every changed crate; and `cargo xtask english forests` is rerun on `default` after
  integration.

## Not in scope

- The `.syl` calls that put and remove counters: `syl-counters-noun-phrase`.

## Pointers

`crates/vocabulary/src/lib.rs` (counter lexemes); `crates/english/grammar/english.fug`
(`WordPartCoordination`, `CompoundCoordination`, `feature CompoundPart`, `Premodifier`);
`plugins/builtin/definitions/counter/`;
`cargo xtask english forestwalk --card "Denry Klin, Editor in Chief" --line 1`.
