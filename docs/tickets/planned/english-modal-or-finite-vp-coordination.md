---
needs: [plural-name-flag]
---
# Is [modal finite VP] *or* [plain finite VP] grammatical?

## Why

With card names number-neutral, *Rhonas can't attack or block* gains Readings that coordinate
finite *can't attack* with plural present *block* (*[they] [can't attack] or [block]*), the name
taking Plural from *block*. Twelve units gain 71 derivations this way: the "[God] can't attack or
block unless …" units (Sab-Sunen, Hazoret the Fervent, Rhonas, Oketra, The Ancient One, Hazoret,
Godseeker, Bontu, Kefnet, Ketramose, Bast, The Lion-Turtle) and Cloud, Planet's Champion's "… that
target Cloud cost {2} less". None ranks cheapest.

These Readings feel ungrammatical: no sentence of the form *they can't X or Y*, with Y a plain
finite VP and no auxiliary, reads as *[they can't X] or [they Y]*. With a 3rd-singular subject, so
that the second verb's form is unambiguous:

- *She can't sing but dances beautifully.* and *He can cook and does the dishes every night.*
  sound acceptable.
- *She can't sing or dances.* sounds bad; *She can sing or dances.* sounds odd.
- *She either sings or dances.* is fine.

The oddness seems to lie with *or* joining a modal-headed finite VP to a plain finite one, not with
finite VP coordination in general. CGEL's finite-VP coordinations (Ch. 15 §2.1, p. 1291, [4i]; §2.9,
p. 1319, [76i]; §2.10, [80]) use *and*, *let alone* and *so*, and none has a negated modal first
coordinate. If the pairing is ungrammatical, the grammar's VP coordination admits a second tree that
English does not, independent of name Number.

## Open questions

- Is [modal-headed finite VP] *or* [plain finite VP] grammatical, and under what conditions (the
  coordinator, negation in the first coordinate, *either*)? Answer from CGEL (Ch. 15 §2.1(b)
  "Coordinates must be syntactically alike", p. 1290; the scope of *or* under negation) and the
  corpus (any attested Oracle *or* joining such coordinates), with a recommendation.

## Done when

- The question is answered with evidence and the owner has ruled.
- If ruled ungrammatical, VP coordination refuses the pairing by a declared constraint (no unit or
  word keyed), the twelve units lose those Readings, a negative judgment pins *Rhonas can't attack
  or block* to the bare-infinitive coordination, and `cargo xtask english strata --check` passes.

## Pointers

- `crates/english/grammar/english.fug` (`coordination VerbPhrase`)
- `docs/decisions/english.md`, `docs/decisions/lexicon.md` §2 (finite forms unmarked for Number)
