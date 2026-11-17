---
needs: []
---
# Remove unattested determinative *next*

## Why

`crates/vocabulary/lexicon/english/closed_class.ron` declares a deictic temporal Determinative `next` (`det:next`) alongside `last` (`det:last`). Unlike `det:last` (needed on 72 Corpus Units such as *if no spells were cast last turn*), `det:next` is needed on 0 Corpus Units and 0 Judgments: Oracle never prints bare determinerless *next* (*next turn*, *next step*), and CGEL restricts determinerless temporal *next* to calendar/deictic nouns rather than sub-steps or phases (`∗next untap step`, `∗next end of combat`; CGEL, Ch. 5, §4, p. 356 fn. 16; Ch. 8, §6.3, p. 698). Every one of `det:next`'s 134 corpus occurrences immediately follows a genitive (`'s` / `s'`), splitting phrases such as *during its controller's next untap step* into a spurious fused-head genitive Complement (`during [its controller's]`) plus a bare temporal Adjunct (`[next untap step]`) beside the intended `GenitiveDetermined` Noun Phrase with ordinal adjective `adj:next` (CGEL, Ch. 5, §9.3(c), p. 416).

## Design

- Delete `(lemma: "next", distribution: Deictic, noun_kind: Some(Temporal))` from `determinatives` in `crates/vocabulary/lexicon/english/closed_class.ron`, keeping `det:last`, `adj:next`, and `adv:next`.
- Update the comments on `FusedSubjectDeterminerHead` in `crates/english/grammar/english.fug` and in `crates/english/judgments/genitive_heads.txt` so they no longer claim *during its controller's next untap step* admits a fused-head genitive plus Adjunct Reading, and assert `!NounPhrase "this turn's" as Complement of at` and `!NounPhrase "its controller's" as Complement of during` on *Triton Tactics*.

## Done when

- `det:next` is absent from `crates/vocabulary/lexicon/english/closed_class.ron`.
- *Triton Tactics* (`crates/english/judgments/genitive_heads.txt`) has no `FusedSubjectDeterminerHead` Reading for `"this turn's"` or `"its controller's"`, while *Deadpool, Trading Card*, *Mirko, Obsessive Theorist*, and *Pelt Collector* keep their fused-head genitive Readings.
- `cargo xtask english strata --check` against a pre-change baseline shows 0 lost faces, 0 verdict regressions, and 0 Corpus Units with increased Readings.
- `cargo test -p sylex_vocabulary`, `cargo clippy -p sylex_vocabulary --tests -- -D warnings`, `cargo test -p sylex_english`, and `cargo clippy -p sylex_english --tests -- -D warnings` pass.

## Not in scope

- `det:last` (*last turn* on 72 Corpus Units, and the 2 Corpus Units — *Halls of Mist*, *Tangle Kelp* — that print *during its/their controller's last turn*).

## Pointers

- `crates/vocabulary/lexicon/english/closed_class.ron` (`determinatives`, lines 6–8)
- `crates/english/grammar/english.fug` (`TemporalLocation`, `FusedSubjectDeterminerHead`)
- `crates/english/judgments/genitive_heads.txt`
- `docs/design/english.md` (§3, §4, §6)
