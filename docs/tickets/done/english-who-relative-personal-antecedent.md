---
needs: []
---

# A *who* relative modifies a non-personal antecedent

## Why

*Disorder*'s "Disorder deals 2 damage to each white creature and each player who controls a white
creature." has a Reading in which "who controls a white creature" is a Modifier postposed out of
the Object "2 damage". Relative *who* takes a personal antecedent and *which* a non-personal one
(CGEL, Ch. 12 §3.5.1, p. 1048), and damage is not personal, so no English grammar admits that
Reading, and an ungrammatical Reading is a defect (`english.md` §8).

Reproduce with `cargo xtask english forestwalk --card Disorder --line 1`: the option
`dependency "who"#1 modifier of VerbPhrase "deals"#1`.

## Design

- A grammar feature `RelativeGender = Personal | NonPersonal`, the gender CGEL's relative
  pronouns contrast in (Ch. 5 §17.3, pp. 497–498): relative *who* is `Personal` and *which*
  `NonPersonal`, declared on the pronouns in the lexicon; *whose*, the genitive of both, carries
  it into no clause. It is a lexical feature, not a Lexical Class: it is agreement between two
  words, not eligibility for a slot.
- A noun declares its `relative_gender`: `NonPersonal` by default (inanimates always take
  *which*), `Personal` for a player (*player*, *opponent*, *owner*, *controller*, *monarch*,
  *protector*, …), `Dual` for a being, which takes either as an animal does (*creature*,
  *planeswalker*, *attacker*, *team*, …) and carries no value. A subtype's family decides it:
  creature and planeswalker types are `Dual`, every other subtype `NonPersonal`. A symbol phrase is
  `NonPersonal`; a head with no noun (a fused determinative, a name) carries none.
- The value rises through the head line to Nominal and Noun Phrase. An *and*-coordination takes
  its last conjunct's (`proximate`), CGEL's proximity principle for coordinated antecedents
  (Ch. 12 §3.5.1, p. 1049, [46]), so *each white creature and each player who* stands.
- One `dependents RelativeAntecedent` declaration states the agreement between a relative clause
  (as Postmodifier, InterleavedModifier or a fused head's RelativeClause) and the nominal it
  modifies; a postposed relative agrees with its Object through the engine's agreement of every
  postposed Modifier with its Object, once `ComplementNounPhrase` carries the feature.

## Done when

- No Reading has a *who* relative whose antecedent is non-personal, on *Disorder* and wherever
  else the census shows one, in a postposed Modifier or beside its nominal.
- A judgment entry on *Disorder* witnesses it, and its entry in
  `crates/english/judgments/clause_over_coordinated_noun_phrases.txt` asserts its Reading count.
- The census before and after shows no face lost, and every unit whose verdict changes is
  checked.

## Pointers

- `crates/english/grammar/english.fug`: `PostposedModifier`, `RelativePostmodifier` and the
  relative clause Productions.
- CGEL, Ch. 12 §3.5.1, pp. 1048–1049; Ch. 5 §17.3 on gender.
