/-- "Flash" means "You may play this card any time you could cast an instant." [CR#702.8a] -/
keyword flash : Ability := {
  lexical := { lemma := "flash" },
  spelling := "flash",
  body :=
    [static (can you (play thisCard) (timing := asInstant))]
}
