card abolish : Spelled :=
  { name := "Abolish", cost := [generic 1, pip .white, pip .white], types := [.instant],
    text :=
      [ static (altCost this_
          (discard (a (and [plains, in_ yourHand])))),
        spell (destroy (target (or [isArtifact, isEnchantment]))) ] }
