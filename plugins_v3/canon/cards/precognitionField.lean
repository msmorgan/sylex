card precognitionField : Spelled :=
  { name := "Precognition Field", cost := [generic 3, pip .blue], types := [.enchantment],
    text :=
      [ static (visibility .lookAt you topOfLibrary),
        static (castFromTop (and [isSpell, isInstantOrSorcery])),
        activated [generic 3] (exile (topSlice 1)) ] }
