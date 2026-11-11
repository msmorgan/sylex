---
needs: []
---
# Number transparency is built for *any number of* only

## Why

`TransparentQuantity` gives a Noun Phrase the Number of a quantificational noun's oblique
(*any number of target creatures become ...*), and requires
`determiner.DeterminativeDistribution = FreeChoice`, which only *any* holds. Number transparency
is a property of the noun *number* with a plural oblique whatever determines it (CGEL Ch. 5
§3.3, pp. 349–351), so *a number of creatures* and *that number of cards* as Subjects appear to
take only the singular of *number*. For frequency Noun Phrases the grammar has both cases:
`TemporalQuantity` for *any* and `DeterminedTemporalQuantity` for other determiners.

## Open questions

- Extend transparency to other determiners of *number* as `DeterminedTemporalQuantity` does for
  *times*, or leave it to *any* and say in the comment that no card needs more?

## Done when

- Whether any card in the supported corpus has *number of* with a plural oblique under a
  determiner other than *any*, in a position where its Number matters (a Subject, or an
  antecedent a plural pronoun or relative agrees with), is recorded. If none does, the ticket
  closes with `TransparentQuantity`'s comment saying it is limited to *any* for that reason.
- If one does, it has a Reading with plural agreement and one tree.
- The census shows no Reading lost on the supported corpus.

## Not in scope

- The `quantity` child's name: `english-renamed-children-head-agreement`.

## Pointers

- `crates/english/grammar/english.fug`: `TransparentQuantity`, `FreeChoiceQuantity`,
  `FreeChoiceDetermined`, `TemporalQuantity`, `DeterminedTemporalQuantity`,
  `feature NounDistribution` (`Quantity`).
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/gemini.md` (E1).
