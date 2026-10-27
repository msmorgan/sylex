card wrathOfGod : Spelled :=
  { name := "Wrath of God", cost := [generic 2, pip .white, pip .white],
    types := [.sorcery],
    text := [spell (destroyCantRegenerate (allOf isCreature))] }
