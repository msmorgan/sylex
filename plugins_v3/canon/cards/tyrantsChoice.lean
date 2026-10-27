/-- Tyrant's Choice -/
card tyrantsChoice : Spelled :=
  { name := "Tyrant's Choice", cost := [generic 1, pip .black], types := [.sorcery],
    text :=
      [ willOfTheCouncil
          (spell (sequentially
            [ instruct eachPlayer (voteStartingWith you .openly (byLabel ["death", "torture"])),
              doIf (voteLead "death" false)
                (instruct eachOpponent (sacrifice (aTheirChoice isCreature))),
              doIf (voteLead "torture" true) (instruct eachOpponent (loseLife 4)) ]))
                  ] }
