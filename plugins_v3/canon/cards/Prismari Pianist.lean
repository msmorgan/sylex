/-- Prismari Pianist -/
card := {
  name := "Prismari Pianist",
  cost := [1, red, red],
  types := [creature],
  subtypes := [djinn, bard],
  text := [
    whenever (does you (cast (a [isInstantOrSorcery, isSpell])))
      (replace (create 1 (creatureToken 1 1 [blue, red] [elemental]))
         (doIf (compareAmt (statOf (stat manaValue) (that spell)) atLeast 5)
            (instead (create 3 asThose []))))
  ],
  power := 2,
  toughness := 1
}
