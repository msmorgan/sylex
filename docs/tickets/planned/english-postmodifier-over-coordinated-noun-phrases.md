---
needs: []
---

# Read a postmodifier over a coordination of determined noun phrases

## Why

*Finale of Promise* prints "You may cast up to one target instant card and/or up to one target
sorcery card from your graveyard each with mana value X or less without paying their mana costs."
Both cards are cast from the graveyard. In every Reading "from your graveyard" modifies "sorcery
card" alone, inside the second conjunct, and the instant card is in no zone.

A coordination of bare nominals has the wide Reading, because the phrase attaches to the Nominal
above the conjuncts: "Artifact cards and red creature cards in your hand" (*Tannuk, Steadfast
Second*). Conjuncts that each have their own determiner have no Nominal above them, and a
postmodifier's host is a Nominal. Where the phrase can instead be the verb's adjunct, the wide
meaning has a Reading already: *Retrieve*'s "Return up to one target creature card and up to one
target noncreature permanent card from your graveyard to your hand". *Finale of Promise* has no
such Reading, because the floated "each with mana value X or less" that follows belongs to the
noun phrase and closes it after "from your graveyard".

The grammar already attaches one modifier to a coordination of noun phrases: the floated *each*
phrase of this same line (`FloatedEachCoordinationPostmodifier`).

One line is known. A regular expression over the corpus for two "up to N target …" noun phrases
joined by "and" or "and/or", then a zone phrase, then more of the noun phrase, finds only this
one.

## Done when

- *Finale of Promise*'s first sentence has a Reading in which "from your graveyard" modifies the
  whole coordination and stands inside the floated *each* phrase's noun phrase. It is a judgment
  entry beside the line's entry in `judgments/floated_each_quantifier.txt`, written and
  adjudicated as `crates/english/judgments/README.md` says.
- A coordination of bare nominals keeps exactly the Readings it has: *Tannuk, Steadfast Second*'s
  line gains none, since its wide Reading is the Nominal's (`docs/design/english.md` §8, one tree
  per analysis).
- The landing lists every unit that gains a Reading. A gained Reading that is no English is a
  defect of the change, fixed in it.
- `cargo xtask english strata --check`, `cargo test -p sylex_english` and
  `cargo test -p sylex_syntax` pass.

## Pointers

- `crates/english/grammar/english.fug` (`FloatedEachCoordinationPostmodifier`, the noun-phrase
  coordinations, `Hosted(PrepositionPhrase, Nominal)`)
- `crates/english/judgments/floated_each_quantifier.txt`
- `docs/design/english.md` §3, §4, §8
