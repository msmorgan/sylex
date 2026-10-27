/-- Twinshot Sniper -/
card twinshotSniper : Spelled :=
  { name := "Twinshot Sniper", cost := [generic 3, pip .red], types := [.artifact, .creature],
    subtypes := [goblin, archer],
    text :=
      [ reach,
        when (does thisCreature enter)
            (instruct it (dealDamage 2 (target isAnyTarget))),
        channel
          (activated (compound [[generic 1, pip .red], discard this_])
            (instruct it (dealDamage 2 (target isAnyTarget)))) ],
    power := stat 2, toughness := stat 3 }
