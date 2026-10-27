/-- [CR#301.5,704.5n] An Equipment may be attached to a creature, and one attached to no legal
host becomes unattached. -/
subtype equipment := {
  lexical := { lemma := "Equipment", plural := "Equipment" },
  spelling := "Equipment",
  kind := .artifact,
  name := "Equipment",
  confers :=
    [Semantics.Conferral.property
        (Semantics.Templates.StaticSpec.mayBeAttachedTo Semantics.Templates.NounPhrase.thisEquipment
          (Semantics.Templates.NounPhrase.allOf Semantics.Templates.Predicate.isCreature)),
      Semantics.Conferral.stateBased
        (Semantics.Templates.Condition.not (Semantics.Templates.Condition.isAttached Semantics.Templates.NounPhrase.thisEquipment))
        (Semantics.Templates.Action.attachment Semantics.AttachMove.unattached Semantics.Templates.NounPhrase.thisEquipment Option.none)]
}
