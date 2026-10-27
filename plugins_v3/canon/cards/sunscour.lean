card sunscour : Spelled :=
  { name := "Sunscour", cost := [generic 5, pip .white, pip .white], types := [.sorcery],
    text :=
      [ static (altCost this_
          (exile
            (counted (exactly 2) (and [hasColor .white, in_ yourHand])))),
        spell (destroy (allOf isCreature)) ] }
