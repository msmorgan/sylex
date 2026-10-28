card := {
  name := "Unesh, Criosphinx Sovereign",
  cost := [4, blue, blue],
  supertypes := [legendary],
  types := [creature],
  subtypes := [sphinx],
  text := [
    flying,
    costShift (allOf [sphinx, isSpell, did you (cast which) thisObject]) (less 2),
    whenever (does (eitherOf thisCreature (a [sphinx, youControl, isOther])) enter)
      [
        reveal (topSlice 4),
        instruct anOpponent (separateIntoPiles them 2),
        putInto onePile hand,
        putInto theOther graveyard
      ]
  ],
  power := 4,
  toughness := 4
}
