card tranquilGrove : Spelled :=
  { name := "Tranquil Grove", cost := [generic 1, pip .green], types := [.enchantment],
    text :=
      [ activated [generic 1, pip .green, pip .green]
          (destroy (allOf (and [isEnchantment, isOther]))) ] }
