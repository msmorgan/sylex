card := {
  name := "Generator Servant",
  cost := [1, red],
  types := [creature],
  subtypes := [elemental],
  text := [
    activated (compound [tapSymbol, sacrifice thisCreature])
      (addMana 1 (manaOf [[colorless, colorless]])
         [
           whenSpentOn affectsIt false (a [isCreature, isSpell])
             (gainHaste (becomes (that spell)) untilEndOfTurn)
         ])
  ],
  power := 2,
  toughness := 1
}
