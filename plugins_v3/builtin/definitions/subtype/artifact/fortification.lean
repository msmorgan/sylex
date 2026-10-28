/-- [CR#301.6,704.5n] The Land twin of Equipment. -/
subtype fortification := {
  lexical := { lemma := "Fortification" },
  spelling := "Fortification",
  kind := .artifact,
  name := "Fortification",
  confers :=
    [Semantics.Conferral.property
        (mayBeAttachedTo thisFortification
          (allOf isLand)),
      Semantics.Conferral.stateBased
        (not (isAttached thisFortification))
        (attachment Semantics.AttachMove.unattached thisFortification none)]
}
