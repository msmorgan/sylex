card := {
  name := "Fblthp, the Lost",
  cost := [1, blue],
  supertypes := [legendary],
  types := [creature],
  subtypes := [homunculus],
  text := [
    when (does this_ enter)
      (replace (draw 1)
         (doIf
            (or
               [
                 did it (enter (from_ := yourLibrary)) triggering,
                 is it (did any (cast which (from_ := yourLibrary)) thisObject)
               ])
            (instead (draw 2)))),
    when (does (a isSpell) (target this_)) (shuffleInto this_)
  ],
  power := 1,
  toughness := 1
}
