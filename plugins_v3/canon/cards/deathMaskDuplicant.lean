/-- Death-Mask Duplicant -/
card deathMaskDuplicant : Spelled :=
  { name := "Death-Mask Duplicant", cost := [generic 7], types := [.artifact, .creature],
    subtypes := [shapeshifter],
    text :=
      [ imprint
          (activated [generic 1] (exile (target (and [isCreature, in_ yourGraveyard])))),
        alsoForKeywords
          (fun k => static (onlyWhile
            (keywordGrant thisCreature k (and [in_ (exiledWith thisCreature), hasKeyword k]))
            (exists_ (and [in_ (exiledWith thisCreature), hasKeyword k]))))
          [ flying, fear, firstStrike, doubleStrike, haste, landwalkAbilities,
            protectionAbilities, trample ] ],
    power := stat 5, toughness := stat 5 }
