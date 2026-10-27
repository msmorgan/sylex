/-- Phyrexian Rebirth -/
card phyrexianRebirth : Spelled :=
  { name := "Phyrexian Rebirth", cost := [generic 4, pip .white, pip .white],
    types := [.sorcery],
    text :=
      [ spell (sequentially
          [ destroy (allOf isCreature),
            create 1 (written { characteristics :=
                  { types := [.artifact, .creature],
                    subtypes := [phyrexian, horror],
                    power := letter .x, toughness := (letter .x) } }),
            define .x
                (countOf (and [isCreature, was (destroy which) thisWay])) ]) ] }
