/-- Escaped Shapeshifter -/
card := {
  name := "Escaped Shapeshifter",
  cost := [3, blue, blue],
  types := [creature],
  subtypes := [shapeshifter],
  text := [
    alsoForKeywords
      (fun k => static
         (onlyWhile
            (keywordGrant thisCreature k
               [isCreature, anOpponentControls, hasKeyword k, not (named "Escaped Shapeshifter")])
            (exists_
               [isCreature, anOpponentControls, hasKeyword k, not (named "Escaped Shapeshifter")])))
      [flying, firstStrike, trample, (anyIn ⟨protection, some color⟩)]
  ],
  power := 3,
  toughness := 4
}
