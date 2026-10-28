card := {
  name := "Pridemalkin",
  cost := [2, green],
  types := [creature],
  subtypes := [cat],
  text := [
    when (does thisCreature enter) (putCounters 1 p1p1Counter (target [isCreature, youControl])),
    abilityGrant (each [isCreature, youControl, hasCounter p1p1Counter]) trample
  ],
  power := 2,
  toughness := 1
}
