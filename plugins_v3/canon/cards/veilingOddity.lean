/-- Veiling Oddity -/
card veilingOddity : Spelled :=
  { name := "Veiling Oddity", cost := [generic 3, pip .blue], types := [.creature],
    subtypes := [illusion],
    text := [suspend 4 [generic 1, pip .blue], (when (does any (removeCounters theLast timeCounter
      this_))
          (while_ := whileTrue (is this_ (cardIn exileZone)))
          (establishThroughout (cant any (block (allOf isCreature))) thisTurn))],
    power := stat 2, toughness := stat 3 }
