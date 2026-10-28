/-- Donal, Herald of Wings -/
card := {
  name := "Donal, Herald of Wings",
  cost := [2, blue, blue],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, wizard],
  text := [
    whenever
      (does you (cast (a [isCreature, isSpell, not (hasSupertype legendary), hasKeyword flying])))
      (doOnlyOnceEachTurn
         (may
            (copyOnStack it
               (exceptions :=
                  (copyCharacteristics { subtypes := [spirit], power := 1, toughness := 1 } true)))))
  ],
  power := 3,
  toughness := 3
}
