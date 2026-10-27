template giftATappedFish : Ability := {
  spelling := "gift a tapped Fish",
  body :=
    gift (createTappedToken (creatureToken 1 1 [.blue] [fish]))
}
