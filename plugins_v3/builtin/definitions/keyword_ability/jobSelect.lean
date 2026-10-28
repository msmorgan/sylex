/-- "Job select" means "When this Equipment enters, create a 1/1 colorless Hero creature token, then
attach this Equipment to it." [CR#702.182a] -/
keyword jobSelect : Ability := {
  lexical := { lemma := "job select" },
  spelling := "job select",
  body :=
    [ when (does thisEquipment enter)
        (sequentially
          [ create 1 (creatureToken 1 1 [] [hero]),
            attachTo thisEquipment thatToken ]) ]
}
