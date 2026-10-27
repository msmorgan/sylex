/-- Commune with the Gods -/
card communeWithTheGods : Spelled :=
  { name := "Commune with the Gods", cost := [generic 1, pip .green], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ reveal (topSlice 5),
            may (putInto (fromAmong (exactly 1) (or [isCreature, isEnchantment]) them) hand),
            putInto theRest graveyard ]) ] }
