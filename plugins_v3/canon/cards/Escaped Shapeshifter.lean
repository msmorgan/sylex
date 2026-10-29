card := {
  name := "Escaped Shapeshifter",
  cost := [3, blue, blue],
  types := [creature],
  subtypes := [shapeshifter],
  text := [
    alsoForKeywords
      (static
         (onlyWhile
            (keywordGrant thisCreature thatKeyword
               [isCreature, anOpponentControls, hasKeyword thatKeyword,
                 not (named "Escaped Shapeshifter")])
            (exists_
               [isCreature, anOpponentControls, hasKeyword thatKeyword,
                 not (named "Escaped Shapeshifter")])))
      [flying, firstStrike, trample, (anyIn { word := protection, sort := some color })]
  ],
  power := 3,
  toughness := 4
}
