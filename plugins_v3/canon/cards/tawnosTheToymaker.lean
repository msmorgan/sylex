/-- Tawnos, the Toymaker -/
card := {
  name := "Tawnos, the Toymaker",
  cost := [3, green, blue],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, artificer],
  text := [
    whenever (does you (cast (a [or [beast, bird], isCreature, isSpell])))
      (may (copyOnStack it (exceptions := [isAlso [artifact] []])))
  ],
  power := 3,
  toughness := 5
}
