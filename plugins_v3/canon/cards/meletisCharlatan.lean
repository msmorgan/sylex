/-- Meletis Charlatan -/
card meletisCharlatan : Spelled :=
  { name := "Meletis Charlatan", cost := [generic 2, pip .blue], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ activated (compound [[generic 2, pip .blue], tapSymbol])
          (sequentially
            [ instruct (controllerOf (target (and [isInstantOrSorcery, isSpell]))) (copyOnStack it),
              instruct (that player) (may (chooseNewTargets (that .copy))) ]) ],
    power := stat 2, toughness := stat 3 }
