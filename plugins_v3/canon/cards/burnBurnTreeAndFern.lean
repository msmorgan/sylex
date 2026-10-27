/-- Burn, Burn, Tree and Fern -/
card burnBurnTreeAndFern : Spelled :=
  { name := "Burn, Burn, Tree and Fern", cost := [generic 3, pip .red],
    types := [.enchantment], subtypes := [saga],
    text :=
      [ when (chapterMark [1])
          (instruct this_
             (dealDamage 6 (target (and [isCreature, anOpponentControls])))),
        when (chapterMark [2])
          (destroy (target (and [isArtifact, anOpponentControls]))),
        when (chapterMark [3, 4]) (addMana 1 (manaOf [[.of .red]])) ] }
