/-- "From anywhere" observes Dread after its arrival in the graveyard. -/
card := {
  name := "Dread",
  cost := [3, black, black, black],
  types := [creature],
  subtypes := [elemental, incarnation],
  text := [
    fear,
    whenever (does (a isCreature) (dealDamage (to := you))) (destroy it),
    when (does any (putInto this_ graveyard (from_ := some any)))
      (instruct (ownerOf it) (shuffleInto it))
  ],
  power := 6,
  toughness := 6
}
