/-- Escaped Shapeshifter -/
card escapedShapeshifter : Spelled :=
  { name := "Escaped Shapeshifter", cost := [generic 3, pip .blue, pip .blue],
    types := [.creature], subtypes := [shapeshifter],
    text :=
      [ alsoForKeywords
          (fun k => static (onlyWhile
            (keywordGrant thisCreature k
              (and [ isCreature, anOpponentControls, hasKeyword k,
                     not (named "Escaped Shapeshifter") ]))
            (exists_
              (and [ isCreature, anOpponentControls, hasKeyword k,
                     not (named "Escaped Shapeshifter") ]))))
          [flying, firstStrike, trample, protectionFromAnyColor] ],
    power := stat 3, toughness := stat 4 }
