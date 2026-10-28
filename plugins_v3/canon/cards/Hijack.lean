card := {
  name := "Hijack",
  cost := [1, red, red],
  types := [sorcery],
  text := [
    spell
      [
        gainControl (target (or [isArtifact, isCreature])) untilEndOfTurn,
        untap it,
        gainHaste it untilEndOfTurn
      ]
  ]
}
