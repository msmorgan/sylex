card := {
  name := "Denry Klin, Editor in Chief",
  cost := [2, white, blue],
  supertypes := [legendary],
  types := [creature],
  subtypes := [cat, advisor],
  text := [
    enters thisCreature
      [withCounters 1 (chosenKind [p1p1Counter, firstStrikeCounter, vigilanceCounter]) fresh],
    (whenever (does (a [isNontoken, [isCreature, youControl]]) enter)
       (if_ := is thisCreature hasCounter)
       (doForEachKind counterKind (thisCreature) counterKind
          (putCounters (statOf boundCounter thisCreature) thatKind (that creature))))
  ],
  power := 2,
  toughness := 2
}
