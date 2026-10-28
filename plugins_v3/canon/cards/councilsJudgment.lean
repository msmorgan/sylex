/-- Council's Judgment -/
card := {
  name := "Council's Judgment",
  cost := [1, white, white],
  types := [sorcery],
  text := [
    willOfTheCouncil
      (spell
         [
           instruct eachPlayer
             (voteStartingWith you openly
                (byCandidate (a [isPermanent, non isLand, youDontControl]))),
           exile (allOf [isPermanent, withMostVotes])
         ])
  ]
}
