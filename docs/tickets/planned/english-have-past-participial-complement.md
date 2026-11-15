---
needs: []
---

# Read *have* with an Object and a past-participial Complement

## Why

*Melira's Keepers* prints "This creature can't have counters put on it." Its one Reading gives
*have* a single Object, the noun phrase "counters put on it", whose participle is a postmodifier:
the creature can't possess counters that were put on it. The card forbids the putting. Its rulings
say "Melira's Keepers can't have any kind of counters placed on it, not just -1/-1 counters" and
"If Melira's Keepers would enter with a counter on it, it simply enters without that counter".

English has the construction: *have* takes an Object and a past-participial Complement in the
complex catenative construction, "She had the house painted", "I had my wallet stolen" (CGEL,
Ch. 14 §5.6.2, p. 1245, [60ii]), with a sense in which the subject undergoes the event and
arranges nothing (Ch. 14 §5.4, p. 1236). The lexicon declares *have*'s complex catenative frame
with a bare infinitival only ("you may have target player draw a card"), and the comment on
*haven't*, the entry after it, says no frame selects a past participle.

About a dozen lines print it, by a regular expression over the corpus for a form of *have*, a
noun phrase and a past participle with its own dependent: "each creature that had counters put on
it this way" (*Agitator Ant*), "each creature that had a +1/+1 counter put on it this way"
(*Fangs of Kalonia*), "you may have that damage dealt to you instead" (*Blood of the Martyr*),
"If an opponent had three or more cards put into their graveyard from anywhere this turn", and
"can't have -1/-1 counters put on them".

*Agitator Ant*'s and *Fangs of Kalonia*'s lines also have a Reading that gives *have* an Object and
a Complement in the wrong form: "put" as a bare infinitival whose Object, shifted to the end, is
"this way".

## Done when

- *Melira's Keepers*, *Agitator Ant*, *Fangs of Kalonia* and *Blood of the Martyr* each have a
  Reading in which *have* takes the noun phrase as its Object and the past-participial verb phrase
  as its Complement ("counters" and "put on it"). Each is a judgment entry, written and
  adjudicated as `crates/english/judgments/README.md` says.
- Every entry of `judgments/causative_have.txt` still holds.
- The comments on *have* and *haven't* in `closed_class.ron` say what *have*'s frames select.
- The Reading of *Agitator Ant*'s and *Fangs of Kalonia*'s lines that takes "put" as a bare
  infinitival with "this way" as its shifted Object is judged: kept as English, or removed by
  tightening what admits it.
- The landing lists every unit that gains a Reading. A gained Reading that is no English is a
  defect of the change, fixed in it.
- `cargo xtask english strata --check`, `cargo test -p sylex_vocabulary`,
  `cargo test -p sylex_syntax` and `cargo test -p sylex_english` pass.

## Not in scope

Who performs the event the participle names, and how a `.syl` term writes the construction.

## Pointers

- `crates/vocabulary/lexicon/english/closed_class.ron` (*have*), `crates/vocabulary/src/schema.rs`
  (`Catenation`), `crates/syntax/src/frames.rs` (`ComplexCatenative`)
- `crates/english/judgments/causative_have.txt`, `crates/english/judgments/library_positions.txt`
- `docs/design/english.md` §3
