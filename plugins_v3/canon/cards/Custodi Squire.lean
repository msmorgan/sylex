card := {
  name := "Custodi Squire",
  cost := [4, white],
  types := [creature],
  subtypes := [spirit, cleric],
  text := [
    flying,
    willOfTheCouncil
      (when (does thisCreature enter)
         [
           instruct eachPlayer
             (voteStartingWith you openly
                (byCandidate (a [or [isArtifact, isCreature, isEnchantment], in_ yourGraveyard]))),
           returnToHand (allOf [isCard, withMostVotes])
         ])
  ],
  power := 3,
  toughness := 3
}
