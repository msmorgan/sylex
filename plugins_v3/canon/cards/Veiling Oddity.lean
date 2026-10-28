/-- Veiling Oddity -/
card := {
  name := "Veiling Oddity",
  cost := [3, blue],
  types := [creature],
  subtypes := [illusion],
  text := [
    suspend 4 [1, blue],
    (when (does any (removeCounters theLast timeCounter this_))
       (while_ := whileTrue (is this_ (cardIn exileZone)))
       (establishThroughout (cant any (block (allOf isCreature))) thisTurn))
  ],
  power := 2,
  toughness := 3
}
