/-- Echoing Ruin -/
card echoingRuin : Spelled :=
  { name := "Echoing Ruin", cost := [generic 1, pip .red], types := [.sorcery],
    text :=
      [ spell (destroy
          (both (target isArtifact)
            (allOf (and [ isArtifact, (shares .cardName .share (that artifact)),
                           isOther ])))) ] }
