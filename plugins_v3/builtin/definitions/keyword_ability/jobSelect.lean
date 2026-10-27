keyword jobSelect : Ability := {
  lexical := { lemma := "job select" },
  spelling := "job select",
  body :=
    [ when (does thisEquipment enter)
        (sequentially
          [ create 1 (creatureToken 1 1 [] [hero]),
            attachTo thisEquipment thatToken ]) ]
}
