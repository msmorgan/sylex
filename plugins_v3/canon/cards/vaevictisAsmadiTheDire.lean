/-- Vaevictis Asmadi, the Dire -/
card := {
  name := "Vaevictis Asmadi, the Dire",
  cost := [3, black, red, green],
  supertypes := [legendary],
  types := [creature],
  subtypes := [elder, dragon],
  text := [
    flying,
    whenever (does thisCreature attack)
      [
        doForEach eachPlayer (choose (target [isPermanent, controlledBy they])),
        instruct (those player) (sacrifice (those permanent)),
        doForEach (each [isPlayer, did which (sacrifice (a isPermanent)) thisWay])
          [
            instruct they (reveal (slice top 1 (libraryOf they))),
            doIf (is it isPermanentCard) (putOntoBattlefield it)
          ]
      ]
  ],
  power := 6,
  toughness := 6
}
