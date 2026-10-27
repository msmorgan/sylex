card lionHeart : Spelled :=
  { name := "Lion Heart", cost := [generic 4], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ when (does thisEquipment enter)
          (instruct it (dealDamage 2 (target isAnyTarget))),
        static (getsPt (equipped creature) (up 2) (up 1)),
        equip [generic 2] ] }
