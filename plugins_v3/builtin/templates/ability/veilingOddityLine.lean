template veilingOddityLine : Ability := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := when (does any (removeCounters theLast timeCounter this_))
    (while_ := whileTrue (is this_ (cardIn exileZone)))
    (establishThroughout (cant any (block (allOf isCreature))) thisTurn)
}
