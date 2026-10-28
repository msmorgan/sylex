card := {
  name := "Adrix and Nev, Twincasters",
  cost := [2, green, blue],
  supertypes := [legendary],
  types := [creature],
  subtypes := [merfolk, wizard],
  text := [
    ward [2],
    replace
      (would any (createObject anyAmount (token (copyOf (counted (orMore 1) isToken)) []) you))
      (instead (create (times 2 groupSize) asThose []))
  ],
  power := 2,
  toughness := 2
}
