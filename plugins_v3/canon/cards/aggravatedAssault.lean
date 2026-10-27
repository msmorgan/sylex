card aggravatedAssault : Spelled :=
  { name := "Aggravated Assault", cost := [generic 2, pip .red], types := [.enchantment],
    text :=
      [ activatedOnlyDuring [generic 3, pip .red, pip .red]
          (sequentially
            [ untap (allOf (and [isCreature, youControl])),
              additionalPhase .combat (some .mainPhase) 1 (some .mainPhase) ])
          asSorcery ] }
