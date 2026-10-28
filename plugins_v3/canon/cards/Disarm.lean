/-- Disarm {U} — Instant. "Unattach all Equipment from target creature." -/
card := {
  name := "Disarm",
  cost := [blue],
  types := [instant],
  text := [spell (unattach (allOf [equipment, attachedTo (target isCreature)]))]
}
