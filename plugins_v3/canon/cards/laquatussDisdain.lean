card laquatussDisdain : Spelled :=
  { name := "Laquatus's Disdain", cost := [generic 1, pip .blue], types := [.instant],
    text :=
      [ spell (sequentially
          [ counter (target (and [isSpell, did any (cast which (from_ := graveyard)) thisObject])),
              draw 1 ]) ] }
