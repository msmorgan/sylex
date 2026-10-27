/-- Hijack -/
card hijack : Spelled :=
  { name := "Hijack", cost := [generic 1, pip .red, pip .red], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ gainControl (target (or [isArtifact, isCreature])) untilEndOfTurn,
            untap it,
            gainHaste it untilEndOfTurn ]) ] }
