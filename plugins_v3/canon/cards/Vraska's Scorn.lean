/-- Vraska's Scorn -/
card := {
  name := "Vraska's Scorn",
  cost := [2, black, black],
  types := [sorcery],
  text := [
    spell
      [
        instruct (target isOpponent) (loseLife 4),
        may
          [
            searchLibraryOrGraveyard (a (named "Vraska, Scheming Gorgon")),
            reveal it,
            putInto it hand
          ],
        doIf (did you (search yourLibrary) thisWay) shuffle
      ]
  ]
}
