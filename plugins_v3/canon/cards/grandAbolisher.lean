/-- Grand Abolisher -/
card grandAbolisher : Spelled :=
  { name := "Grand Abolisher", cost := [pip .white, pip .white], types := [.creature],
    subtypes := [human, cleric],
    text :=
      [ static (partScope .turn you
          (conjunction [ cant yourOpponents (cast (bare isSpell)),
              cant yourOpponents (activate (allOf (and [ abilityHead .anyActivated,
                               abilityOf (allOf (or [isArtifact, isCreature, isEnchantment])) ]))) ])) ],
    power := stat 2, toughness := stat 2 }
