/-- Glassworks // Shattered Yard -/
card glassworksShatteredYard : Spelled := { body := .sharedLineSplit
  { characteristics := { types := [.enchantment], subtypes := [room] } }
  { name := "Glassworks", cost := [generic 2, pip .red],
    text :=
      [ when (does you (unlock thisDoor))
          (instruct thisRoom
             (dealDamage 4 (target (and [isCreature, anOpponentControls])))) ] }
  { name := "Shattered Yard", cost := [generic 4, pip .red],
    text :=
      [ at_ (beginningOfYour .endStep)
          (instruct thisRoom (dealDamage 1 eachOpponent)) ] } }
