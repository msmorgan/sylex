keyword forMirrodin : Ability := {
  lexical := { lemma := "for Mirrodin!" },
  spelling := "for Mirrodin!",
  body :=
    [ when (does thisEquipment enter)
        (sequentially
          [ create 1 (creatureToken 2 2 [.red] [rebel]),
            attachTo thisEquipment thatToken ]) ]
}
