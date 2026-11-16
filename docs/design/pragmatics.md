# Pragmatics

## Status: DRAFT

This document is directional. It describes the step that chooses among the meanings a card's
text can have once every wrong reading has been refused, and the design is not settled: §5 lists
what is open, and the rest is not fixed either.

Recovery gives the candidate terms a card's text can be read as, and semantics refuses the ones
that have no meaning ([Architecture](architecture.md), "The layers"). What is left is sometimes
more than one meaning, each a faithful reading of the words. Pragmatics ranks those, and the top
one is proposed as the card's term.

## 1. What reaches pragmatics

*Broken Wings* prints "Destroy target artifact, enchantment, or creature with flying." and
*Troublemaker Ouphe* "exile target artifact or enchantment an opponent controls". In each, the
trailing modifier describes either the whole coordination or its last conjunct. The grammar
gives both Readings for both cards, both terms are well formed, and nothing in the rules refuses
either: an artifact can have flying, and "creature" needs no modifier. *Broken Wings* means the
last conjunct and *Troublemaker Ouphe* the whole coordination, and only what a card is likely to
say tells them apart.

Pragmatics ranks classes of meaning. Two candidate terms with one meaning are one class before
it runs, so it is never asked to choose between two ways of writing the same thing. What it
inspects to rank a class, the term or the meaning, is open (§5).

## 2. Principles

1. **Rank proposes and the stored term decides.** Pragmatics runs when a card's text is read into a
   term for the first time. Reading a stored term to play it never runs it; the check of a stored
   card may run it to report a rank, which gates nothing. A wrong choice is corrected in the card's
   `.syl` file, and the check on a stored term is that it is among the candidates of its text, that
   semantics accepts it and that it spells back to its text, whatever its rank. Why: a ranking that
   decided what a stored card means would be a second, unwritten source for that meaning.
2. **Pragmatics never overrules semantics.** It ranks only what semantics accepted, and it
   refuses nothing. A reading that cannot be true of the game is semantics' to refuse; a reading
   that is merely unlikely is ranked low here. Why: the two kinds of knowledge fail differently.
   A refusal is a rule with a citation, and a preference is a guess that some card will defy.
3. **Each kind of ambiguity has a default, and a preference is an exception to it.** Most cards
   are decided by the default, so the data stays small and each entry in it marks something
   unusual. Which default a trailing modifier over a coordination gets is not settled; §4 keeps
   every hypothesis.
4. **A preference is data on the declaration it concerns, and names no card.** Flying is an evasion
   ability [CR#702.9a] that limits which creatures can block one [CR#702.9b], so the `flying`
   declaration says it prefers a creature host. The override for one card is that card's stored
   term (item 1). Why: a preference keyed to a card is a card's meaning written outside its term.
5. **Every choice is explained and a tie is not broken.** The report for a card names the
   default or preference that ranked its classes. Two classes ranked equal are reported as a
   tie, and no term is stored until an author picks one.

## 3. The data

A declaration carries its preferences in a `pragmatics` field beside its `lexical`, `spelling`
and meaning (the form is a sketch):

    ability flying  { pragmatics = {preferHostTypes = [creature]}, … }
    subtype vehicle { pragmatics = {mimicTypes = [creature]}, … }

`preferHostTypes` says which hosts a description by this ability is likely to describe. What
`mimicTypes` does depends on the default (§4): under hypothesis B a type that mimics another
counts as that other's sort, and under the others a type that mimics another meets a preference
that names the other.

## 4. A trailing modifier over a coordination: the hypotheses

The lines the hypotheses are tried against, each with the reading it means:

| Line | Means |
|---|---|
| *Troublemaker Ouphe*: "target artifact or enchantment an opponent controls" | wide |
| "Creatures and Vehicles you control gain trample until end of turn." | wide |
| "Artifact creatures and Heroes you control get +1/+1 for each Artificer you control …" | wide |
| "you may tap two untapped creatures and/or Treasures you control" | wide |
| "Destroy all artifacts and creatures with mana value X or less." | wide |
| *Broken Wings*: "Destroy target artifact, enchantment, or creature with flying." | narrow |
| *Elven Riders*: "can't be blocked except by Walls and/or creatures with flying" | narrow |
| "an instant card or a card with flash" | narrow |
| *Exorcise*: "Exile target artifact, enchantment, or creature with power 4 or greater." | narrow |
| *Airlift Chaplain*: "a Plains card or a creature card with mana value 3 or less" | narrow |
| *Detection Tower*: "your opponents and creatures your opponents control with hexproof can be the targets of spells and abilities you control as though they didn't have hexproof" | wide |

Wide is the modifier over the whole coordination and narrow over its last arm. The lines come
from regular expressions over Oracle text that is not restricted to Vintage-legal cards, so they
are attested wordings and not counts.

- **A. Wide, unless a preference narrows it.** `flying` prefers a creature host, which narrows
  *Broken Wings*. It reads *Elven Riders* wide, wrongly: a Wall is a creature, so the preference
  is met by both arms and narrows nothing. It reads the flash, *Exorcise* and *Airlift Chaplain*
  lines wide, wrongly, unless `flash`, power and mana value each carry a preference.
- **B. Wide where the arms' sorts match, narrow where they differ, unless a preference says
  otherwise.** Card types match (*Troublemaker Ouphe*; *Broken Wings*, which `flying` then
  narrows), and a subtype beside a card type does not, which reads *Elven Riders* narrow.
  `mimicTypes` makes Vehicle count as a card type for "Creatures and Vehicles you control". It
  reads the Heroes and Treasures lines narrow, wrongly, unless each such subtype carries the
  same data, "an instant card or a card with flash" and *Exorcise* wide, wrongly, since their
  arms' sorts match, and *Detection Tower* narrow, wrongly, since a player and a creature differ.
- **C. Wide, unless a preference narrows it or the wide reading makes one arm a subset of
  another.** Every Wall with flying is a creature with flying, so the wide reading of *Elven
  Riders* says its first arm twice and the narrow one is taken. It needs which card type a
  subtype belongs to, which the Catalogs hold. It reads *Airlift Chaplain* wide, wrongly, since
  neither arm is a subset of the other, and *Exorcise* narrow only if it knows that a
  noncreature permanent has no power.
- **D. By kind of modifier: "you control" and "an opponent controls" are wide, and "with" and a
  keyword ability is narrow.** It reads *Detection Tower* narrow, wrongly, and the control and
  keyword lines as they mean, with no data on any declaration. It says nothing of the mana
  value and power lines: each other kind of modifier ("with mana value
  …", "with power …", a relative clause, "named …") needs its own row.

No hypothesis reads every line above as it means. In the *Airlift Chaplain* line the two
readings pick out the same cards, since a land card's mana value is 0, so whether it reaches
pragmatics at all depends on when two meanings are the same ([Semantics](semantics.md) §8).

## 5. Open

- **The kinds of ambiguity.** One is described here. The voice design names another ("is
  tapped" as a passive or as a copula with an adjective), which the clause's position may
  settle before pragmatics is reached. The full list, and the default for each kind, comes from
  the first run of recovery and semantics over the corpus.
- **What a preference may say.** Two fields are sketched. Whether preferences are a closed set
  of fields or a small language, and whether two that disagree are ordered or reported as a
  tie, is not designed.
- **What pragmatics reads.** A modifier's host is a fact of the surface term, and a type's
  relation to another is a fact about the game. Whether the step reads the term, the meaning or
  both is not settled.
- **Where the data is checked.** A preference that names a type or declaration that does not
  exist has to be refused at load; which tool does it is not chosen.
