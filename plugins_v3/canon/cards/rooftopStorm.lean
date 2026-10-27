/-- Rooftop Storm -/
card rooftopStorm : Spelled :=
  { name := "Rooftop Storm", cost := [generic 5, pip .blue], types := [.enchantment],
    text :=
      [ static (altCost
          (allOf (and [zombie, isCreature, isSpell, did you (cast which) thisObject]))
          [generic 0]) ] }
