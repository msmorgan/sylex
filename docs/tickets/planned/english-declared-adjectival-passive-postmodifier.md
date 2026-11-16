---
needs: []
---

# A declared Adjectival Passive does not postmodify beside its participle

## Why

*Destroy target Aura attached to a creature.* has two Readings that say the same thing: *attached
to a creature* modifies *Aura* once as a participial Verb Phrase (`ParticipialPostmodifier`, the
passive participle of *attach*) and once as an Adjective Phrase (`PostpositiveAdjective`, the
declared adjective *attached* with its *to* Complement). The grammar already keeps a declared
Adjectival Passive and its verb's participle apart as a premodifier, as a bare single postmodifier
and as a derived adjectival passive (`AdjectivalPassive = Lexical` on the verb). This position is
the one it misses. *Remove Enchantments* has four *attached* and carries the fork at each.

## Design

- The participial Verb Phrase is the postmodifier (*each Aura attached to it*, as
  `judgments/trigger_clauses.txt` asserts). The declared adjective stays a predicative Complement
  (*becomes attached to this creature*) and a depictive Adjunct (*enters attached to a creature*).
- The loader marks the adjective as well as the verb when it pairs a declared Adjective with a
  verb's past participle, with the same `AdjectivalPassive = Lexical` feature, and
  `PostpositiveAdjective` excludes an Adjective Phrase whose head is so marked.

## Done when

- *Destroy target Aura attached to a creature.* has two Readings: *attached to a creature* as a
  Verb Phrase modifying *Aura*, and as an Adjective Phrase Adjunct of *destroy*. A judgment asserts
  both and asserts that no Reading has the Adjective Phrase as Modifier of *Aura*.
- In `cargo xtask english forestwalk --card 'Remove Enchantments' --line 1`, no Derivation has
  `adj:attached` as `modifier of Nominal`.
- The entries of `judgments/trigger_clauses.txt` for *becomes attached to this creature* and
  *enters attached to a creature* hold unchanged.
- The census before and after (`crates/english/README.md`, Working rules) loses no face and no
  unit's last Reading; every unit whose Readings change is reported. A unit that read only through
  the excluded structure reads another way in this ticket.
- The gate's ablation is reported as the README requires, and the gate's comment in `english.fug`
  names one real card and the duplicate it prevents.
- The Derivation count for *Remove Enchantments* in the comment of `judgments/large_forests.txt`
  is current.
- `crates/vocabulary/src/schema.rs`'s note on `AdjectiveEntry` and the comments at the feature's
  declaration say what the mark now does.

## Not in scope

- The verbal and adjectival passive under *be* (*the creature it's attached to*).
- The depictive Reading of *Destroy target Aura attached to a creature.*: it is a different
  structure, and ranking it is pragmatics.

## Pointers

- `crates/english/grammar/english.fug`: `feature AdjectivalPassive`, `PostpositiveAdjective`,
  `ParticipialPostmodifier`, `SingleParticipialPostmodifier`, `premodification Participle`.
- `crates/vocabulary/src/lib.rs`: `mark_lexical_adjectival_passives`.
- `crates/vocabulary/lexicon/english/adjectives.ron`: the *attached* entry.
