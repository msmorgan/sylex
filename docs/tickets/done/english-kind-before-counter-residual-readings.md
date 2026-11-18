---
needs: [english-counter-kind-premodifies-counter]
---

# A kind's homograph rebuilds the kind and its head

## Why

A counter's kind is a premodifier of the noun "counter", licensed by `KindLicense`. Where a kind
name has a homograph, the homograph and the head word form another constituent of their own,
beside the kind reading, in five units:

- **The homograph as a verb, with "counters" its Object**: *Deadly Designs* line 2 ("five or
  [more plot counters on this enchantment]", "plot" a verb, 3 of 4 Derivations); *Cryptex* line 2
  (the same with "unlock", and "unlock" as a bare infinitival under "has", 11 of 14); *Fylgja*
  line 2 and *Ursine Fylgja* line 1 ("enters with four, [healing counters on it]", 3 of 5).
- **The homograph as a participle premodifying "counter"**: *Sensei Golden-Tail* line 2
  ("training", 2 of 4).

In each, the two words make a constituent that begins with the kind word or ends with the head
word, which is the kind's own analysis built a second way. Words that meet by accident across a
constituent boundary are a different analysis: *Ajani Unyielding*'s −9 line reads "five loyalty"
as a Subject and "counters on each other planeswalker" as its verb phrase in a relative clause,
and *Stalwart Successor* line 2 needs "the first time [counters have been put on that
creature]". Ajani's other Readings with that Subject (6 of 11) attach "on each other
planeswalker" above a clause "[five loyalty] [counters]" that ends with the head word, and
rebuild the pair.

In Oracle text every word ending in "-ing" or "-ed" printed before "counter" or "counters" is a
kind name in `crates/vocabulary/catalogs/counters.ron` (flying, hatchling, healing, mining,
blessing, awakening, feeding, hatching, contested, exalted, decayed, training, enlightened).

## Design

- A kind and its head hold their words as a multiword term does (`docs/design/lexicon.md` §5): a
  word that carries `KindLicense` `Kind`, followed by a word that carries `Head`, keeps its other
  analyses only where the two straddle a constituent boundary. Where they would rebuild the pair,
  only the kind's own analysis, the Production that admits the head (`:= Head`), holds them. The
  rule lives in the lexicon and the engine, keyed on the license feature, never on a Production
  per kind.
- No grammar refusal of a participle premodifying "counter": the participle premodification
  schema gets no host equation and the engine no capability for one. Where the participle is a
  kind name's homograph, the rule above removes it.

## Done when

- Deadly Designs line 2, Cryptex line 2, Fylgja line 2, Ursine Fylgja line 1 and Sensei
  Golden-Tail line 2 keep only the kind reading of the word before "counter(s)".
- Each of the thirteen units read with a spurious kind when "time counter" was one word
  (Sibylline Soothsayer, Gandalf of the Secret Fire, Ojer Pakpatiq, Taigam, Doom's Time Platform,
  Spark Rupture, Cryptex, Deadly Designs, Fylgja, Ursine Fylgja, Sensei Golden-Tail, Fangs of
  Kalonia, Hindervines) has no Reading that reads the word before "counter(s)" as anything but
  its kind, or the landing says which Readings remain and why each is a reading of the English.
- Ajani Unyielding's −9 line and Stalwart Successor line 2 keep their straddling Readings.
- The census shows which Readings changed on the supported corpus and why, and `cargo xtask
  english strata --check`, `cargo test` and `cargo clippy --tests -- -D warnings` pass for every
  changed crate.

## Pointers

`crates/english/grammar/english.fug` (`KindPremodifier`, `feature KindLicense`);
`crates/syntax/src/chart.rs` (`Seams`, `atomic_spans`); `crates/syntax/src/reading.rs`
(`atomic_reading`); `crates/vocabulary/src/lib.rs` (`counter_kinds`); `docs/design/lexicon.md`
§5; `docs/design/english.md` §3 (License features).
