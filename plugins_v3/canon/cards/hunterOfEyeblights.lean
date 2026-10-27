card hunterOfEyeblights : Spelled :=
  { name := "Hunter of Eyeblights", cost := [generic 3, pip .black, pip .black],
    types := [.creature], subtypes := [elf, assassin],
    text :=
      [ when (does thisCreature enter)
          (putCounters 1 p1p1Counter (target (and [isCreature, youDontControl]))),
        activated (compound [[generic 2, pip .black], tapSymbol])
          (destroy (target (and [isCreature, hasCounter]))) ],
    power := stat 3, toughness := stat 3 }
