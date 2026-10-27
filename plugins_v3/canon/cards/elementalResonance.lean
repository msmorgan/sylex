/-- Elemental Resonance -/
card elementalResonance : Spelled :=
  { name := "Elemental Resonance", cost := [generic 2, pip .green, pip .green],
    types := [.enchantment], subtypes := [aura],
    text :=
      [ enchant isPermanent,
        at_ (beginningOfYour .firstMain)
          (addMana 1 (matchingManaCostOf (enchanted permanent))) ] }
