/-- Plea for Power -/
card pleaForPower : Spelled :=
  { name := "Plea for Power", cost := [generic 3, pip .blue], types := [.sorcery],
    text :=
      [ willOfTheCouncil
          (spell (sequentially
            [ instruct eachPlayer (voteStartingWith you .openly (byLabel ["time", "knowledge"])),
              doIf (voteLead "time" false) (extraTurn 1),
              doIf (voteLead "knowledge" true) (draw 3) ])) ] }
