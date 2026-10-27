/-- Mana Flare -/
card manaFlare : Spelled :=
  { name := "Mana Flare", cost := [generic 2, pip .red], types := [.enchantment],
    text :=
      [ whenever (does (a isPlayer) (tapForMana (a isLand)))
          (instruct (that player) (addMana 1 (ofTypeProducedBy (that land))))
              ] }
