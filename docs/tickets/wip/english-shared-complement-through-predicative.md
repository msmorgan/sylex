---
needs: []
---

# Read a Shared Complement that one conjunct reaches through a predicative

## Why

*Dream Fighter* prints "Whenever this creature blocks or becomes blocked by a creature, this
creature and that creature phase out." Its only Reading coordinates two complete verb phrases,
an intransitive "blocks" and "becomes blocked by a creature", so "a creature" belongs to the
second conjunct alone. The cards mean it of both: *Engulfing Slagwurm*'s ruling says "If your
Engulfing Slagwurm blocks or becomes blocked by multiple creatures, its ability triggers that many
times", once for each creature it blocks, and under the Reading the grammar builds, *Mirror
Shield*'s granted "Whenever a creature with deathtouch blocks or becomes blocked by this creature,
destroy that creature" would trigger when a creature with deathtouch blocks anything.

English builds the other Reading as a delayed right constituent coordination that pairs a simple
verb with a complex expression, as in "He had either telephoned or written a letter to his son's
boss" (CGEL, Ch. 15 §4.4, p. 1344, [24]): "a creature" is the Object of "blocks" and the
Complement of "by". The grammar already reads the participle form that way: the 14 lines that
print "blocking or blocked by …" (*Abu Ja'far*, *Cromat*, *Alms Beast*) have a Shared Complement
Reading. The 42 lines on 41 cards that print "blocks or becomes blocked by …" have none, counted
in `data/derived/english-forests.sqlite` by text and by the Productions at the coordination's
span.

The second conjunct reaches the shared phrase through *become*'s predicative Complement, the
adjective phrase "blocked by". Neither step carries a Gap: the `Predicative` Category declares
none, and the adjectival passive closes its *by* phrase. An extraction through the same
words in another frame is unaffected, since it reads through the verbal passive: "the creature
it's attached to" (*Aura Barbs*), "all creatures that the other is blocked by" (*General
Jarkeld*).

`judgments/become_adjectival_passive.txt` asserts the complete-conjunct Reading of one of these
lines, and `judgments/delayed_right_constituent_passive.txt` lists another among its "complete
controls". The complete-conjunct Reading is English and stays; the lines stop being controls for
complete conjuncts once they also have the Shared Complement Reading.

## Done when

- *Dream Fighter*'s line has a Reading in which "a creature" is the Shared Complement of "blocks"
  and of "by", and *Mirror Shield*'s granted ability one in which "this creature" is. Each is a
  judgment entry, written and adjudicated as `crates/english/judgments/README.md` says.
- Every line that prints "blocks or becomes blocked by" before a noun phrase has that Reading;
  the landing gives the count and the query.
- *Abu Ja'far*'s "blocking or blocked by it" keeps its Shared Complement Reading, and "the
  creature it's attached to" (*Aura Barbs*) keeps its Readings.
- The line in `delayed_right_constituent_passive.txt`'s "complete controls" is filed with the
  entries it now belongs with, and the entry in `become_adjectival_passive.txt` still holds.
- The landing lists every unit that gains a Reading. A gained Reading that is no English is a
  defect of the change, fixed in it.
- `cargo xtask english strata --check`, `cargo test -p sylex_english` and
  `cargo test -p sylex_syntax` pass.

## Not in scope

How a `.syl` term writes the shared phrase: `syl-shared-phrase-written-once`.

## Pointers

- `crates/english/grammar/english.fug` (`category Predicative`, `AdjectivalPredicative`, the
  `valence VerbPhrase` schema's `adjectival_passive`)
- `crates/syntax/src/compile.rs` (`adjectival_passive`, where the *by* phrase is closed)
- `crates/english/judgments/become_adjectival_passive.txt`,
  `crates/english/judgments/delayed_right_constituent_passive.txt`
- `docs/design/english.md` §3, §4, §8
