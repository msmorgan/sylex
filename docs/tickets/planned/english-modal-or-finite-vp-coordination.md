---
needs: []
---
# Disjunctive finite VP coordination excludes modal with lexical verb

## Why

Disjunctive `coordination VerbPhrase` (`or`, `and/or`, `nor`) currently admits a modal-headed
finite VP coordinated with a plain lexical finite VP across 108 corpus units: singular proper-name
subjects with unvalued `Number` (*Rhonas can't attack or block*), plural subjects (*Creatures you
control can't attack or block*, *Players can't pay life or sacrifice creatures*), second-person
`you` subjects (*you may tap or untap target …*), and 3rd-singular subjects before invariant
preterites (*Damping Engine*'s `"[can't play lands] or [cast/Preterite … spells]"`), because
`feature VerbKind = Auxiliary` is privative (`Auxiliary` unifies with unvalued lexical verbs) and
`coordination VerbPhrase` puts `Inflection` in `unshared:`. Zero corpus units intend that finite
coordination: under modal/negated scope, `can't/may V1 or V2` coordinates `BareInf`s under the
modal (CGEL, Ch. 15, §2.1(b), p. 1290).

Meanwhile, *Lesser Werewolf* (`"{B}: If this creature's power is 1 or more, it gets -1/-0 until end
of turn and put a -0/-1 counter on target creature blocking or blocked by this creature."`) only
parses today through that `Present + Preterite` `put` VP coordination because `coordination Clause`
has `Plain + and + Finite` and `Finite + , but + Plain` without `Finite + and + Plain`.

## Design

- Split finite active `coordination VerbPhrase` between `And`/`But` and `Disjunctive` (`or`,
  `and/or`, `nor`), excluding modal catenative + lexical finite VP coordination under `Disjunctive`
  while preserving `and`/`but` (*Ace's Baseball Bat*), `be + modal` (*Break the Ice*) and
  `lexical + copular be` (*Hooded Horror*).
- Forbid mixing `Present` and `Preterite` finite VPs in `coordination VerbPhrase`, and add
  `Finite + and + Plain` to `coordination Clause` so *Lesser Werewolf* parses as coordinated clauses
  with an imperative second conjunct.

## Done when

- Disjunctive finite VP coordination refuses modal + lexical finite VPs and finite `Present +
  Preterite` mixing by declared grammar constraints, eliminating the spurious finite derivations
  across the 108 units while preserving *Ace's Baseball Bat*, *Break the Ice*, *Hooded Horror* and
  *Lesser Werewolf*.
- Judgment tests pin *Rhonas can't attack or block* to bare-infinitive coordination and *Lesser
  Werewolf* to clause coordination, and `cargo xtask english strata --check`, `cargo test -p
  sylex_english` and `cargo clippy -p sylex_english --tests -- -D warnings` pass.

## Pointers

- `crates/english/grammar/english.fug` (`coordination VerbPhrase`)
- `docs/design/english.md`, `docs/design/lexicon.md` §2 (finite forms unmarked for Number)
