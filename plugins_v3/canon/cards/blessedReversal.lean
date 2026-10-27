/-- Blessed Reversal -/
card blessedReversal : Spelled :=
  { name := "Blessed Reversal", cost := [generic 1, pip .white], types := [.instant],
    text :=
      [ spell (gainLife
          (forEach 3 (and [isCreature, inCombat .attackerOf you]))) ] }
