/-- "For Mirrodin!" means "When this Equipment enters, create a 2/2 red Rebel creature token, then
attach this Equipment to it." [CR#702.163a] -/
keyword forMirrodin : Ability := {
  lexical := { lemma := "for Mirrodin!" },
  spelling := "for Mirrodin!",
  body :=
    [ when (does thisEquipment enter)
        (sequentially
          [ create 1 (creatureToken 2 2 [.red] [rebel]),
            attachTo thisEquipment thatToken ]) ]
}
