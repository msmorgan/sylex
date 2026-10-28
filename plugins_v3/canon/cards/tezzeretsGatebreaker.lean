/-- Tezzeret's Gatebreaker -/
card := {
  name := "Tezzeret's Gatebreaker",
  cost := [4],
  types := [artifact],
  text := [
    when (does thisArtifact enter)
      [
        lookAt (topSlice 5),
        may
          [
            reveal (fromAmong (exactly 1) (or [hasColor blue, isArtifact]) them),
            putInto (that card) hand
          ],
        putInto theRest onBottom (riders := [(inOrder randomOrder)])
      ],
    activated (compound [[5, blue], tapSymbol, (sacrifice thisArtifact)])
      (establishThroughout (cant any (block (allOf [isCreature, youControl]))) thisTurn)
  ]
}
