card moonlitWakeCard : Spelled :=
  { name := "Moonlit Wake", cost := [generic 2, pip .white], types := [.enchantment],
    text := [(whenever (does (a isCreature) die) (gainLife 1))] }
