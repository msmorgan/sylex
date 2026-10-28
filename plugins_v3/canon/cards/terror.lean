card := {
  name := "Terror",
  cost := [1, black],
  types := [instant],
  text := [spell (destroyCantRegenerate (target [isCreature, non isArtifact, non (hasColor black)]))]
}
