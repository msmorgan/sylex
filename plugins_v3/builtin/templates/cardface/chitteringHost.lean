template chitteringHost : CardFace := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := { characteristics :=
    { name := "Chittering Host", types := [.creature],
      subtypes := [eldrazi, horror],
      text :=
        [ haste, menace,
          when (does thisCreature enter)
            (establishFor (allOf (and [isCreature, youControl, isOther]))
              [getsPt them (up 1) (up 0), abilityGrant them menace]
              untilEndOfTurn) ],
      power := stat 5, toughness := stat 6 } }
}
