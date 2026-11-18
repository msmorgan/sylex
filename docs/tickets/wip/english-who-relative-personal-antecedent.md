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
