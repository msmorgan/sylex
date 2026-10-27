/-- Disarm {U} — Instant. "Unattach all Equipment from target creature." -/
card disarm : Spelled :=
  { name := "Disarm", cost := [pip .blue], types := [.instant],
    text :=
      [ spell (unattach (allOf
          (and [equipment,
                 attachedTo (target isCreature)]))) ] }
