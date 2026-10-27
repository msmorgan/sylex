/-- Faith's Reward -/
card faithsReward : Spelled :=
  { name := "Faith's Reward", cost := [generic 3, pip .white], types := [.instant],
    text :=
      [ spell (returnToBattlefield (allOf (and [ isPermanentCard, in_ yourGraveyard,
                         was (putInto which yourGraveyard (from_ := battlefield))
                           (within soFarThisTurn) ]))) ] }
