/-- [CR#301.6,704.5n] The Land twin of Equipment. -/
subtype fortification := {
  lexical := { lemma := "Fortification" },
  spelling := "Fortification",
  kind := .artifact,
  name := "Fortification",
  confers :=
    [Semantics.Conferral.property
        (Semantics.Templates.StaticSpec.mayBeAttachedTo Semantics.Templates.NounPhrase.thisFortification
          (Semantics.Templates.NounPhrase.allOf Semantics.Templates.Predicate.isLand)),
      Semantics.Conferral.stateBased
        (Semantics.Templates.Condition.not (Semantics.Templates.Condition.isAttached Semantics.Templates.NounPhrase.thisFortification))
        (Semantics.Templates.Action.attachment Semantics.AttachMove.unattached Semantics.Templates.NounPhrase.thisFortification Option.none)]
}
