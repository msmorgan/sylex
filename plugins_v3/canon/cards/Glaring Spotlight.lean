/-- Pins an as-though exception for targeting by your spells and abilities.
The exception applies only to that act and does not remove hexproof [CR#609.4]. -/
card := {
  name := "Glaring Spotlight",
  cost := [1],
  types := [artifact],
  text := [
    can (by_ (allOf [or [isSpell, abilityHead anyOnStack], youControl]))
      (target (allOf [isCreature, opponentsControl, hasKeyword hexproof]))
      (asThough := holds (is them (not (hasKeyword hexproof)))),
    activated (compound [[3], sacrifice thisArtifact])
      [
        gain (allOf [isCreature, youControl]) hexproof untilEndOfTurn,
        establishThroughout (cant any (block (allOf [isCreature, youControl]))) thisTurn
      ]
  ]
}
