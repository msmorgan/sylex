/-- "Living weapon" means "When this Equipment enters, create a 0/0 black Phyrexian Germ creature
token, then attach this Equipment to it." [CR#702.92a] -/
keyword livingWeapon : Ability := {
  lexical := { lemma := "living weapon" },
  spelling := "living weapon",
  body :=
    [ when (does thisEquipment enter)
        (sequentially
          [ create 1 (creatureToken 0 0 [.black] [phyrexian, germ]),
            attachTo thisEquipment thatToken ]) ]
}
