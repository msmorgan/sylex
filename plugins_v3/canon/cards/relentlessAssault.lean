card relentlessAssault : Spelled :=
  { name := "Relentless Assault", cost := [generic 2, pip .red, pip .red],
    types := [.sorcery],
    text :=
      [ spell (sequentially
          [ untap
              (allOf
                 (and
                    [isCreature, did which attack (within soFarThisTurn)])),
            additionalPhase .combat (some .mainPhase) 1 (some .mainPhase) ]) ] }
