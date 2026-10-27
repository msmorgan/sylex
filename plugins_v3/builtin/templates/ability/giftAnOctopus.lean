template giftAnOctopus : Ability := {
  spelling := "gift an Octopus",
  body :=
    gift (create 1 (creatureToken 8 8 [.blue] [octopus]))
}
