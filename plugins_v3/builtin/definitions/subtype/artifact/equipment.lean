/-- [CR#301.5,704.5n] An Equipment may be attached to a creature, and one attached to no legal
host becomes unattached. -/
subtype equipment := {
  lexical := { lemma := "Equipment", plural := "Equipment" },
  spelling := "Equipment",
  kind := .artifact,
  name := "Equipment",
  confers :=
    [.property
        (mayBeAttachedTo thisEquipment
          (allOf isCreature)),
      .stateBased
        (not (isAttached thisEquipment))
        (attachment .unattached thisEquipment none)]
}
