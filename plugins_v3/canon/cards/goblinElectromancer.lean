card goblinElectromancer : Spelled :=
  { name := "Goblin Electromancer", cost := [pip .blue, pip .red], types := [.creature],
    subtypes := [goblin, wizard],
    text := [static (spellsCost (and [isInstantOrSorcery, isSpell,
        did you (cast which) thisObject]) (less 1))],
    power := stat 2, toughness := stat 2 }
