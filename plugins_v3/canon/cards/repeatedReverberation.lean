/-- Repeated Reverberation -/
card repeatedReverberation : Spelled :=
  { name := "Repeated Reverberation", cost := [generic 2, pip .red, pip .red],
    types := [.instant],
    text :=
      [ spell (delayWithin
          (does you (cast (a (and
            [isInstant, isSpell]))))
          thisTurn
          (sequentially
            [ copyOnStack (that .stack) 2,
              may (chooseNewTargets (those .copy)) ])
          [ does you (cast (a
            (and [isSorcery, isSpell]))),
            does you (activate (a (abilityHead .loyalty))) ]) ] }
