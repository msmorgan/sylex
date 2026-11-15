# Pragmatics

## Status: DRAFT

This document is directional. It describes the step that chooses among the meanings a card's
text can have once every wrong reading has been refused, and the design is not settled: §4 lists
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

Pragmatics sees meanings, not candidates. Two candidate terms with one meaning are one class
before it runs, so it is never asked to choose between two ways of writing the same thing.

## 2. Principles

1. **Rank proposes and the stored term decides.** Pragmatics runs when a card's text is read
   into a term for the first time. Reading a stored term never runs it. A wrong choice is
   corrected in the card's `.syl` file, and the check on a stored term is that it is among the
   candidates of its text and that semantics accepts it, whatever its rank. Why: a ranking that
   decided what a stored card means would be a second, unwritten source for that meaning.
2. **Pragmatics never overrules semantics.** It ranks only what semantics accepted, and it
   refuses nothing. A reading that cannot be true of the game is semantics' to refuse; a reading
   that is merely unlikely is ranked low here. Why: the two kinds of knowledge fail differently.
   A refusal is a rule with a citation, and a preference is a guess that some card will defy.
3. **Each kind of ambiguity has a default, and a preference is an exception to it.** For a
   trailing modifier over a coordination the default is the whole coordination, as on
   *Troublemaker Ouphe* and in "Creatures and Vehicles you control gain trample until end of
   turn"; a preference narrows it, as `flying` does on *Broken Wings*. Why: most cards are
   decided by the default, so the data stays small and each entry in it marks something
   unusual.
4. **A preference is data on the declaration it concerns, and names no card.** Flying is an
   evasion ability of creatures [CR#702.9a,702.9b], so the `flying` declaration says it prefers
   a creature host. The override for one card is that card's stored term (item 1). Why: a
   preference keyed to a card is a card's meaning written outside its term.
5. **Every choice is explained and a tie is not broken.** The report for a card names the
   default or preference that ranked its classes. Two classes ranked equal are reported as a
   tie, and no term is stored until an author picks one.

## 3. The data

A declaration carries its preferences in a `pragmatics` field beside its `lexical`, `spelling`
and meaning (the form is a sketch):

    keyword_ability flying = { pragmatics = { preferHostTypes = [creature] }, … }
    subtype vehicle        = { pragmatics = { mimicTypes = [creature] }, … }

`preferHostTypes` says which hosts a description by this ability is likely to describe.
`mimicTypes` says a type stands in for another where a preference names that other: a Vehicle
is described the way a creature is ("target creature or Vehicle you control gains double
strike"), since most Vehicles become creatures [CR#301.7], so a preference for creature hosts
is met by a Vehicle too.

## 4. Open

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
