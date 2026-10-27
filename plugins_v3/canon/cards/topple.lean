/-- Topple {2}{W} — Sorcery. "Exile target creature with the greatest power among creatures
on the battlefield." -/
card topple : Spelled :=
  { name := "Topple", cost := [generic 2, pip .white], types := [.sorcery],
    text :=
      [ spell (exile (target
          (and [isCreature,
                 superlative .max (.stat .power) (and [isCreature, isPermanent])]))) ] }
