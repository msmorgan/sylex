card lavaAxe : Spelled :=
  { name := "Lava Axe", cost := [generic 4, pip .red], types := [.sorcery],
    text :=
      [ spell (instruct this_ (dealDamage 5 (target (or [isPlaneswalker, isPlayer])))) ] }
