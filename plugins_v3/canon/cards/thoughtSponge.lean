/-- Thought Sponge -/
card thoughtSponge : Spelled :=
  { name := "Thought Sponge", cost := [generic 3, pip .blue], types := [.creature],
    subtypes := [sponge],
    text :=
      [ flash,
        static (entersWithCounters thisCreature greatestCardsAnOpponentDrew p1p1Counter),
        when (does thisCreature die) (draw (powerOf thisCreature)) ],
    power := stat 1, toughness := stat 1 }
