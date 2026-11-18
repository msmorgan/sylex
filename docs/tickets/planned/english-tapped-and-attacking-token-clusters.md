---
needs: []
---

# *A tapped and attacking … token* is no coordination of determined Clusters

## Why

On six cards that create *a tapped and attacking* token (Rite of Ascension, Ghalta and Mavren,
Maestros Diabolist, Pugnacious Pugilist, U.S.S. Stargazer, Here and There, Worf, Still Not
Merry),
`SharedNominalDetermined` builds a second Noun Phrase beside `Determined`'s: Clusters *[a
tapped]* and *[attacking 2/1 red]*, with *attacking* the combat-role determinative and the Noun
Phrase plural. Rite of Ascension's line 1 has 4 of its 6 Derivations on that tree
(`cargo xtask english forestwalk --card 'Rite of Ascension' --line 1`). It is an ungrammatical
Reading: *tapped* and *attacking* are coordinated premodifiers under the one determiner *a*,
and the Noun Phrase is singular. The only card text
the Production serves is Balthor the Defiled's *all black and all red creature cards*; its
comment justifies the coordination's Number resolution only by CGEL's *a black and a red
creature* and *each black and each red creature* (Ch. 5 §18.4, p. 508), which no card prints.

## Done when

- None of the six units has a `SharedNominalDetermined` Reading, by a general rule (no exclusion
  aimed at *attacking*), and Balthor the Defiled keeps its Reading.
- The `SharedNominalDetermined` comment names Balthor the Defiled for what the Production does,
  and the `coordination NominalClusters` variants each serve card text or are removed.
- The census shows no face lost and no verdict change; units whose Readings change are reported.

## Pointers

- `crates/english/grammar/english.fug`: `SharedNominalDetermined`, `DeterminedCluster`,
  `coordination NominalClusters`.
- `crates/vocabulary/lexicon/english/closed_class.ron` (*attacking*, *defending*).
