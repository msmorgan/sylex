card := {
  name := "Archfiend's Vessel",
  cost := [black],
  types := [creature],
  subtypes := [human, cleric],
  text := [
    lifelink,
    when (does thisCreature enter)
      (if_ := some
         (or
            [
              did it (enter (from_ := yourGraveyard)) triggering,
              is it (did you (cast which (from_ := yourGraveyard)) thisObject)
            ]))
      (doIfDone (exile it)
         (some <| create 1
            (written
               {
                 colors := [black],
                 types := [creature],
                 subtypes := [demon],
                 text := [flying],
                 power := 5,
                 toughness := 5
               })))
  ],
  power := 1,
  toughness := 1
}
