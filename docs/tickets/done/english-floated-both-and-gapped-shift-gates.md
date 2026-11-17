---
needs: []
---

# Gate floated *both* after a singular Subject and an Object shifted past a gapped Preposition Phrase

## Why

*Remove Enchantments* line 1 has 11,040 Derivations (276 × 40 over its two sentences), and two
structures that no card needs account for about two thirds of them.

"all enchantments you both own and control" reads *both* as a floating quantifier on *you* beside
the correlative *both own and control*. `FloatedEach` and its twins admit a Subject that is not
third person singular so that *You each lose life* (Keen Duelist) parses, and *both* is in the
same `FloatingQuantifier` class. Every one of the ten cards that print "you both" prints "you both
own and control"; the pool floats *both* only after a plural Subject (*this creature and other
target creature you control both gain …*).

"all Auras you own attached to permanents you control" reads *attached* as a preterite heading a
bare relative on "enchantments", with the Gap under *to* and "permanents you control" a shifted
Object after it. `valence_order` in `crates/syntax/src/compile.rs` postposes an Object past any
selected Preposition Phrase, a gapped one included. Across the pool a shifted Object inside a
relative clause is in 28 units and is the only analysis in none (*attach* 8, *deal* 10, *put* 7);
on *Portal of Sanctuary* the shifted Object is the pronoun "it" (*each Aura attached to* — *it*).
The same words always have the analysis with the Object's own Gap (*attached* — *to it*).

Reproduce with `cargo xtask english forestwalk --card 'Remove Enchantments' --line 1` and read
the options of `dependency "both"#1` and `dependency "permanents"#1`.

## Design

- A floated *both* takes a Plural Subject; floated *each* keeps the Subjects it has. The gate is
  on the Productions that float a `FloatingQuantifier`, not on the lexeme.
- The shifted-Object order is not derived for a draft whose crossed Preposition Phrase introduces
  the Gap. A shifted Object past a complete Preposition Phrase is unchanged (*you may attach to it
  any number of Auras on the battlefield*, Bruna, Light of Alabaster).
- Each gate's comment names its card and the structure it prevents, and its ablation is reported
  (`english.md` §6).

## Done when

- *Remove Enchantments* line 1 has no Derivation in which *both* is an Adjunct of a Clause, and
  none in which "permanents" or "creatures" is the Object of *attached* after a Gap under *to*.
- *Portal of Sanctuary* line 1 has no Derivation in which "it" is the Object of *attached*.
- Judgment entries witness the floated *both* after a plural Subject and the shifted Object past a
  complete Preposition Phrase on real card text, and refuse the two gated structures.
- The census before and after shows no face lost and no unit with more Readings; every unit whose
  verdict changes is checked, and the units each gate changes are counted.
- `cargo xtask english strata --check`, `cargo test -p sylex_english`, `cargo test -p
  sylex_syntax` and clippy on both pass.

## Not in scope

- The object-gap twin (*enchantments all Auras attached* — *to permanents*) and *to* with a
  gerund-participial Complement (*attached to [attacking creatures]*): both are grammatical, and
  meaning rejects them (`english.md` §4).

## Pointers

- `crates/english/grammar/english.fug`: `FloatedEach`, `RelativeFloatedEach`,
  `AuxiliaryFloatedEach`, `FloatedEachSubjectGap`, the `FloatingQuantifier` class.
- `crates/syntax/src/compile.rs`: `valence_order`, the `shifted-object` draft and `introduces`.
- `crates/english/README.md`, "Working rules"; `crates/english/judgments/README.md`.
- `docs/design/english.md` §4, §6.
