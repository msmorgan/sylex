/-- Faith's Reward -/
card := {
  name := "Faith's Reward",
  cost := [3, white],
  types := [instant],
  text := [
    spell
      (returnToBattlefield
         (allOf
            [
              isPermanentCard,
              in_ yourGraveyard,
              was (putInto which yourGraveyard (from_ := battlefield)) (within soFarThisTurn)
            ]))
  ]
}
