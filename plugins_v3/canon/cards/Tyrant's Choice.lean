/-- Tyrant's Choice -/
card := {
  name := "Tyrant's Choice",
  cost := [1, black],
  types := [sorcery],
  text := [
    willOfTheCouncil
      (spell
         [
           instruct eachPlayer (voteStartingWith you openly (byLabel ["death", "torture"])),
           doIf (voteLead "death" false)
             (instruct eachOpponent (sacrifice (aTheirChoice isCreature))),
           doIf (voteLead "torture" true) (instruct eachOpponent (loseLife 4))
         ])
  ]
}
