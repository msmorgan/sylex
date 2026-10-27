keyword livingWeapon : Ability := {
  lexical := { lemma := "living weapon" },
  spelling := "living weapon",
  body :=
    [ when (does thisEquipment enter)
        (sequentially
          [ create 1 (creatureToken 0 0 [.black] [phyrexian, germ]),
            attachTo thisEquipment thatToken ]) ]
}
