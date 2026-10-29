card := {
  name := "Death-Mask Duplicant",
  cost := [7],
  types := [artifact, creature],
  subtypes := [shapeshifter],
  text := [
    imprint (activated [1] (exile (target [isCreature, in_ yourGraveyard]))),
    alsoForKeywords
      (fun k => static
         (onlyWhile (keywordGrant thisCreature k [in_ (exiledWith thisCreature), hasKeyword k])
            (exists_ [in_ (exiledWith thisCreature), hasKeyword k])))
      [
        flying,
        fear,
        firstStrike,
        doubleStrike,
        haste,
        (anyIn { word := landwalk }),
        (anyIn { word := protection }),
        trample
      ]
  ],
  power := 5,
  toughness := 5
}
