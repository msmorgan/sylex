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
`feature VerbKind = Auxiliary` is privative (`Auxiliary` unifies with unvalued lexical verbs). Zero
corpus units intend that finite coordination: under modal or negated scope, `can't/may V1 or V2`
coordinates `BareInf`s under the modal (CGEL, Ch. 15, §2.1(b), p. 1290).

By contrast, `Present + Preterite` finite VP coordination is needed on real cards (`Inflection`
stays in `unshared:` for *Break the Ice*'s `"is snow or could produce {C}"`, *Cache Grab*'s `"control
a Squirrel or returned a Squirrel card …"`, and *Illusionary Mask*'s `"has not been turned face up
and would assign …"`).

## Design

- Split finite active `coordination VerbPhrase` between `And`/`But` and `Disjunctive` (`or`,
  `and/or`, `nor`), keeping `Inflection` in `unshared:` and excluding modal catenative +
  non-auxiliary lexical finite VP coordination under `Disjunctive` while preserving `and`/`but`
  (*Ace's Baseball Bat*, *Illusionary Mask*), `be + modal` (*Break the Ice*), `lexical + copular be`
  (*Hooded Horror*), and `lexical + lexical` (*Cache Grab*).

## Done when

- Disjunctive finite VP coordination refuses modal catenative + lexical finite VPs by a declared
  grammar constraint, eliminating the spurious finite derivations across the 108 units while
  preserving *Ace's Baseball Bat*, *Break the Ice*, *Cache Grab*, *Hooded Horror* and *Illusionary
  Mask*.
- A judgment test pins *Rhonas can't attack or block* to bare-infinitive coordination, the census
  shows no verdict loss on the supported corpus, and `cargo xtask english strata --check`,
  `cargo test -p sylex_english` and `cargo clippy -p sylex_english --tests -- -D warnings` pass.

## Pointers

- `crates/english/grammar/english.fug` (`feature VerbKind`, `coordination VerbPhrase`).
- `docs/design/english.md`, `docs/design/lexicon.md` §2.
