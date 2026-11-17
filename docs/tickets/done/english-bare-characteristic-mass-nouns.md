---
needs: []
---

# Prevent bare unquantified relational and characteristic mass nouns from standing alone under `BareMass`

## Why

Several nouns declared `Mass` or `[Count, Mass]` (`power`, `toughness`, `mana value`, `mana cost`,
`control`, `loyalty`, `time`) never stand as a bare single-word `BareMass` `NounPhrase` in Oracle:

- Characteristic nouns (`power`, `toughness`, `mana value`, `mana cost`): `BareMass` admits bare
  `"with mana value"` / `"with power"` / `"with toughness"` as a complete Preposition Phrase on
  `spell` or `creature`, leaving its `PostpositiveAdjective` (*"less than or equal to …"*, *"equal
  to …"*) to jump outside `with …` onto the outer noun (`[[a spell with mana value] [less than or
  equal to …]]`). Across the corpus, single-word `BareMass` on these nouns accounts for 8,848
  derivations across 446 units (including 6,179 `PostpositiveAdjective`-jump derivations across 193
  units: *Sphere of Annihilation* 972/1,349, *Maralen, Fae Ascendant* 944/4,515, *Tetsuo, Imperial
  Champion* 369/1,179, *Mockingbird* 260/702, *Brain in a Jar* 204/839).
- Relational `control`: bare `"control"` stands with its `of` complement (*"gain control of target
  creature"*) or determined (*"under its owner's control"*), never as a bare single-word `BareMass`
  NP; single-word `BareMass` on `"control"` produces 429 bogus derivations across 6 cards (*Tawnos's
  Coffin* 304/476, reading `"under [its owner's]"` as an independent genitive followed by a bare
  relative `[control tapped …]`!).
- `time`: `nouns.ron` declares `(lemma: "time", kind: Temporal, distribution: Serial, countability:
  [Count, Mass])`, though every `Mass` node of `noun:time` in the corpus is spurious (275 derivations
  across 7 cards: *Sibylline Soothsayer* 108/288, *Ojer Pakpatiq* 48/101 reading `"with [three]
  [time counters on it]"` as a bare relative where mass `"time"` is the Subject of the verb
  `action:counter`, and *Plea for Power* 2/4 duplicating `vote_option:time`!).
- `loyalty`: bare `"loyalty"` without an `Amount` (`starting loyalty X`) or determiner (`its loyalty`)
  produces 112/224 bogus derivations on *Spark Rupture*.

## Design

- Drop `Mass` from `noun:time` in `nouns.ron` (`countability: [Count]`, the default).
- Distinguish characteristic/relational mass nouns (`power`, `toughness`, `mana value`, `mana cost`,
  `control`, `loyalty` — vs substance/state mass nouns that stand bare alone: `damage`, `combat
  damage`, `life`, `mana`, `exile`, `combat`, `radiation`, color nouns, coin/choice nouns `heads`,
  `tails`, `evidence`, `chaos`, `fame`, `fortune`, `money`, `peace`, `silence`, `war`, `tribute`) by
  grammatical distribution (`NounDistribution` or `BareMass` licensing, respecting `english.md` §3's
  ban on single-word flags) so a bare characteristic or relational mass nominal stands under
  `BareMass` only when licensed by a complement, premodifier, postmodifier or `Nominal` coordination
  (`"control of target creature"`, `"power 3 or less"`, `"mana value less than X"`, `"turn order"`,
  `"power and toughness"`), never as a bare single word.

## Done when

- `noun:time` is `Count`-only, removing the `time counters` relative-clause misparses (*Ojer
  Pakpatiq*, *Sibylline Soothsayer*, *Taigam, Master Opportunist*, *Doom's Time Platform*) and the
  duplicate `noun:time` Reading on *Plea for Power*.
- Single-word `BareMass` is refused on characteristic/relational mass nouns (`*[a spell with mana
  value] [less than 3]` on *Maralen* and *Sphere of Annihilation*, and `under [its owner's] [control
  tapped …]` on *Tawnos's Coffin*), while `"gain control of target creature"`, `"under its owner's
  control"`, `"with mana value less than 3"`, `"with power 3 or less"`, `"starting loyalty X"`,
  `"power and toughness"`, `"its power and toughness"` and `"has mana value equal to 1 plus …"` read
  cleanly.
- Every mass noun in `nouns.ron` is checked against the corpus so no supported Corpus Unit loses its
  verdict.
- `cargo xtask english strata --check`, `cargo test -p sylex_vocabulary -p sylex_english` and
  `cargo clippy -p sylex_vocabulary -p sylex_english --tests -- -D warnings` pass.

## Pointers

- `crates/vocabulary/lexicon/english/nouns.ron`: `power`, `toughness`, `mana value`, `mana cost`, and
  all `Mass` nouns.
- `crates/english/grammar/english.fug`: `BareMass`, `PostpositiveAdjective`, `feature NounDistribution`.
- `docs/design/english.md` §3, §6.
