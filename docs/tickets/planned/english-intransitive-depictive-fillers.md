---
needs: [english-comparative-and-inequality-depictives]
---
# Restrict `AdjectivalDepictive` and `DepictiveAdjunct` to intransitive Adjective Phrases

## Why

`InterleavedDepictive` in `crates/english/grammar/english.fug` requires `ComplementKind = Intransitive`, while `AdjectivalDepictive` (`Depictive -> filler:AdjectivePhrase`) and `DepictiveAdjunct` (`PredicateAdjunct -> filler:AdjectivePhrase`) only exclude `ComplementKind != Amount`. Across the corpus, every needed occurrence of `AdjectivalDepictive` (95 Corpus Units) and `DepictiveAdjunct` (30 Corpus Units) is bare `"tapped"` (`ComplementKind = Intransitive`), whereas admitting complemented `AdjectivePhrase`s (`ComplementKind = PP`) produces spurious `DepictiveAdjunct` and `AdjectivalDepictive` Readings with `adj:attached` (*Strong Back*: *gets +2/+2 for each Aura and Equipment attached to it*, *Mantle of the Ancients*, *Wyleth, Soul of Steel*) and `adj:tied` (*tied for most life*).

## Design

- Replace `filler.ComplementKind != Amount` with `filler.ComplementKind = Intransitive` on `AdjectivalDepictive` and `DepictiveAdjunct` in `crates/english/grammar/english.fug`, naming witness *Strong Back* in the gate comment (`docs/design/english.md` §6).
- Add a Judgment assertion on *Strong Back* (or *Mantle of the Ancients*) pinning `!PredicateAdjunct "attached to it"` while keeping `PastParticipialAbsolute` (*Daybreak Coronet*: *Enchant creature with another Aura attached to it*) and `DepictiveAdjective` (*Infinite Reflection*, *Arachnus Spinner*) intact.

## Done when

- `AdjectivalDepictive` and `DepictiveAdjunct` require `filler.ComplementKind = Intransitive`.
- *Strong Back* has no `DepictiveAdjunct` Reading for `"attached to it"`, and *Daybreak Coronet*, *Infinite Reflection*, and *Arachnus Spinner* keep their Readings.
- `cargo xtask english strata --check` against a pre-change baseline shows 0 lost faces, 0 verdict regressions, and 0 Corpus Units with increased Readings.
- `cargo test -p sylex_english` and `cargo clippy -p sylex_english --tests -- -D warnings` pass.

## Not in scope

- Cross-host attachment ambiguity on `DepictiveAdjective` (`VerbPhrase -> VerbPhrase adjunct:AdjectivePhrase`, e.g. *Blastfire Bolt* and *Remove Enchantments* attaching `attached to …` to the Noun Phrase via `action:attach` vs to the Verb Phrase via `adj:attached`), and verbal-vs-copular passive ambiguity after *be* (*is attached*, *is tapped*).

## Pointers

- `crates/english/grammar/english.fug` (`InterleavedDepictive`, `AdjectivalDepictive`, `DepictiveAdjunct`, `DepictiveAdjective`, `PastParticipialAbsolute`)
- `docs/design/english.md` (§4, §6)
