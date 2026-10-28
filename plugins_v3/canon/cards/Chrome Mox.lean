card := {
  name := "Chrome Mox",
  cost := some [],
  types := [artifact],
  text := [
    imprint
      (when (does thisArtifact enter)
         (may (exile (a [non isArtifact, non isLand, cardIn yourHand])))),
    activated tapSymbol (addMana 1 (ofAnyColorOf (the [was (exile which), isCard])))
  ]
}
