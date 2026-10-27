/-- Chrome Mox -/
card chromeMoxMana : Spelled :=
  { name := "Chrome Mox", cost := some [], types := [.artifact],
    text :=
      [ imprint
          (when (does thisArtifact enter)
            (may (exile (a (and [non isArtifact, non isLand, cardIn yourHand]))))),
        activated tapSymbol
          (addMana 1 (ofAnyColorOf (the (and [was (exile which), isCard])))) ] }
