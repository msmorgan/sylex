card := {
  name := "Yedora, Grave Gardener",
  cost := [4, green],
  supertypes := [legendary],
  types := [creature],
  subtypes := [treefolk, druid],
  text := [
    whenever (does (a [isNontoken, [isCreature, youControl], isOther]) die)
      (may
         [
           returnToBattlefield it
             [(faceDownAs { characteristics := some { types := [land], subtypes := [forest] } }), (under (ownerOf it))]
         ])
  ],
  power := 5,
  toughness := 5
}
