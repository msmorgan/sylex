card urzasFilter : Spelled :=
  { name := "Urza's Filter", cost := [generic 4], types := [.artifact],
    text := [static (spellsCost (and [isMulticolored, isSpell]) (less 2))] }
