/-- Rings of Brighthearth -/
card ringsOfBrighthearth : Spelled :=
  { name := "Rings of Brighthearth", cost := [generic 3], types := [.artifact],
    text :=
      [ whenever (does you (activate (a (abilityHead .anyActivated))))
          (if_ := some (not (is (that .ability) isManaAbility)))
          (may (pay [generic 2] .once)
            (sequentially
              [ copyOnStack (that .ability),
                may (chooseNewTargets (that (.copied .ability))) ])) ] }
