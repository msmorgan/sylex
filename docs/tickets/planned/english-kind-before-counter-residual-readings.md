---
needs: [english-counter-kind-premodifies-counter]
---

# Thirteen units read a counter's kind as another word

## Why

A counter's kind is a premodifier of the noun "counter", licensed by `KindLicense`. Thirteen units
have more Readings than they had when "time counter" was one compound word, each a spurious one:

- **A kind's homograph as a noun or verb, with "counters" as a verb** (11 units). After a fused
  numeral, "with three [time counters on it]" reads "time" as an ordinary noun and "counters" as
  a verb: *Sibylline Soothsayer* (420 to 672 Derivations), *Gandalf of the Secret Fire*, *Ojer
  Pakpatiq*, *Taigam*, *Doom's Time Platform*, *Spark Rupture* ("loyalty"), *Cryptex* ("unlock"
  as a verb), *Deadly Designs* ("plot" as a verb), *Fylgja* and *Ursine Fylgja* ("healing" as a
  participle), *Sensei Golden-Tail* ("training" as a participle).
- **A fused measure under a determiner before "counter"** (2 units): *Fangs of Kalonia* ("had [a
  +1/+1] [counter put on it]", 123 to 127) and *Hindervines* ("with [no +1/+1] …", 60 to 65).

Each structure is one English admits, and the compound word's atomicity was what excluded it. No
constraint stated over the grammar's categories was found that is true of the language.

In Oracle text every word ending in "-ing" or "-ed" printed before "counter" or "counters" is a
kind name in `crates/vocabulary/catalogs/counters.ron` (flying, hatchling, healing, mining,
blessing, awakening, feeding, hatching, contested, exalted, decayed, training, enlightened), so no
in-scope card prints a participle that modifies "counter".

## Open questions

- **Whether a participle may premodify "counter".** None is printed. Refusing one removes the
  *Fylgja*, *Ursine Fylgja* and *Sensei Golden-Tail* Readings, and needs an equation on the host
  in the participle premodification schema, which the engine does not support today.
- **What restores a kind's hold on the word before "counter".** The other ten units need the
  grammar to prefer, or require, the kind reading of a word that is a kind name when "counter"
  follows it. Options: a lexical mechanism like the multiword-term straddle rule
  (`docs/design/lexicon.md` §5), applied to a kind and its head; or leaving the Readings to the
  step that ranks them (`docs/design/pragmatics.md`).

## Done when

- The open questions are answered with the units that decide them.
- Each of the thirteen units has the Derivation count it had before the split, or the landing
  says which Readings remain and why each is a reading of the English.
- The census shows which Readings changed on the supported corpus and why, and `cargo xtask
  english strata --check`, `cargo test` and `cargo clippy --tests -- -D warnings` pass for every
  changed crate.

## Pointers

`crates/english/grammar/english.fug` (`Kind`, `KindPremodifier`, `feature KindLicense`);
`crates/vocabulary/catalogs/counters.ron`; `docs/design/english.md` §3 (License features), §6;
`cargo xtask english forestwalk --card "Sibylline Soothsayer" --line 1`.
