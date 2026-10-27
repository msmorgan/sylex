card searingFlesh : Spelled :=
  { name := "Searing Flesh", cost := [generic 6, pip .red], types := [.sorcery],
    text :=
      [ spell (instruct this_
          (dealDamage 7 (target (or [isPlaneswalker, isOpponent])))) ] }
