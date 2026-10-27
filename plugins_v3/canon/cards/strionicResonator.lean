/-- Strionic Resonator -/
card strionicResonator : Spelled :=
  { name := "Strionic Resonator", cost := [generic 2], types := [.artifact],
    text :=
      [ activated (compound [[generic 2], tapSymbol])
          (sequentially
            [ copyOnStack (target (and [abilityHead .anyTriggered, youControl])),
              may (chooseNewTargets (that (.copied .ability))) ]) ] }
